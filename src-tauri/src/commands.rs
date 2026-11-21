//! Tauri commands used by the window.
//!
//! `tz` is the user's time zone offset in minutes east of UTC (the
//! webview's `-new Date().getTimezoneOffset()`), so days are local days.

use std::path::PathBuf;

use serde::Serialize;
use studyvut_core::content::Problem;
use studyvut_core::quiz::{Response, SetSpec, Verdict};
use studyvut_service::views::{
    Activity, ChapterPayload, Overview, ProblemCount, QuestionRow, SessionPayload, SessionResult,
};
use studyvut_service::{ContentSource, ServiceError, Settings};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

use crate::i18n::Lang;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct CmdError {
    /// Machine-readable reason the window can translate (e.g. `empty_set`).
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<&'static str>,
    message: String,
}

impl From<ServiceError> for CmdError {
    fn from(e: ServiceError) -> Self {
        Self {
            code: Some(e.code()),
            message: e.to_string(),
        }
    }
}

impl CmdError {
    fn other(e: impl std::fmt::Display) -> Self {
        Self {
            code: None,
            message: e.to_string(),
        }
    }
}

type CmdResult<T> = Result<T, CmdError>;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct AppInfo {
    version: String,
    data_dir: PathBuf,
    content: ContentSource,
    os: &'static str,
    /// For the "System" language.
    system_language: Lang,
    debug: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Snapshot {
    app: AppInfo,
    settings: Settings,
    overview: Overview,
    first_run: bool,
}

#[tauri::command]
pub fn get_snapshot(app: AppHandle, state: State<'_, AppState>, tz: i32) -> CmdResult<Snapshot> {
    let s = &state.service;
    Ok(Snapshot {
        app: AppInfo {
            version: app.package_info().version.to_string(),
            data_dir: s.paths.data_dir.clone(),
            content: s.source.clone(),
            os: std::env::consts::OS,
            system_language: Lang::system(),
            debug: cfg!(debug_assertions),
        },
        settings: s.settings(),
        overview: s.overview(tz)?,
        first_run: s.first_run,
    })
}

#[tauri::command]
pub fn get_overview(state: State<'_, AppState>, tz: i32) -> CmdResult<Overview> {
    Ok(state.service.overview(tz)?)
}

#[tauri::command]
pub fn get_chapter(
    state: State<'_, AppState>,
    subject: String,
    chapter: String,
) -> CmdResult<ChapterPayload> {
    Ok(state.service.chapter(&subject, &chapter)?)
}

#[tauri::command]
pub fn start_session(
    state: State<'_, AppState>,
    subject: String,
    spec: SetSpec,
    tz: i32,
) -> CmdResult<SessionPayload> {
    Ok(state.service.start_session(&subject, &spec, tz)?)
}

#[tauri::command]
pub fn answer(
    state: State<'_, AppState>,
    session: Option<i64>,
    subject: String,
    question: String,
    response: Response,
    tz: i32,
) -> CmdResult<Verdict> {
    Ok(state
        .service
        .answer(session, &subject, &question, &response, tz)?)
}

#[tauri::command]
pub fn finish_session(state: State<'_, AppState>, session: i64) -> CmdResult<SessionResult> {
    Ok(state.service.finish_session(session)?)
}

#[tauri::command]
pub fn get_activity(state: State<'_, AppState>, days: u32, tz: i32) -> CmdResult<Activity> {
    Ok(state.service.activity(days.min(400), tz)?)
}

#[tauri::command]
pub fn get_questions(state: State<'_, AppState>) -> CmdResult<Vec<QuestionRow>> {
    Ok(state.service.questions()?)
}

#[tauri::command]
pub fn get_problems(state: State<'_, AppState>) -> Vec<Problem> {
    state.service.problems()
}

#[tauri::command]
pub fn save_settings(state: State<'_, AppState>, settings: Settings) -> CmdResult<Settings> {
    Ok(state.service.save_settings(settings)?)
}

#[tauri::command]
pub fn reload_content(state: State<'_, AppState>) -> ProblemCount {
    state.service.reload_content()
}

#[tauri::command]
pub fn reset_progress(state: State<'_, AppState>, subject: Option<String>) -> CmdResult<()> {
    Ok(state.service.reset_progress(subject.as_deref())?)
}

/// Ask where to save, then write every answer as JSON. `None` when cancelled.
#[tauri::command]
pub async fn export_progress(
    app: AppHandle,
    state: State<'_, AppState>,
    lang: Option<String>,
) -> CmdResult<Option<String>> {
    let json = state.service.export_progress()?;
    let tr = match lang.as_deref() {
        Some("en") => Lang::En,
        Some("cs") => Lang::Cs,
        _ => Lang::system(),
    }
    .tr();
    let (tx, rx) = std::sync::mpsc::channel();
    app.dialog()
        .file()
        .set_title(tr.export_title)
        .set_file_name(tr.export_name)
        .add_filter("JSON", &["json"])
        .save_file(move |path| {
            let _ = tx.send(path);
        });
    let Some(path) = rx.recv().ok().flatten() else {
        return Ok(None);
    };
    let path = path.into_path().map_err(CmdError::other)?;
    std::fs::write(&path, json).map_err(CmdError::other)?;
    Ok(Some(path.display().to_string()))
}

#[tauri::command]
pub fn open_data_folder(app: AppHandle, state: State<'_, AppState>) -> CmdResult<()> {
    app.opener()
        .open_path(
            state.service.paths.data_dir.display().to_string(),
            None::<&str>,
        )
        .map_err(CmdError::other)
}

/// Open a page of the project in the system browser.
#[tauri::command]
pub fn open_link(app: AppHandle, target: String) -> CmdResult<()> {
    let url = match target.as_str() {
        "repository" => "https://github.com/Filiflaj/STUDYVUT",
        "releases" => "https://github.com/Filiflaj/STUDYVUT/releases",
        "issues" => "https://github.com/Filiflaj/STUDYVUT/issues",
        _ => return Err(CmdError::other(format!("unknown link {target:?}"))),
    };
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(CmdError::other)
}

/// A link of the study texts in the system browser; web pages only.
#[tauri::command]
pub fn open_url(app: AppHandle, url: String) -> CmdResult<()> {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err(CmdError::other("only web pages can be opened"));
    }
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(CmdError::other)
}
