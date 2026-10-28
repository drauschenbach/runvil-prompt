mod install_cancel;
mod progress_recorder;
mod tokio_timer;

use crate::install_cancel::install_cancel;
use crate::progress_recorder::ProgressRecorder;
use crate::tokio_timer::TokioTimer;
use harness::pack::{HostContext, HostServices, PackRegistry};
use harness::{Harness, HostSnapshot, RunRequest};
use harness_gateway_client::{GatewayBroker, GatewayEndpoint, SecretString};
use std::{env, sync::Arc};
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    eprintln!("Starting");
    run().await.expect("Run error");
    eprintln!("Finished");
}

async fn run() -> anyhow::Result<()> {
    // Configure harness
    let gateway_url = env::var("GATEWAY_URL").expect("a GATEWAY_URL is required");
    let gateway_url = format!("{gateway_url}/v1");
    let gateway_secret = env::var("GATEWAY_SECRET").expect("a GATEWAY_SECRET is required");
    let gateway_endpoint = GatewayEndpoint::new(&gateway_url)?;
    let gateway_secret = SecretString::new(gateway_secret)?;
    let gateway_broker = GatewayBroker::new(gateway_endpoint, gateway_secret);
    let mut pack_registry = PackRegistry::new();
    plugin_web::install(&mut pack_registry, json!({}))?;
    let host_services = HostServices::new();
    let host_context = HostContext::new(pack_registry, host_services.clone());
    let harness = Harness::new(
        Arc::new(ProgressRecorder),
        Arc::new(gateway_broker),
        Arc::new(TokioTimer),
        Arc::new(host_context),
        host_services,
    );

    // Configure Host
    let selected_model = Some("Claude Opus 5.5".to_string());
    let workspace_roots = vec![];
    let host = HostSnapshot {
        selected_model,
        workspace_roots,
    };

    // Prepare request
    const SOURCE: &str = include_str!("../prompt.md");
    let req = RunRequest {
        name: "my run request".to_string(),
        source: SOURCE.to_string(),
        args: "".to_string(),
        input_text: None,
        vfs: Default::default(),
        host,
    };

    // Start the run
    let control = harness.control();
    let run = harness.run(req);
    tokio::pin!(run);

    // Run loop
    let mut cancel = install_cancel();
    let mut cancel_requested = false;
    let report = loop {
        tokio::select! {
            result = &mut run => break result,
            Ok(()) = &mut cancel, if !cancel_requested => {
                cancel_requested = true;
                control.cancel();
            }
        }
    };

    // Display report
    eprintln!("{report:?}");

    Ok(())
}
