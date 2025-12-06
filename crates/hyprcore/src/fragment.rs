use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Fragment {
    pub meta: FragmentMeta,
    #[serde(default)]
    pub templates: Vec<Template>,
    #[serde(default)]
    pub files: Vec<Template>, // Same structure as templates
    #[serde(default)]
    pub hooks: Hooks,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fragment_deserialization() {
        let toml = r#"
            [meta]
            id = "test.frag"

            [[templates]]
            target = "~/.config/test"
            content = "Hello {{ name }}"

            [hooks]
            reload = "echo reload"
        "#;

        let pkg: Fragment = toml::from_str(toml).unwrap();
        assert_eq!(pkg.meta.id, "test.frag");
        assert_eq!(pkg.templates.len(), 1);
        assert_eq!(pkg.templates[0].target, "~/.config/test");
        assert_eq!(pkg.hooks.reload.unwrap(), "echo reload");
    }

    #[test]
    fn test_fragment_missing_optional() {
        let toml = r#"
            [meta]
            id = "minimal"
        "#;
        let pkg: Fragment = toml::from_str(toml).unwrap();
        assert_eq!(pkg.meta.id, "minimal");
        assert!(pkg.templates.is_empty());
        assert!(pkg.hooks.reload.is_none());
    }

    #[test]
    fn test_fragment_invalid_toml() {
        let toml = r#"
            [meta]
            id = "broken"
            [[templates]]
            target = 
        "#;
        let res: Result<Fragment, _> = toml::from_str(toml);
        assert!(res.is_err());
    }
}

#[derive(Debug, Deserialize)]
pub struct FragmentMeta {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub target: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Hooks {
    pub reload: Option<String>,
}
