//! Internal commands for one off fixes.

// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{bail, Context, Result};
use std::path;

use crate::core::OSTREE_BOOTED;

/// Stamp file used to avoid repeating this on every boot
const STAMP_FILE: &str = "/etc/.rpm-ostree-shadow-mode-fixed2.stamp";

/// Main entrypoint for the hidden "internals" sub command.
/// Supported sub-commands:
/// - fix-shadow: Fix mode on /etc/[g]shadow[-] files in all deployments (CVE-2024-2905)
pub fn entrypoint(_args: &[&str]) -> Result<()> {
    // Skip if we are not run on an ostree booted system
    if !path::Path::new(OSTREE_BOOTED).exists() {
        bail!("Not running on an ostree based system");
    }

    // Skip if we are not running as root
    if rustix::process::geteuid().as_raw() != 0 {
        bail!("Must run as root");
    }

    // Iterate over all deployments and fix mode on /etc/[g]shadow[-]

    // Touch stamp file
    // touch /etc/.rpm-ostree-shadow-mode-fixed2.stamp
    let statedir = Dir::open_ambient_dir(STATE_DIR, cap_std::ambient_authority())?;
    statedir.atomic_replace_with(COUNTME_COOKIE, |w| -> Result<_> {
        Ok(serde_json::to_writer(w, &cookie)?)
    });

    println!("Successful requests: {}/{}", successful, repos.len());
    if let Err(e) = cookie.persist() {
        // Do not exit with a non zero code here as we have still made at least
        // one successful request thus we have been counted.
        eprintln!("Failed to persist cookie: {}", e);
    }
    Ok(())
}
