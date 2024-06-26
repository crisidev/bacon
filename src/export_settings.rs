use {
    crate::*,
    serde::Deserialize,
    std::path::PathBuf,
};

#[derive(Debug, Clone, Deserialize)]
pub struct ExportSettings {
    pub enabled: bool,
    pub path: PathBuf,
    pub line_format: String,
    pub add_context_to_message: bool
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            path: default_path(),
            line_format: default_line_format().to_string(),
            add_context_to_message: false
        }
    }
}

pub fn default_line_format() -> &'static str {
    "{kind} {path}:{line}:{column} {message}"
}

pub fn default_path() -> PathBuf {
    PathBuf::from(".bacon-locations")
}

impl ExportSettings {
    /// Apply one of the configuration element, overriding
    /// defaults and previously applied configuration elements
    pub fn apply_config(
        &mut self,
        config: &ExportConfig,
    ) {
        if let Some(enabled) = config.enabled {
            self.enabled = enabled;
        }
        if let Some(path) = &config.path {
            self.path.clone_from(path);
        }
        if let Some(line_format) = &config.line_format {
            self.line_format.clone_from(line_format);
        }
        if let Some(add_context_to_message) = config.add_context_to_message {
            self.add_context_to_message = add_context_to_message;
        }
    }
}
