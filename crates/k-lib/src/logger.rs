use crate::config::Cookbook;
use crate::factory::{ColorResolver, TagFactory};
use anyhow::{Context, Result};
use chrono::Local;
use colored::Colorize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn log_to_terminal(config: &Cookbook, level: &str, scope: &str, msg: &str) {
    let icon_set_key = &config.theme.settings.active_icons;
    let icon = if icon_set_key == "nerdfont" {
        config
            .icons
            .nerdfont
            .get(level)
            .map(|s| s.as_str())
            .unwrap_or("?")
    } else {
        config
            .icons
            .ascii
            .get(level)
            .map(|s| s.as_str())
            .unwrap_or("?")
    };

    let tag = TagFactory::create_tag(config, level);

    let level_color_hex = config
        .theme
        .colors
        .get(level)
        .or_else(|| config.theme.colors.get("fg"))
        .map(|s| s.as_str())
        .unwrap_or("#ffffff");
    let level_color = ColorResolver::hex_to_color(level_color_hex);

    let structure = &config.layout.structure.terminal;

    let parts = parse_structure(structure);

    for part in parts {
        match part.as_str() {
            "{tag}" => print!("{}", tag.custom_color(level_color)),
            "{icon}" => print!("{}", icon.custom_color(level_color)),
            "{scope}" => print!("{}", scope.white().dimmed()),
            "{msg}" => print_formatted_msg(msg, config),
            _ => print!("{}", part),
        }
    }
    println!();
}

