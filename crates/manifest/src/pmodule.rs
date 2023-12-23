use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[non_exhaustive]
#[derive(Debug, JsonSchema, Deserialize, Serialize)]
pub enum Abi {
    #[serde(rename = "emscripten")]
    Emscripten,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "wasi")]
    Wasi,
    #[serde(rename = "wasm4")]
    WASM4,
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
pub struct WaiBindings {
    /// The version of the WAI format being used.
    pub wai_version: Version,
    /// The `*.wai` file defining the interface this package exposes.
    pub exports: Option<PathBuf>,
    /// The `*.wai` files for any functionality this package imports from the
    /// host.
    pub imports: Vec<PathBuf>,
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WitBindings {
    /// The version of the WIT format being used.
    pub wit_bindgen: Version,
    /// The `*.wit` file's location on disk.
    pub wit_exports: PathBuf,
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
pub enum Bindings {
    Wit(WitBindings),
    Wai(WaiBindings),
}

#[derive(Debug, JsonSchema, Deserialize, Serialize)]
pub struct Module {
    /// The name used to refer to this module.
    pub name: String,
    /// The location of the module file on disk, relative to the manifest
    /// directory.
    pub source: PathBuf,
    /// The ABI this module satisfies.
    pub abi: Abi,
    pub kind: Option<String>,
    /// WebAssembly interfaces this module requires.
    pub interfaces: Option<HashMap<String, String>>,
    /// Interface definitions that can be used to generate bindings to this
    /// module.
    pub bindings: Option<Bindings>,
}
