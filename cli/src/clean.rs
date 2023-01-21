use std::fs;

use anyhow::{bail, Context};
use eludris::{check_eludris_exists, check_user_permissions, end_progress_bar, new_progress_bar};

pub fn clean() -> anyhow::Result<()> {
    check_user_permissions()?;

    if !check_eludris_exists()? {
        bail!("Could not find an Eludris instance on this machine");
    }

    let bar = new_progress_bar("Removing old instance files...");
    fs::remove_dir_all("/usr/eludris").context("Could not remove Eludris instance files")?;
    end_progress_bar(bar, "Removed old instance files");
    Ok(())
}
