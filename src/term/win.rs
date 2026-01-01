use std::{env, fs::File, io::Write, path::PathBuf};

use color_eyre::eyre::{Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    colors::unified::{AnsiIndex, AnsiPaletteHex},
    utils::ERR_MSG,
};
const SCHEME_NAME: &str = "yaat";

/// return `stable`, `preview`, `unpkg`
fn get_winterm_dirs() -> Option<(PathBuf, PathBuf, PathBuf)> {
    let local_appdata = env::var("LOCALAPPDATA").ok();

    if let Some(local_appdata) = local_appdata {
        let local_appdata = PathBuf::from(local_appdata);
        return Some((
            local_appdata
                .join("Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json"),
            local_appdata.join(
                "Packages/Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe/LocalState/settings.json",
            ),
            local_appdata.join("Microsoft/WindowsTerminal/settings.json"),
        ));
    }

    None
}

pub fn win_term(ansi: &AnsiPaletteHex) -> Result<()> {
    let Some((stable, preview, unpkg)) = get_winterm_dirs() else {
        bail!("couldn't get %LOCALAPPDATA%")
    };

    for i in [stable, preview, unpkg] {
        let context = match std::fs::read_to_string(&i) {
            Ok(o) => o,
            Err(e) => continue,
        };

        let mut setttings_json = match serde_json::from_str::<WinTerm>(&context) {
            Ok(o) => o,
            Err(e) => {
                eprintln!(
                    "{w}: deserializing json failed {p}: {e}",
                    p = i.display(),
                    w = ERR_MSG
                );
                continue;
            }
        };

        let mut found = false;

        for (i, s) in setttings_json.schemes.iter().enumerate() {
            if s.name == SCHEME_NAME {
                setttings_json.schemes[i] = ansi.into();
                found = true;
                break;
            }
        }

        if found == false {
            setttings_json.schemes.push(ansi.into());
        }

        let new_json = match serde_json::to_string_pretty(&setttings_json) {
            Ok(o) => o,
            Err(e) => {
                eprintln!(
                    "{}: writing json failed: {p}: {e}",
                    ERR_MSG,
                    p = i.display()
                );
                continue;
            }
        };

        File::create(&i)?.write_all(new_json.as_bytes())?
    }

    Ok(())
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WinTerm {
    #[serde(rename = "$help")]
    pub help: String,
    #[serde(rename = "$schema")]
    pub schema: String,
    pub actions: Value,
    pub copy_formatting: String,
    pub copy_on_select: bool,
    pub default_profile: String,
    pub new_tab_menu: Value,
    pub profiles: Value,
    pub themes: Value,
    pub schemes: Vec<WinScheme>,
}

/// a WindowsTerminal Scheme
#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WinScheme {
    pub name: String,
    pub cursor_color: String,
    pub selection_background: String,
    pub background: String,
    pub foreground: String,
    pub black: String,
    pub blue: String,
    pub cyan: String,
    pub green: String,
    pub purple: String,
    pub red: String,
    pub white: String,
    pub yellow: String,
    pub bright_black: String,
    pub bright_blue: String,
    pub bright_cyan: String,
    pub bright_green: String,
    pub bright_purple: String,
    pub bright_red: String,
    pub bright_white: String,
    pub bright_yellow: String,
}

impl From<&AnsiPaletteHex> for WinScheme {
    fn from(c: &AnsiPaletteHex) -> Self {
        Self {
            name: SCHEME_NAME.to_string(),
            cursor_color: "#FFFFFF".to_string(),
            selection_background: c.bright.get_ro(AnsiIndex::White as usize).to_string(),
            foreground: "#D3D7CF".to_string(),
            background: "#000000".to_string(),
            black: c.normal.get_ro(AnsiIndex::Black as usize).to_string(),
            blue: c.normal.get_ro(AnsiIndex::Blue as usize).to_string(),
            cyan: c.normal.get_ro(AnsiIndex::Cyan as usize).to_string(),
            green: c.normal.get_ro(AnsiIndex::Green as usize).to_string(),
            purple: c.normal.get_ro(AnsiIndex::Magenta as usize).to_string(),
            red: c.normal.get_ro(AnsiIndex::Red as usize).to_string(),
            white: c.normal.get_ro(AnsiIndex::White as usize).to_string(),
            yellow: c.normal.get_ro(AnsiIndex::Yellow as usize).to_string(),
            bright_black: c.bright.get_ro(AnsiIndex::Black as usize).to_string(),
            bright_blue: c.bright.get_ro(AnsiIndex::Blue as usize).to_string(),
            bright_cyan: c.bright.get_ro(AnsiIndex::Cyan as usize).to_string(),
            bright_green: c.bright.get_ro(AnsiIndex::Green as usize).to_string(),
            bright_purple: c.bright.get_ro(AnsiIndex::Magenta as usize).to_string(),
            bright_red: c.bright.get_ro(AnsiIndex::Red as usize).to_string(),
            bright_white: c.bright.get_ro(AnsiIndex::White as usize).to_string(),
            bright_yellow: c.bright.get_ro(AnsiIndex::Yellow as usize).to_string(),
        }
    }
}
