mod app_config;
mod command;
pub mod convert;
mod package;
mod pmodule;

use anyhow::Context;
use app_config::AppConfigV1;
use command::Command;
use indexmap::IndexMap;
use package::PackageMetadata;
use pmodule::Module;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum PackageManifestKind {
    #[serde(rename = "punji/package.v0")]
    PackageV0,
}

impl Default for PackageManifestKind {
    fn default() -> Self {
        Self::PackageV0
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum ProjectKind {
    #[serde(rename = "punji/project.v0")]
    ProjectV0,
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PackageManifest {
    /// Metadata about the package itself.
    #[serde(default, skip_serializing)]
    pub kind: PackageManifestKind,
    /// Metadata about the package itself.
    #[serde(rename(serialize = "package"))]
    pub metadata: PackageMetadata,
    pub dependencies: Option<HashMap<String, String>>,
    pub fs: Option<IndexMap<String, PathBuf>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename(serialize = "module")
    )]
    pub modules: Option<Vec<Module>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename(serialize = "command")
    )]
    pub commands: Option<Vec<Command>>,
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum Project {
    #[serde(rename(deserialize = "punji/package.v0"))]
    Package(Box<PackageManifest>),
    #[serde(rename(deserialize = "punji/app.v0"))]
    App(AppConfigV1),
}

impl Project {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(input)?)
    }

    pub fn parse_many(input: &str) -> anyhow::Result<Vec<Self>> {
        let deserializer = serde_yaml::Deserializer::from_str(input);
        deserializer
            .into_iter()
            .map(|document| {
                serde_yaml::from_value(
                    serde_yaml::Value::deserialize(document)
                        .with_context(|| "Invalid yaml file")?,
                )
                .with_context(|| "Invalid punji manifest")
            })
            .collect()
    }
}
