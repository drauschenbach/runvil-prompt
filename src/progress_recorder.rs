// lifted from https://github.com/cppalliance/wg21-paperflow/tree/master/crates/paperweight

use harness::record::{
    Record, RecordKind, RecorderFuture, RunId, RunMeta, RunOutcome, RunRecorder,
};

/// The run's recorder: prints one progress line to stderr for each event
/// and keeps no history, since the CLI reads the run's result from its
/// report.
#[derive(Debug)]
pub struct ProgressRecorder;

impl RunRecorder for ProgressRecorder {
    fn begin_run(&self, _meta: RunMeta) -> RecorderFuture<'_, RunId> {
        Box::pin(async { Ok(RunId::from_raw(1)) })
    }

    fn append(&self, _run: RunId, record: Record) -> RecorderFuture<'_, ()> {
        if matches!(record.kind, RecordKind::Event) {
            print_progress(&record.payload);
        }
        Box::pin(async { Ok(()) })
    }

    fn end_run(&self, _run: RunId, _outcome: RunOutcome) -> RecorderFuture<'_, ()> {
        Box::pin(async { Ok(()) })
    }
}

/// Writes one progress line for a run event to stderr.
fn print_progress(event: &serde_json::Value) {
    // Progress is a side channel, so a failed write to stderr is
    // deliberately dropped rather than surfaced.
    use std::io::{self, Write as _};
    let _ignored = writeln!(
        io::stderr(),
        "[{}] {}: {}",
        event_field(event, "execution"),
        event_field(event, "section"),
        event_field(event, "kind"),
    );
}

/// Returns the string field `name` of the event, or `-`.
fn event_field<'a>(event: &'a serde_json::Value, name: &str) -> &'a str {
    let Some(value) = event.get(name) else {
        return "-";
    };
    value.as_str().unwrap_or("-")
}
