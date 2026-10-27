// lifted from https://github.com/cppalliance/wg21-paperflow/tree/master/crates/paperweight

use tokio::sync::oneshot;

// Installs a Ctrl-C handler that resolves the returned receiver once.
///
/// A failure to register the signal listener is reported to stderr rather than
/// discarded: the run simply stays non-cancellable in that case.
pub fn install_cancel() -> oneshot::Receiver<()> {
    let (sender, receiver) = oneshot::channel();
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            // The run may already have ended and dropped the receiver.
            Ok(()) => {
                let _ignored = sender.send(());
            }
            Err(error) => {
                eprintln!("warning: cannot listen for Ctrl-C, run is not cancellable: {error}");
            }
        }
    });
    receiver
}
