// Copyright (C) 2022 Stephane Raux. Distributed under the 0BSD license.

//! # Overview
//! - [📦 crates.io](https://crates.io/crates/needs-reboot)
//! - [📖 Documentation](https://docs.rs/needs-reboot)
//! - [⚖ 0BSD license](https://spdx.org/licenses/0BSD.html)
//!
//! Crate to tell if a linux system needs to be rebooted
//!
//! This can help to decide whether to reboot a system after updating it. Currently only NixOS is
//! supported.
//!
//! # Features
//! - `tool`: Enables a CLI tool to check if a system should be rebooted.
//!
//! # Contribute
//! All contributions shall be licensed under the [0BSD license](https://spdx.org/licenses/0BSD.html).

#![deny(missing_docs)]
#![deny(warnings)]

use std::{io, path::Path};

/// Returns `true` is the system should be rebooted, else `false`.
pub fn check() -> Result<bool, io::Error> {
    let booted = Path::new("/run/booted-system");
    let new = Path::new("/nix/var/nix/profiles/system");
    ["initrd", "kernel", "kernel-modules"]
        .into_iter()
        .map(
            |component| Ok(booted.join(component).read_link()? != new.join(component).read_link()?),
        )
        .find(|needs| !matches!(needs, Ok(false)))
        .unwrap_or(Ok(false))
}
