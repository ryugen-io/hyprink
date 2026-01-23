use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    #[serde(alias = "package", alias = "meta")]
    pub manifest: TemplateManifest,
    #[serde(default)]
    pub targets: Vec<Target>,
    #[serde(default)]
    pub files: Vec<Target>,
    #[serde(default)]
    pub hooks: Hooks,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateManifest {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: String,
    pub repository: Option<String>,
    pub license: Option<String>,
    #[serde(default)]
    pub ignored: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Target {
    pub target: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Hooks {
    pub reload: Option<String>,
}
