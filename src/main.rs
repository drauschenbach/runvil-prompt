mod progress_recorder;
mod tokio_timer;

use crate::progress_recorder::ProgressRecorder;
use crate::tokio_timer::TokioTimer;
use harness::Harness;
use harness::pack::{HostContext, HostServices, PackRegistry};
use harness_gateway_client::{GatewayBroker, GatewayEndpoint, SecretString};
use std::env;
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    eprintln!("Starting");
    run().await.expect("Run error");
}

async fn run() -> anyhow::Result<()> {
    let gateway_url = env::var("GATEWAY_URL").expect("a GATEWAY_URL is required");
    let gateway_secret = env::var("GATEWAY_SECRET").expect("a GATEWAY_SECRET is required");
    let gateway_endpoint = GatewayEndpoint::new(&gateway_url)?;
    let gateway_secret = SecretString::new(gateway_secret)?;
    let gateway_broker = GatewayBroker::new(gateway_endpoint, gateway_secret);
    let pack_registry = PackRegistry::new();
    let host_services = HostServices::new();
    let host_context = HostContext::new(pack_registry, host_services.clone());
    let _harness = Harness::new(
        Arc::new(ProgressRecorder),
        Arc::new(gateway_broker),
        Arc::new(TokioTimer),
        Arc::new(host_context),
        host_services,
    );
    Ok(())
}
