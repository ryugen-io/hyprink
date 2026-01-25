use super::super::logging::info;
use crate::config::Config;
use crate::packager;
use anyhow::Result;
use std::path::PathBuf;

pub fn execute(input: PathBuf, output: Option<PathBuf>, _config: &Config) -> Result<()> {
    let out = output.unwrap_or_else(|| {
        let name = input.file_name().unwrap_or_default().to_string_lossy();
        PathBuf::from(format!("{}.pkg", name))
    });
    packager::pack(&input, &out)?;
    info("PACK", &format!("packed templates to {}", out.display()));
    Ok(())
}
