//! Logs: a daily file in the data folder's `logs/`, and stderr in debug
//! builds. `RUST_LOG` overrides the level.

use std::path::Path;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

/// Start logging; keep the guard until the end, it flushes the file.
pub fn init(dir: &Path) -> Option<WorkerGuard> {
    let filter = || EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let (file, guard) = match std::fs::create_dir_all(dir) {
        Ok(()) => {
            let appender = tracing_appender::rolling::Builder::new()
                .rotation(tracing_appender::rolling::Rotation::DAILY)
                .filename_prefix("studyvut")
                .filename_suffix("log")
                .max_log_files(14)
                .build(dir)
                .ok();
            match appender {
                Some(a) => {
                    let (writer, guard) = tracing_appender::non_blocking(a);
                    (
                        Some(
                            fmt::layer()
                                .with_ansi(false)
                                .with_writer(writer)
                                .with_filter(filter()),
                        ),
                        Some(guard),
                    )
                }
                None => (None, None),
            }
        }
        Err(_) => (None, None),
    };
    let stderr = cfg!(debug_assertions).then(|| {
        fmt::layer()
            .with_writer(std::io::stderr)
            .with_filter(filter())
    });
    let _ = tracing_subscriber::registry()
        .with(file)
        .with(stderr)
        .try_init();
    guard
}
