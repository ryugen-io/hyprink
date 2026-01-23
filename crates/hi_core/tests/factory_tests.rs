use hi_core::factory::{ColorResolver, TagFactory};

mod common;

#[test]
fn test_tag_factory_uppercase_transform() {
    let config = common::create_test_config();
    let tag = TagFactory::create_tag(&config, "debug");
    assert_eq!(tag, "[DEBUG]");
}

#[test]
fn test_tag_factory_lowercase_transform() {
    let mut config = common::create_test_config();
    config.layout.tag.transform = "lowercase".to_string();
    let tag = TagFactory::create_tag(&config, "DEBUG");
    assert_eq!(tag, "[debug]");
}

#[test]
fn test_tag_factory_capitalize_transform() {
    let mut config = common::create_test_config();
    config.layout.tag.transform = "capitalize".to_string();
    let tag = TagFactory::create_tag(&config, "debug");
    assert_eq!(tag, "[Debug]");
}

#[test]
fn test_tag_factory_no_transform() {
    let mut config = common::create_test_config();
    config.layout.tag.transform = "none".to_string();
    let tag = TagFactory::create_tag(&config, "MiXeD");
    assert_eq!(tag, "[MiXeD]");
}

#[test]
fn test_tag_factory_label_lookup() {
    let config = common::create_test_config();
    let tag = TagFactory::create_tag(&config, "info");
    assert_eq!(tag, "[ INF ]");
}

#[test]
fn test_tag_factory_min_width_padding() {
    let mut config = common::create_test_config();
    config.layout.tag.min_width = 10;
    let tag = TagFactory::create_tag(&config, "ok");
    assert_eq!(tag, "[    OK    ]");
}

#[test]
fn test_tag_factory_no_padding_when_exceeds() {
    let mut config = common::create_test_config();
    config.layout.tag.min_width = 3;
    let tag = TagFactory::create_tag(&config, "debug");
    assert_eq!(tag, "[DEBUG]");
}

#[test]
fn test_tag_factory_custom_brackets() {
    let mut config = common::create_test_config();
    config.layout.tag.prefix = "<<".to_string();
    config.layout.tag.suffix = ">>".to_string();
    config.layout.tag.min_width = 0;
    let tag = TagFactory::create_tag(&config, "ok");
    assert_eq!(tag, "<<OK>>");
}

#[test]
fn test_color_resolver_valid_hex_with_hash() {
    let color = ColorResolver::hex_to_color("#FF5500");
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 85);
    assert_eq!(color.b, 0);
}

#[test]
fn test_color_resolver_valid_hex_without_hash() {
    let color = ColorResolver::hex_to_color("00FF00");
    assert_eq!(color.r, 0);
    assert_eq!(color.g, 255);
    assert_eq!(color.b, 0);
}

#[test]
fn test_color_resolver_black() {
    let color = ColorResolver::hex_to_color("#000000");
    assert_eq!(color.r, 0);
    assert_eq!(color.g, 0);
    assert_eq!(color.b, 0);
}

#[test]
fn test_color_resolver_white() {
    let color = ColorResolver::hex_to_color("#FFFFFF");
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 255);
    assert_eq!(color.b, 255);
}

#[test]
fn test_color_resolver_invalid_short() {
    let color = ColorResolver::hex_to_color("#FFF");
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 255);
    assert_eq!(color.b, 255);
}

#[test]
fn test_color_resolver_invalid_empty() {
    let color = ColorResolver::hex_to_color("");
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 255);
    assert_eq!(color.b, 255);
}

#[test]
fn test_color_resolver_lowercase_hex() {
    let color = ColorResolver::hex_to_color("#aabbcc");
    assert_eq!(color.r, 170);
    assert_eq!(color.g, 187);
    assert_eq!(color.b, 204);
}
