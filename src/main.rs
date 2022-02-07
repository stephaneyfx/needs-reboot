// Copyright (C) 2022 Stephane Raux. Distributed under the 0BSD license.

#![deny(warnings)]

use clap::Parser;

fn main() {
    let _ = Args::parse();
    match needs_reboot::check() {
        Ok(true) => println!("Yes"),
        Ok(false) => println!("No"),
        Err(e) => {
            eprintln!("Failed: {}", e);
            std::process::exit(1);
        }
    }
}

/// Tool indicating if a system should be rebooted.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args;
