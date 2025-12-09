use kitchn_lib::config::{Cookbook, ThemeConfig, ThemeMeta, ThemeSettings, IconsConfig, LayoutConfig, DictionaryConfig, TagConfig, StructureConfig, LoggingConfig};
use kitchn_lib::ingredient::{Ingredient, IngredientMeta, IngredientHooks};
use kitchn_lib::processor;
use std::collections::HashMap;

fn mock_cookbook() -> Cookbook {
    Cookbook {
        theme: ThemeConfig {
            meta: ThemeMeta { name: "test".into() },
            settings: ThemeSettings { active_icons: "ascii".into() },
            colors: HashMap::new(),
            fonts: HashMap::new(),
            include: None,
        },
        icons: IconsConfig {
            nerdfont: HashMap::new(),
            ascii: HashMap::new(),
            include: None,
        },
        layout: LayoutConfig {
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
            },
            include: None,
        },
        dictionary: DictionaryConfig {
             presets: HashMap::new(),
             include: None,
        },
    }
}

#[test]
fn test_processor_apply_hook_success() {
    let config = mock_cookbook();
    // Disable actual logging for test cleanliness if possible, or ignore it
    
    let mut pkg = Ingredient {
        meta: IngredientMeta {
            name: "test_success".to_string(),
            version: "0.1".to_string(),
            description: "test".to_string(), // added field
            authors: vec!["test".to_string()], // added field
            ..Default::default()
        },
        templates: vec![],
        files: vec![],
        hooks: IngredientHooks {
            reload: Some("true".to_string()), // 'true' command always succeeds
            ..Default::default()
        },
    };

    let result = processor::apply(&pkg, &config);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true, "Hook should succeed");
}

#[test]
fn test_processor_apply_hook_failure() {
    let config = mock_cookbook();
    
    let pkg = Ingredient {
        meta: IngredientMeta {
            name: "test_fail".to_string(),
            version: "0.1".to_string(),
            description: "test".to_string(),
            authors: vec!["test".to_string()],
            ..Default::default()
        },
        templates: vec![],
        files: vec![],
        hooks: IngredientHooks {
            reload: Some("false".to_string()), // 'false' command always fails
            ..Default::default()
        },
    };

    let result = processor::apply(&pkg, &config);
    assert!(result.is_ok(), "Function should not error on hook failure, but return Ok(false)");
    assert_eq!(result.unwrap(), false, "Hook should fail");
}
