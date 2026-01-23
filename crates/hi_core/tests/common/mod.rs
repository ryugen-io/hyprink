// Each integration test is compiled as a separate binary.
// Functions not used by a specific test appear "dead" from its perspective.
#![allow(dead_code)]

use hi_core::config::{
    Config, IconsSection, LayoutSection, LoggingConfig, RetentionConfig, StructureConfig,
    TagConfig, ThemeSection,
};
use hi_core::template::{Hooks, Target, Template, TemplateManifest};
use std::collections::HashMap;

pub fn create_test_config() -> Config {
    Config {
        theme: ThemeSection {
            name: "test".to_string(),
            active_icons: "nerdfont".to_string(),
            colors: HashMap::new(),
            fonts: HashMap::new(),
        },
        icons: IconsSection {
            nerdfont: HashMap::new(),
            ascii: HashMap::new(),
        },
        layout: LayoutSection {
            tag: TagConfig {
                prefix: "[".to_string(),
                suffix: "]".to_string(),
                transform: "uppercase".to_string(),
                min_width: 5,
                alignment: "center".to_string(),
            },
            labels: HashMap::from([
                ("info".to_string(), "INF".to_string()),
                ("error".to_string(), "ERR".to_string()),
            ]),
            structure: StructureConfig {
                terminal: "{tag} {msg}".to_string(),
                file: "{msg}".to_string(),
            },
            logging: LoggingConfig {
                base_dir: "/tmp".to_string(),
                path_structure: "{app}".to_string(),
                filename_structure: "{level}.log".to_string(),
                timestamp_format: "%H:%M:%S".to_string(),
                write_by_default: false,
                app_name: "test".to_string(),
                retention: RetentionConfig::default(),
            },
        },
    }
}

pub fn create_test_template(name: &str) -> Template {
    Template {
        manifest: TemplateManifest {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            authors: vec!["Test".to_string()],
            description: "Test template".to_string(),
            repository: None,
            license: None,
            ignored: false,
        },
        targets: vec![Target {
            target: "~/.config/test".to_string(),
            content: "test content".to_string(),
        }],
        files: vec![],
        hooks: Hooks::default(),
    }
}
