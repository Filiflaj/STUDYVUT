//! The main window.

use std::path::PathBuf;

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const LABEL: &str = "main";
const WIDTH: f64 = 1180.0;
const HEIGHT: f64 = 820.0;

/// Open the window, within the screen's work area.
pub fn open(app: &AppHandle, webview_dir: Option<PathBuf>) -> tauri::Result<()> {
    let (w, h) = initial_size(app);
    let mut builder = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("index.html".into()))
        .title("STUDYVUT")
        .inner_size(w, h)
        .min_inner_size(860.0, 560.0)
        .center();
    if let Some(dir) = webview_dir {
        // Development instances keep their own local storage.
        builder = builder.data_directory(dir);
    }
    builder.build()?;
    Ok(())
}

/// Bring the window to the front (a second start of the app).
pub fn show(app: &AppHandle) {
    if let Some(w) = app.get_webview_window(LABEL) {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn initial_size(app: &AppHandle) -> (f64, f64) {
    let Ok(Some(monitor)) = app.primary_monitor() else {
        return (WIDTH, HEIGHT);
    };
    let area = monitor
        .work_area()
        .size
        .to_logical::<f64>(monitor.scale_factor());
    (WIDTH.min(area.width - 40.0), HEIGHT.min(area.height - 40.0))
}
