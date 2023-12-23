use schemars::JsonSchema;

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum ModuleReference {
    CurrentPackage { module: String },
    Dependency { dependency: String, module: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Command {
    pub name: String,
    #[serde(flatten)]
    pub module: ModuleReference,
    pub runner: String,
}
