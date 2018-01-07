#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![deny(warnings)]
#![allow(unknown_lints)]

extern crate log;
extern crate yaml_rust;

extern crate pgpdump;

#[macro_use]
extern crate clap;
use clap::App;

use std::process::exit;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let _verbosity = match matches.occurrences_of("v") {
        0 => log::Level::Error,
        1 => log::Level::Info,
        2 => log::Level::Debug,
        3 | _ => log::Level::Trace,
    };

    exit(0);
}
