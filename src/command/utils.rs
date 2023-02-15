use crate::{command::*, result::Result};
use anyhow::Ok;
use std::{env, fs};

#[cfg(target_os = "macos")]
pub fn local_bind_init(path: &String) -> String {
    use std::os::unix::fs::PermissionsExt;
    fs::create_dir_all(path.clone());
    let attr = fs::metadata(path.clone()).unwrap();
    let mut perms = attr.permissions();
    perms.set_mode(0o777);
    fs::set_permissions(path.clone(), perms);

    String::from(path)
}

#[cfg(target_os = "windows")]
pub fn local_bind_init(path: &String) -> String {
    fs::create_dir_all(path.clone());

    String::from(path)
}

#[cfg(target_os = "linux")]
pub fn local_bind_init(path: &String) -> String {
    fs::create_dir_all(path.clone());

    String::from(path)
}

pub fn local_bind_delete(path: String) -> Result {
    fs::remove_dir_all(path);

    Ok(())
}

pub fn get_default_mount() -> String {
    let tmp_dir = env::temp_dir();
    let temp_dir = tmp_dir.join("illa-builder");

    temp_dir.display().to_string()
}
