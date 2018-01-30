#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[cfg(test)]
mod tests;

extern crate base64;
extern crate byteorder;
extern crate crc24;

use std::str::FromStr;
use base64::decode;
use crc24::hash_raw;

#[derive(Debug)]
pub struct PgpDump {}

/// Strip PGP headers and end lines from an ASCII Armored PGP key.
fn strip_magic(data: &str) -> &str {
    // ASCII Armored PGP keys start with headers and are followed by a blank link and then data
    let start = data.find("\n\n").expect("Could not find start");
    let end = data.rfind("-----END").expect("Could not find end");
    data[start..end].trim_right()
}

/// A PGP data block in Radix-64 format can optionally include a CRC24 checksum consisting of an
/// equal sign followed by 4 alphanumeric characters. These 4 alphanumeric characters correspond to
/// 3 digits or 24 bits.
fn find_crc_checksum(data: &str) -> Result<Option<Vec<u8>>, base64::DecodeError> {
    let end = data.len();
    let r = data.as_bytes()[end - 5] as char;
    match r {
        '=' => match decode(&data[end - 4..end]) {
            Ok(x) => Ok(Some(x)),
            Err(e) => Err(e),
        },
        _ => Ok(None),
    }
}

impl FromStr for PgpDump {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("-----BEGIN PGP ") {
            let data = strip_magic(s);
            // Verify that the CRC24 checksum matches if it exists
            let known_crc = find_crc_checksum(data);
            match known_crc {
                Ok(Some(checksum)) => {
                    let end = data.len() - 5;
                    let computed_crc = hash_raw(&data[0..end].as_bytes());
                    println!("{:?}", computed_crc);
                    println!("{:?}", checksum);
                }
                Ok(None) => println!("No checksum found"),
                Err(_) => println!("Problem base64 decoding crc24 checksum"),
            }
            Err("Not implemented yet.")
        } else {
            Err("Does not start with PGP string")
        }
    }
}

impl PgpDump {
    pub fn get_packets(&self) -> Vec<u8> {
        unimplemented!()
    }
}
