use semver::Version;
use std::path::PathBuf;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PackageMetadata {
    /// The package's name in the form `namespace/name`.
    pub name: String,
    /// The package's version number.
    pub version: Version,
    /// A brief description of the package.
    pub description: String,
    /// A SPDX license specifier for this package.
    pub license: Option<String>,
    /// The location of the license file, useful for non-standard licenses
    #[serde(rename = "license-file")]
    pub license_file: Option<PathBuf>,
    /// The package's README file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<PathBuf>,
    /// A URL pointing to the package's source code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// The website used as the package's homepage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// The name of the command that should be used by `wasmer run` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    /// Mark this as a private package
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub private: bool,
}
