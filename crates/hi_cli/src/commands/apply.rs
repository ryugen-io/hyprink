use crate::logging::{log, log_msg};
use anyhow::Result;
use hi_core::config::Config;
use hi_core::db::Store;
use hi_core::processor;

pub fn execute(db: &Store, config: &Config, force: bool) -> Result<()> {
    let templates = db.list();
    if templates.is_empty() {
        log(config, "apply_empty");
        return Ok(());
    }

    let total = templates.len();
    let mut hook_failures = 0;
    let mut skipped = 0;

    for tpl in templates {
        if tpl.manifest.ignored {
            log_msg(
                config,
                "apply_skip",
                &format!(
                    "ignoring <secondary>{}</secondary> (disabled)",
                    tpl.manifest.name
                ),
            );
            skipped += 1;
            continue;
        }

        log_msg(
            config,
            "apply_start",
            &format!("applying <primary>{}</primary>", tpl.manifest.name),
        );
        if !processor::apply(tpl, config, force)? {
            hook_failures += 1;
        }
    }

    let applied = total - skipped;

    if hook_failures > 0 {
        log_msg(
            config,
            "apply_ok",
            &format!(
                "applied {} templates ({} skipped) but {} hooks failed",
                applied, skipped, hook_failures
            ),
        );
    } else {
        log_msg(
            config,
            "apply_ok",
            &format!("applied {} templates ({} skipped)", applied, skipped),
        );
    }

    Ok(())
}
