use crate::{command::*, result::Result};
use anyhow::Ok;
use std::{env, fs};

#[cfg(target_os = "macos")]
pub fn local_bind_init() -> String {
    use std::os::unix::fs::PermissionsExt;
    fs::create_dir_all("/tmp/illa-data");
    let attr = fs::metadata("/tmp/illa-data").unwrap();
    let mut perms = attr.permissions();
    perms.set_mode(0o777);
    fs::set_permissions("/tmp/illa-data", perms);

    String::from("/tmp/illa-data")
}

#[cfg(target_os = "windows")]
pub fn local_bind_init() -> String {
    fs::create_dir_all("C:/tmp/illa-data");

    String::from("C:/tmp/illa-data")
}

#[cfg(target_os = "linux")]
pub fn local_bind_init() -> String {
    fs::create_dir_all("/tmp/illa-data");

    String::from("/tmp/illa-data")
}

pub fn local_bind_delete() -> Result {
    if cfg!(windows) {
        fs::remove_dir_all("C:/tmp/illa-data");
    } else {
        fs::remove_dir_all("/tmp/illa-data");
    }

    Ok(())
}
