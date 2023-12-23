use std::path::Path;

use crate::{app_config::AppConfigV1, PackageManifest};

pub trait ConvertToFile {
    // Property to store the filename
    fn filename(&self) -> &Path;

    // Method to convert to the specified file type
    fn convert(&self) -> anyhow::Result<String>;
}

impl ConvertToFile for &PackageManifest {
    fn filename(&self) -> &Path {
        Path::new("wasmer.toml")
    }

    fn convert(&self) -> anyhow::Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }
}

impl ConvertToFile for AppConfigV1 {
    fn filename(&self) -> &Path {
        Path::new("app.yaml")
    }

    fn convert(&self) -> anyhow::Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }
}
