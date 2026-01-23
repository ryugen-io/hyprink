use crate::logging::{info, warn};
use anyhow::Result;
use hi_core::config::Config;
use hi_core::db::Store;
use hi_core::processor;

pub fn execute(db: &Store, config: &Config, force: bool) -> Result<()> {
    let templates = db.list();
    if templates.is_empty() {
        info("APPLY", "No templates to apply");
        return Ok(());
    }

    let total = templates.len();
    let mut hook_failures = 0;
    let mut skipped = 0;

    for tpl in templates {
        if tpl.manifest.ignored {
            info(
                "APPLY",
                &format!(
                    "ignoring <secondary>{}</secondary> (disabled)",
                    tpl.manifest.name
                ),
            );
            skipped += 1;
            continue;
        }

        info(
            "APPLY",
            &format!("applying <primary>{}</primary>", tpl.manifest.name),
        );
        if !processor::apply(tpl, config, force)? {
            hook_failures += 1;
        }
    }

    let applied = total - skipped;

    if hook_failures > 0 {
        warn(
            "APPLY",
            &format!(
                "applied {} templates ({} skipped) but {} hooks failed",
                applied, skipped, hook_failures
            ),
        );
    } else {
        info(
            "APPLY",
            &format!("applied {} templates ({} skipped)", applied, skipped),
        );
    }

    Ok(())
}
