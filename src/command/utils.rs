use crate::{command::*, result::Result};
use anyhow::Ok;
use std::{env, fs};

#[cfg(target_os = "macos")]
pub fn local_bind_init() -> Result {
    use std::os::unix::fs::PermissionsExt;
    fs::create_dir_all("/tmp/illa-data");
    let mut perms = fs::metadata("/tmp/illa-data")?.permissions();
    perms.set_mode(0o777);
    fs::set_permissions("/tmp/illa-data", perms);

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn local_bind_init() -> Result {
    fs::create_dir_all("/tmp/illa-data");

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn local_bind_init() -> Result {
    fs::create_dir_all("/tmp/illa-data");

    Ok(())
}

pub fn local_bind_delete() -> Result {
    fs::remove_dir_all("/tmp/illa-data");

    Ok(())
}
