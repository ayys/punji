//! User-facing app.yaml file config: [`AppConfigV1`].

use std::collections::HashMap;

use bytesize::ByteSize;

/// User-facing app.yaml config file for apps.
///
/// NOTE: only used by the backend, Edge itself does not use this format, and
/// uses [`super::AppVersionV1Spec`] instead.
#[derive(
    serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug, PartialEq, Eq,
)]
pub struct AppConfigV1 {
    /// Name of the app.
    pub name: String,

    /// The package to execute.
    pub package: String,

    /// Environment variables.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub env: HashMap<String, String>,

    /// CLI arguments passed to the runner.
    /// Only applicable for runners that accept CLI arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli_args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AppConfigCapabilityMapV1>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<AppVolume>>,

    /// Enable debug mode, which will show detailed error pages in the web gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
}

#[derive(
    serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug, PartialEq, Eq,
)]
pub struct AppVolume {
    pub name: String,
    pub mounts: Vec<AppVolumeMount>,
}

#[derive(
    serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug, PartialEq, Eq,
)]
pub struct AppVolumeMount {
    /// Path to mount the volume at.
    pub mount_path: String,
    /// Sub-path within the volume to mount.
    pub sub_path: Option<String>,
}

/// Restricted version of [`super::CapabilityMapV1`], with only a select subset
/// of settings.
#[derive(
    serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug, PartialEq, Eq,
)]
pub struct AppConfigCapabilityMapV1 {
    /// Instance memory settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<AppConfigCapabilityMemoryV1>,
}

/// Memory capability settings.
///
/// NOTE: this is kept separate from the [`super::CapabilityMemoryV1`] struct
/// to have separation between the high-level app.yaml and the more internal
/// App entity.
#[derive(
    serde::Serialize, serde::Deserialize, schemars::JsonSchema, Clone, Debug, PartialEq, Eq,
)]
pub struct AppConfigCapabilityMemoryV1 {
    /// Memory limit for an instance.
    ///
    /// Format: [digit][unit], where unit is Mb/Gb/MiB/GiB,...
    #[schemars(with = "Option<String>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<ByteSize>,
}