fn parse_structure(structure: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let placeholders = vec!["{tag}", "{scope}", "{icon}", "{msg}"];

    let mut i = 0;
    while i < structure.len() {
        let remainder = &structure[i..];
        let mut matched = false;

        for ph in &placeholders {
            if remainder.starts_with(ph) {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
                parts.push(ph.to_string());
                i += ph.len();
                matched = true;
                break;
            }
        }

        if !matched {
            current.push(structure.chars().nth(i).unwrap());
            i += 1;
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

fn print_formatted_msg(msg: &str, config: &Cookbook) {
    let mut i = 0;
    while i < msg.len() {
        if let Some(start_tag_open) = msg[i..].find('<') {
            print!("{}", &msg[i..i + start_tag_open]);
            i += start_tag_open;

            if let Some(tag_close_idx) = msg[i..].find('>') {
                let tag_name = &msg[i + 1..i + tag_close_idx];
                let content_start = i + tag_close_idx + 1;
                let close_tag = format!("</{}>", tag_name);

                if let Some(content_end_rel) = msg[content_start..].find(&close_tag) {
                    let content_end = content_start + content_end_rel;
                    let inner_text = &msg[content_start..content_end];
                    apply_style(inner_text, tag_name, config);
                    i = content_end + close_tag.len();
                } else {
                    print!("<");
                    i += 1;
                }
            } else {
                print!("<");
                i += 1;
            }
        } else {
            print!("{}", &msg[i..]);
            break;
        }
    }
}

fn apply_style(text: &str, style: &str, config: &Cookbook) {
    if style == "bold" {
        print!("{}", text.bold());
    } else if let Some(hex) = config.theme.colors.get(style) {
        let color = ColorResolver::hex_to_color(hex);
        print!("{}", text.custom_color(color));
    } else {
        print!("{}", text);
    }
}

pub fn log_to_file(
    config: &Cookbook,
    level: &str,
    scope: &str,
    msg: &str,
    app_override: Option<&str>,
) -> Result<()> {
    let clean_msg = strip_tags(msg);
    let now = Local::now();
    let tag = TagFactory::create_tag(config, level);
    let timestamp = now
        .format(&config.layout.logging.timestamp_format)
        .to_string();

    let app_name = app_override.unwrap_or(&config.layout.logging.app_name);

    let mut content = config.layout.structure.file.clone();
    content = content.replace("{timestamp}", &timestamp);
    content = content.replace("{tag}", &tag);
    content = content.replace("{msg}", &clean_msg);
    content = content.replace("{scope}", scope);

    let base_dir_str = &config.layout.logging.base_dir;
    let base_dir = if base_dir_str.starts_with("~") {
        let home = directories::UserDirs::new().context("Could not find home dir")?;
        PathBuf::from(base_dir_str.replace("~", home.home_dir().to_str().unwrap()))
    } else {
        PathBuf::from(base_dir_str)
    };

    let year = now.format("%Y").to_string();
    let month = now.format("%m").to_string();
    let day = now.format("%d").to_string();

    let mut rel_path = config.layout.logging.path_structure.clone();
    rel_path = rel_path.replace("{year}", &year);
    rel_path = rel_path.replace("{month}", &month);
    rel_path = rel_path.replace("{scope}", scope);
    rel_path = rel_path.replace("{app}", app_name);

    let mut filename = config.layout.logging.filename_structure.clone();
    filename = filename.replace("{level}", level);
    filename = filename.replace("{year}", &year);
    filename = filename.replace("{month}", &month);
    filename = filename.replace("{day}", &day);
    filename = filename.replace("{app}", app_name);
    filename = filename.replace("{scope}", scope);

    let full_dir = base_dir.join(rel_path);
    fs::create_dir_all(&full_dir)?;

    let file_path = full_dir.join(filename);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", content)?;

    Ok(())
}

fn strip_tags(msg: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < msg.len() {
        if let Some(start) = msg[i..].find('<') {
            result.push_str(&msg[i..i + start]);
            i += start;
            if let Some(end) = msg[i..].find('>') {
                i += end + 1;
            } else {
                result.push('<');
                i += 1;
            }
        } else {
            result.push_str(&msg[i..]);
            break;
        }
    }
    result
}

// ============================================================================
// Log Cleanup & Stats
// ============================================================================

/// Options for log cleanup operations
#[derive(Debug, Clone, Default)]
pub struct CleanupOptions {
    /// Override max age in days (None = use config default)
    pub max_age_days: Option<u32>,
    /// Override max total size (None = use config default)
    pub max_total_size: Option<String>,
    /// Delete ALL files regardless of age/size
    pub delete_all: bool,
    /// Dry run - don't actually delete, just report
    pub dry_run: bool,
}

/// Statistics about log files
#[derive(Debug, Default)]
pub struct LogStats {
    pub total_files: usize,
    pub total_size: u64,
    pub oldest_file: Option<String>,
    pub newest_file: Option<String>,
    pub files_by_age: Vec<(String, u64, u64)>, // (path, size, age_days)
}

/// Clean up old log files based on retention policy
pub fn cleanup(config: &Cookbook, options: CleanupOptions) -> Result<CleanupResult> {
    let base_dir = resolve_base_dir(&config.layout.logging.base_dir)?;
    let retention = &config.layout.logging.retention;

    let max_age = options.max_age_days.unwrap_or(retention.max_age_days);
    let max_size = options
        .max_total_size
        .as_ref()
        .or(retention.max_total_size.as_ref())
        .and_then(|s| parse_size(s));

    let mut result = CleanupResult::default();
    let now = std::time::SystemTime::now();

    // Collect all log files
    let mut files: Vec<(PathBuf, u64, u64)> = Vec::new(); // (path, size, age_days)

    if base_dir.exists() {
        collect_log_files(&base_dir, &mut files, now)?;
    }

    // Sort by age (oldest first)
    files.sort_by_key(|(_, _, age)| std::cmp::Reverse(*age));

    // Delete all files if requested, otherwise by age
    for (path, size, age) in &files {
        if options.delete_all || *age > max_age as u64 {
            if options.dry_run {
                result.would_delete.push(path.display().to_string());
                result.would_free += size;
            } else if fs::remove_file(path).is_ok() {
                result.deleted.push(path.display().to_string());
                result.freed += size;
            }
        }
    }

    // If max_size is set, delete oldest files until under limit
    if let Some(limit) = max_size {
        let remaining: Vec<_> = files
            .iter()
            .filter(|(p, _, _)| !result.deleted.contains(&p.display().to_string()))
            .collect();

        let mut total: u64 = remaining.iter().map(|(_, s, _)| s).sum();

        for (path, size, _) in remaining.iter().rev() {
            if total <= limit {
                break;
            }
            if options.dry_run {
                result.would_delete.push(path.display().to_string());
                result.would_free += size;
            } else if fs::remove_file(path).is_ok() {
                result.deleted.push(path.display().to_string());
                result.freed += size;
                total -= size;
            }
        }
    }

    // Clean up empty directories
    if !options.dry_run {
        cleanup_empty_dirs(&base_dir)?;
    }

    Ok(result)
}

/// Result of a cleanup operation
#[derive(Debug, Default)]
pub struct CleanupResult {
    pub deleted: Vec<String>,
    pub freed: u64,
    pub would_delete: Vec<String>, // for dry-run
    pub would_free: u64,           // for dry-run
}

/// Get statistics about log files
pub fn stats(config: &Cookbook) -> Result<LogStats> {
    let base_dir = resolve_base_dir(&config.layout.logging.base_dir)?;
    let mut stats = LogStats::default();
    let now = std::time::SystemTime::now();

    if !base_dir.exists() {
        return Ok(stats);
    }

    let mut files: Vec<(PathBuf, u64, u64)> = Vec::new();
    collect_log_files(&base_dir, &mut files, now)?;

    stats.total_files = files.len();
    stats.total_size = files.iter().map(|(_, s, _)| s).sum();

    if let Some((path, _, _)) = files.iter().max_by_key(|(_, _, age)| age) {
        stats.oldest_file = Some(path.display().to_string());
    }
    if let Some((path, _, _)) = files.iter().min_by_key(|(_, _, age)| age) {
        stats.newest_file = Some(path.display().to_string());
    }

    stats.files_by_age = files
        .into_iter()
        .map(|(p, s, a)| (p.display().to_string(), s, a))
        .collect();

    Ok(stats)
}

fn resolve_base_dir(base_dir_str: &str) -> Result<PathBuf> {
    if base_dir_str.starts_with('~') {
        let home = directories::UserDirs::new().context("Could not find home dir")?;
        Ok(PathBuf::from(
            base_dir_str.replace('~', home.home_dir().to_str().unwrap()),
        ))
    } else {
        Ok(PathBuf::from(base_dir_str))
    }
}

fn collect_log_files(
    dir: &Path,
    files: &mut Vec<(PathBuf, u64, u64)>,
    now: std::time::SystemTime,
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_log_files(&path, files, now)?;
        } else if path.extension().is_some_and(|e| e == "log") {
            let meta = fs::metadata(&path)?;
            let size = meta.len();
            let age_days = meta
                .modified()
                .ok()
                .and_then(|m| now.duration_since(m).ok())
                .map(|d| d.as_secs() / 86400)
                .unwrap_or(0);

            files.push((path, size, age_days));
        }
    }

    Ok(())
}

fn cleanup_empty_dirs(dir: &Path) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            cleanup_empty_dirs(&path)?;
            // Try to remove if empty (will fail silently if not empty)
            let _ = fs::remove_dir(&path);
        }
    }

    Ok(())
}

fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();
    let (num_str, multiplier): (&str, f64) = if s.ends_with("G") || s.ends_with("GB") {
        (s.trim_end_matches("GB").trim_end_matches('G'), 1024.0 * 1024.0 * 1024.0)
    } else if s.ends_with("M") || s.ends_with("MB") {
        (s.trim_end_matches("MB").trim_end_matches('M'), 1024.0 * 1024.0)
    } else if s.ends_with("K") || s.ends_with("KB") {
        (s.trim_end_matches("KB").trim_end_matches('K'), 1024.0)
    } else {
        (s.as_str(), 1.0)
    };

    num_str.trim().parse::<f64>().ok().map(|n| (n * multiplier) as u64)
}

/// Format bytes as human-readable string
pub fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        DictionaryConfig, IconsConfig, LayoutConfig, LoggingConfig, RetentionConfig,
        StructureConfig, TagConfig, ThemeConfig, ThemeMeta, ThemeSettings,
    };
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn create_mock_config() -> Cookbook {
        Cookbook {
            theme: ThemeConfig {
                meta: ThemeMeta {
                    name: "Test".to_string(),
                },
                settings: ThemeSettings {
                    active_icons: "nerdfont".to_string(),
                },
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
                    prefix: "[".to_string(),
                    suffix: "]".to_string(),
                    transform: "uppercase".to_string(),
                    min_width: 10,
                    alignment: "center".to_string(),
                },
                labels: HashMap::new(),
                structure: StructureConfig {
                    terminal: "".to_string(),
                    file: "{msg}".to_string(),
                },
                logging: LoggingConfig {
                    base_dir: "".to_string(), // will be set
                    path_structure: "{app}/{scope}".to_string(),
                    filename_structure: "log.txt".to_string(),
                    timestamp_format: "%Y".to_string(),
                    write_by_default: true,
                    app_name: "default_app".to_string(),
                    retention: RetentionConfig::default(),
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
    fn test_log_to_file_default_app() {
        let dir = tempdir().unwrap();
        let mut config = create_mock_config();
        config.layout.logging.base_dir = dir.path().to_str().unwrap().to_string();

        log_to_file(&config, "info", "MAIN", "test message", None).unwrap();

        let expected_path = dir.path().join("default_app/MAIN/log.txt");
        assert!(expected_path.exists());

        let content = fs::read_to_string(expected_path).unwrap();
        assert!(content.contains("test message"));
    }

    #[test]
    fn test_log_to_file_app_override() {
        let dir = tempdir().unwrap();
        let mut config = create_mock_config();
        config.layout.logging.base_dir = dir.path().to_str().unwrap().to_string();

        log_to_file(
            &config,
            "info",
            "MAIN",
            "test message",
            Some("OverriddenApp"),
        )
        .unwrap();

        let expected_path = dir.path().join("OverriddenApp/MAIN/log.txt");
        assert!(expected_path.exists());
    }
}
