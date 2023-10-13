extern crate pnet;

use std::process::Command;

use lazy_static::lazy_static;
use pnet::datalink::{self};
use regex::Regex;
use sysinfo::SystemExt;

use crate::common;
use crate::common::PackageData;

const ICON_FG: &str = common::PINK;
const ICON_BG: &str = common::BLACK;
const ICON_TR: &str = "0xff";
const TEXT_FG: &str = common::PINK;
const TEXT_BG: &str = common::BLACK;
const TEXT_TR: &str = "0xff";

lazy_static! {
    static ref ICON_COLOR: String = format!("^c{}^^b{}{}^", ICON_FG, ICON_BG, ICON_TR);
    static ref TEXT_COLOR: String = format!("^c{}^^b{}{}^", TEXT_FG, TEXT_BG, TEXT_TR);
    static ref DELAY_TIME: i32 = *common::packages_lists().get(NAME).unwrap();
}
const NAME: &str = "wifi";

#[cfg(test)]
#[test]
pub fn test() {
    println!("get() = {:?}", get());
    let expected_regex = Regex::new(r"\^swifi\^\^c#ff79c6\^\^b#282a360xff\^ 󰤨 ").unwrap();
    assert!(expected_regex.is_match(&get().data));
}

pub fn get() -> PackageData {
    // print_interfaces();


    match get_current_network_name() {
        Some(network_name) => println!("Current network name: {}", network_name),
        None => eprintln!("Failed to get network name."),
    }

    let text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, "󰤨");


    PackageData::new(NAME, text)
}

fn print_interfaces() {
    let interfaces = datalink::interfaces();

    for interface in interfaces {
        println!("Interface Name: {}", interface.name);
        println!("Interface Description: {}", interface.description);

        println!("Interface: {:?}", interface);


        if let Some(ipv4) = interface.ips.into_iter().find(|ip| ip.is_ipv4()) {
            println!("IPv4 Address: {}", ipv4.ip());
            println!("Subnet Mask: {}", ipv4.mask());
        }

        if let Some(mac) = interface.mac {
            println!("MAC Address: {}", mac);
        }

        println!();
    }
}


fn get_current_network_name() -> Option<String> {
    let output = Command::new("iwgetid")
        .arg("-r")
        .output()
        .ok()?;

    String::from_utf8(output.stdout).ok()
}

