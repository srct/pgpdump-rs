#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[cfg(test)]
mod tests;

use std::str::FromStr;

#[derive(Debug)]
pub struct PgpDump {}

impl FromStr for PgpDump {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err("Not implemented yet.")
    }
}

impl PgpDump {
    pub fn get_packets(&self) -> Vec<u8> {
        unimplemented!()
    }
}
