//! Minimal local configuration. No network calls, tokens or telemetry.
use serde::{Deserialize, Serialize};
use std::{env, fs, io, path::PathBuf};

#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub quiet_mode: bool,
    pub compact_mode: bool,
}

impl Settings {
    pub fn load() -> Result<Self, String> {
        let path = settings_path()?;
        match fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str(&data)
                .map_err(|error| format!("Settings could not be parsed: {error}")),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(format!("Settings could not be read: {error}")),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = settings_path()?;
        let parent = path.parent().ok_or("Invalid settings path")?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("Cannot create settings directory: {error}"))?;
        let contents = serde_json::to_string_pretty(self)
            .map_err(|error| format!("Cannot serialize settings: {error}"))?;
        fs::write(path, contents)
            .map_err(|error| format!("Cannot write settings: {error}"))
    }
}

fn settings_path() -> Result<PathBuf, String> {
    env::var_os("APPDATA")
        .map(PathBuf::from)
        .map(|base| base.join("FlowRail").join("settings.json"))
        .ok_or_else(|| "APPDATA is not configured on this computer".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_fields_default_to_false() {
        let parsed: Settings = serde_json::from_str(r#"{"quiet_mode":true}"#).unwrap();
        assert!(parsed.quiet_mode);
        assert!(!parsed.compact_mode);
    }
}
