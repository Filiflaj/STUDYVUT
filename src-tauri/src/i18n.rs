//! The few texts the Rust side shows itself (dialogs), and the system
//! language for the "System" language setting.

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    Cs,
    En,
}

pub struct Texts {
    pub cannot_start: &'static str,
    pub export_title: &'static str,
    pub export_name: &'static str,
}

impl Lang {
    /// Czech for Czech and Slovak systems, English for the rest.
    pub fn system() -> Self {
        let locale = sys_locale::get_locale()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if locale.starts_with("cs") || locale.starts_with("sk") {
            Lang::Cs
        } else {
            Lang::En
        }
    }

    pub fn tr(self) -> &'static Texts {
        match self {
            Lang::Cs => &Texts {
                cannot_start: "STUDYVUT se nepodařilo spustit:\n\n{error}",
                export_title: "Exportovat pokrok",
                export_name: "studyvut-pokrok.json",
            },
            Lang::En => &Texts {
                cannot_start: "STUDYVUT could not start:\n\n{error}",
                export_title: "Export progress",
                export_name: "studyvut-progress.json",
            },
        }
    }
}
