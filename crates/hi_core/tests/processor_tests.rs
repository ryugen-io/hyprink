use hi_core::config::{
    Config, IconsSection, LayoutSection, LoggingConfig, RetentionConfig, StructureConfig,
    TagConfig, ThemeSection,
};
use hi_core::processor;
use hi_core::template::{Hooks, Template, TemplateManifest};
use std::collections::HashMap;

fn mock_config() -> Config {
    Config {
        theme: ThemeSection {
            name: "test".into(),
            active_icons: "ascii".into(),
            colors: HashMap::new(),
            fonts: HashMap::new(),
        },
        icons: IconsSection {
            nerdfont: HashMap::new(),
            ascii: HashMap::new(),
        },
        layout: LayoutSection {
            tag: TagConfig {
                prefix: "[".into(),
                suffix: "]".into(),
                transform: "none".into(),
                min_width: 0,
                alignment: "left".into(),
            },
            labels: HashMap::new(),
            structure: StructureConfig {
                terminal: "".into(),
                file: "".into(),
            },
            logging: LoggingConfig {
                base_dir: "".into(),
                path_structure: "".into(),
                filename_structure: "".into(),
                timestamp_format: "".into(),
                write_by_default: false,
                app_name: "test".into(),
                retention: RetentionConfig::default(),
            },
        },
    }
}

#[test]
fn test_processor_apply_hook_success() {
    let config = mock_config();

    let tpl = Template {
        manifest: TemplateManifest {
            name: "test_success".to_string(),
            version: "0.1".to_string(),
            authors: vec!["test".to_string()],
            description: "test".to_string(),
            repository: None,
            license: None,
            ignored: false,
        },
        targets: vec![],
        files: vec![],
        hooks: Hooks {
            reload: Some("true".to_string()),
        },
    };

    let result = processor::apply(&tpl, &config, false);
    assert!(result.is_ok());
    assert!(result.unwrap(), "Hook should succeed");
}

#[test]
fn test_processor_apply_hook_failure() {
    let config = mock_config();

    let tpl = Template {
        manifest: TemplateManifest {
            name: "test_fail".to_string(),
            version: "0.1".to_string(),
            authors: vec!["test".to_string()],
            description: "test".to_string(),
            repository: None,
            license: None,
            ignored: false,
        },
        targets: vec![],
        files: vec![],
        hooks: Hooks {
            reload: Some("false".to_string()),
        },
    };

    let result = processor::apply(&tpl, &config, false);
    assert!(
        result.is_ok(),
        "Function should not error on hook failure, but return Ok(false)"
    );
    assert!(!result.unwrap(), "Hook should fail");
}

#[test]
fn test_processor_apply_no_hooks() {
    let config = mock_config();

    let tpl = Template {
        manifest: TemplateManifest {
            name: "no_hooks".to_string(),
            version: "0.1".to_string(),
            authors: vec!["test".to_string()],
            description: "test".to_string(),
            repository: None,
            license: None,
            ignored: false,
        },
        targets: vec![],
        files: vec![],
        hooks: Hooks { reload: None },
    };

    let result = processor::apply(&tpl, &config, false);
    assert!(result.is_ok());
    assert!(result.unwrap(), "Should succeed when no hooks defined");
}
