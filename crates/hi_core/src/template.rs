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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_deserialization() {
        let toml = r#"
            [manifest]
            name = "test.tpl"
            version = "0.0.1"
            authors = ["Tester"]
            description = "A test template"

            [[targets]]
            target = "~/.config/test"
            content = "Hello {{ name }}"

            [hooks]
            reload = "echo reload"
        "#;

        let tpl: Template = toml::from_str(toml).unwrap();
        assert_eq!(tpl.manifest.name, "test.tpl");
        assert_eq!(tpl.manifest.version, "0.0.1");
        assert_eq!(tpl.targets.len(), 1);
        assert_eq!(tpl.targets[0].target, "~/.config/test");
        assert_eq!(tpl.hooks.reload.unwrap(), "echo reload");
    }

    #[test]
    fn test_template_missing_required_fields() {
        let toml = r#"
            [manifest]
            name = "minimal"
            # Missing version, authors, description
        "#;
        let res: Result<Template, _> = toml::from_str(toml);
        assert!(res.is_err());
    }
}
