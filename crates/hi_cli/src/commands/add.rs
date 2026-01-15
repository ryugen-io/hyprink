use crate::logging::log_msg;
use anyhow::{Context, Result, anyhow};
use hi_core::config::Config;
use hi_core::db::Store;
use hi_core::template::Template;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn add_template(path: &Path, db: &mut Store, config: &Config) -> Result<Vec<Template>> {
    let mut installed_list = Vec::new();

    if !path.exists() {
        return Err(anyhow!("File not found: {:?}", path));
    }

    if path.extension().is_some_and(|ext| ext == "pkg") {
        let file = fs::File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name().ends_with(".tpl") {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let tpl: Template = toml::from_str(&content).with_context(|| {
                    format!("Failed to parse template inside package: {}", file.name())
                })?;

                log_msg(
                    config,
                    "add_ok",
                    &format!("added {} v{}", tpl.manifest.name, tpl.manifest.version),
                );

                let tpl_clone = tpl.clone();
                db.add(tpl)?;
                installed_list.push(tpl_clone);
            }
        }
    } else {
        let content = fs::read_to_string(path)?;
        let tpl: Template = toml::from_str(&content)
            .with_context(|| format!("Failed to parse template: {:?}", path))?;

        log_msg(
            config,
            "add_ok",
            &format!("added {} v{}", tpl.manifest.name, tpl.manifest.version),
        );
        let tpl_clone = tpl.clone();
        db.add(tpl)?;
        installed_list.push(tpl_clone);
    }
    Ok(installed_list)
}
