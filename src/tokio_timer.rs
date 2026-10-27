// lifted from https://github.com/cppalliance/wg21-paperflow/tree/master/crates/paperweight

use harness::{BoxFuture, Timer};
use std::time::Duration;

/// Sleeps on the CLI runtime's tokio timer.
#[derive(Debug)]
pub struct TokioTimer;

impl Timer for TokioTimer {
    fn sleep(&self, duration: Duration) -> BoxFuture<()> {
        Box::pin(tokio::time::sleep(duration))
    }
}
