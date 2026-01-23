use hi_core::template::Template;

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
    "#;
    let res: Result<Template, _> = toml::from_str(toml);
    assert!(res.is_err());
}

#[test]
fn test_manifest_alias_package() {
    let toml = r#"
        [package]
        name = "alias-test"
        version = "1.0.0"
        authors = ["Test"]
        description = "Testing package alias"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert_eq!(tpl.manifest.name, "alias-test");
}

#[test]
fn test_manifest_alias_meta() {
    let toml = r#"
        [meta]
        name = "meta-test"
        version = "2.0.0"
        authors = ["Meta Author"]
        description = "Testing meta alias"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert_eq!(tpl.manifest.name, "meta-test");
}

#[test]
fn test_files_array() {
    let toml = r#"
        [manifest]
        name = "files-test"
        version = "0.1.0"
        authors = ["Dev"]
        description = "Testing files array"

        [[files]]
        target = "/path/one"
        content = "content one"

        [[files]]
        target = "/path/two"
        content = "content two"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert_eq!(tpl.files.len(), 2);
    assert_eq!(tpl.files[0].target, "/path/one");
    assert_eq!(tpl.files[1].target, "/path/two");
}

#[test]
fn test_ignored_flag() {
    let toml = r#"
        [manifest]
        name = "ignored-test"
        version = "0.1.0"
        authors = ["Dev"]
        description = "Template with ignored flag"
        ignored = true
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert!(tpl.manifest.ignored);
}

#[test]
fn test_ignored_default_false() {
    let toml = r#"
        [manifest]
        name = "default-ignored"
        version = "0.1.0"
        authors = ["Dev"]
        description = "Without ignored flag"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert!(!tpl.manifest.ignored);
}

#[test]
fn test_optional_fields() {
    let toml = r#"
        [manifest]
        name = "full-manifest"
        version = "1.0.0"
        authors = ["Author One", "Author Two"]
        description = "Full manifest with optional fields"
        repository = "https://github.com/example/repo"
        license = "MIT"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert_eq!(
        tpl.manifest.repository,
        Some("https://github.com/example/repo".to_string())
    );
    assert_eq!(tpl.manifest.license, Some("MIT".to_string()));
}

#[test]
fn test_hooks_default_none() {
    let toml = r#"
        [manifest]
        name = "no-hooks"
        version = "0.1.0"
        authors = ["Dev"]
        description = "Template without hooks"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert!(tpl.hooks.reload.is_none());
}

#[test]
fn test_multiple_targets() {
    let toml = r#"
        [manifest]
        name = "multi-target"
        version = "0.1.0"
        authors = ["Dev"]
        description = "Multiple targets"

        [[targets]]
        target = "~/.config/app/config.toml"
        content = "key = \"{{ value }}\""

        [[targets]]
        target = "~/.config/app/theme.css"
        content = "color: {{ color }};"

        [[targets]]
        target = "~/.local/share/app/data"
        content = "{{ data }}"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert_eq!(tpl.targets.len(), 3);
}

#[test]
fn test_multiple_authors() {
    let toml = r#"
        [manifest]
        name = "multi-author"
        version = "0.1.0"
        authors = ["Alice", "Bob", "Charlie"]
        description = "Multiple authors"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert_eq!(tpl.manifest.authors.len(), 3);
    assert_eq!(tpl.manifest.authors[1], "Bob");
}

#[test]
fn test_empty_targets_default() {
    let toml = r#"
        [manifest]
        name = "empty-targets"
        version = "0.1.0"
        authors = ["Dev"]
        description = "No targets defined"
    "#;

    let tpl: Template = toml::from_str(toml).unwrap();
    assert!(tpl.targets.is_empty());
    assert!(tpl.files.is_empty());
}
