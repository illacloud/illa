use crate::{command::*, result::Result};
use anyhow::Ok;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs};

pub fn local_bind_init() -> Result {
    if env::consts::OS == "macos" {
        // check the local directory
        fs::create_dir_all("/tmp/illa-data");
        let mut perms = fs::metadata("/tmp/illa-data")?.permissions();
        perms.set_mode(0o777);
        fs::set_permissions("/tmp/illa-data", perms);
    }

    Ok(())
}
