use crate::logging::log_msg;
use anyhow::Result;
use hi_core::config::Config;
use hi_core::packager;
use std::path::PathBuf;

pub fn execute(input: PathBuf, output: Option<PathBuf>, config: &Config) -> Result<()> {
    let out = output.unwrap_or_else(|| {
        let name = input.file_name().unwrap_or_default().to_string_lossy();
        PathBuf::from(format!("{}.pkg", name))
    });
    packager::pack(&input, &out)?;
    log_msg(
        config,
        "pack_ok",
        &format!("packed templates to {}", out.display()),
    );
    Ok(())
}
