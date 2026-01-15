use crate::config::Config;
use colored::CustomColor;

pub struct TagFactory;

impl TagFactory {
    pub fn create_tag(config: &Config, level: &str) -> String {
        // 1. Lookup Label
        let label = config
            .layout
            .labels
            .get(level)
            .map(|s| s.as_str())
            .unwrap_or(level);

        // 2. Transform
        let transformed = match config.layout.tag.transform.as_str() {
            "uppercase" => label.to_uppercase(),
            "lowercase" => label.to_lowercase(),
            "capitalize" => {
                let mut c = label.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            }
            _ => label.to_string(),
        };

        // 3. Pad
        let width = config.layout.tag.min_width;
        let len = transformed.chars().count();
        let padded = if len >= width {
            transformed
        } else {
            let total_padding = width - len;
            let left_pad = total_padding / 2;
            let right_pad = total_padding - left_pad;
            format!(
                "{}{}{}",
                " ".repeat(left_pad),
                transformed,
                " ".repeat(right_pad)
            )
        };

        // 4. Bracket
        format!(
            "{}{}{}",
            config.layout.tag.prefix, padded, config.layout.tag.suffix
        )
    }
}

pub struct ColorResolver;

impl ColorResolver {
    pub fn hex_to_color(hex: &str) -> CustomColor {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
            CustomColor { r, g, b }
        } else {
            CustomColor {
                r: 255,
                g: 255,
                b: 255,
            }
        }
    }
}
