//! STUDYVUT desktop app.

mod commands;
mod content;
mod i18n;
mod logging;
mod protocol;
mod state;
mod window;

use studyvut_service::{Paths, Service, ServiceConfig};
use tauri::{App, Manager};
use tracing::{error, info};

use crate::state::AppState;

/// The log file writer, which flushes when dropped.
struct LogGuard(
    #[allow(dead_code)] parking_lot::Mutex<Option<tracing_appender::non_blocking::WorkerGuard>>,
);

pub fn run() {
    // Development instances (a custom data folder) may run next to an
    // installed copy, so they skip the single-instance check.
    let isolated = std::env::var_os("STUDYVUT_DATA_DIR").is_some();
    let mut builder = tauri::Builder::default();
    if !isolated {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            window::show(app);
        }));
    }
    let app = builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .register_uri_scheme_protocol("content", protocol::handle)
        .invoke_handler(tauri::generate_handler![
            commands::get_snapshot,
            commands::get_overview,
            commands::get_chapter,
            commands::start_session,
            commands::answer,
            commands::finish_session,
            commands::get_problems,
            commands::save_settings,
            commands::reload_content,
            commands::reset_progress,
            commands::export_progress,
            commands::open_data_folder,
            commands::open_link,
            commands::open_url,
        ])
        .setup(|app| {
            if let Err(e) = setup(app) {
                error!("startup failed: {e:#}");
                fatal(app.handle(), &format!("{e:#}"));
            }
            Ok(())
        })
        .build(tauri::generate_context!());

    match app {
        Ok(app) => app.run(|_, _| {}),
        Err(e) => {
            eprintln!("STUDYVUT failed to start: {e}");
            std::process::exit(1);
        }
    }
}

/// A start that cannot go on: say why in a message box (the Windows build
/// has no console), then quit.
fn fatal(app: &tauri::AppHandle, error: &str) {
    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
    let tr = i18n::Lang::system().tr();
    let dialog = app
        .dialog()
        .message(tr.cannot_start.replace("{error}", error))
        .kind(MessageDialogKind::Error)
        .title("STUDYVUT");
    // On Linux a blocking dialog on the main thread, which owns GTK, could
    // hang: there it shows once the event loop runs and quits when closed.
    if cfg!(target_os = "linux") {
        dialog.show(|_| std::process::exit(1));
    } else {
        dialog.blocking_show();
        std::process::exit(1);
    }
}

fn setup(app: &mut App) -> anyhow::Result<()> {
    let paths = Paths::resolve(None).ok_or_else(|| anyhow::anyhow!("no local data folder"))?;
    paths.create()?;
    app.manage(LogGuard(parking_lot::Mutex::new(logging::init(
        &paths.logs_dir(),
    ))));
    info!(
        "STUDYVUT {} starting, data in {}",
        app.package_info().version,
        paths.data_dir.display()
    );

    let (files, source) = content::source();
    let webview_dir = paths.webview_dir();
    let service = Service::start(ServiceConfig {
        paths,
        files,
        source,
    })?;
    app.manage(AppState { service });
    window::open(app.handle(), webview_dir)?;
    Ok(())
}
