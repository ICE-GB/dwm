use std::process::Command;
use std::sync::RwLock;

use lazy_static::lazy_static;
use regex::Regex;
use sysinfo::{NetworkExt, System, SystemExt};

use crate::common;
use crate::common::{Button, PackageData};

const ICON_FG: &str = common::PINK;
const ICON_BG: &str = common::BLACK;
const ICON_TR: &str = "0xff";
const TEXT_FG: &str = common::PINK;
const TEXT_BG: &str = common::BLACK;
const TEXT_TR: &str = "0xff";

lazy_static! {
    static ref ICON_COLOR: String = format!("^c{}^^b{}{}^", ICON_FG, ICON_BG, ICON_TR);
    static ref TEXT_COLOR: String = format!("^c{}^^b{}{}^", TEXT_FG, TEXT_BG, TEXT_TR);
    static ref SYSTEM: RwLock<System> = RwLock::new(System::new_all());
    static ref DELAY_TIME: i32 = *common::packages_lists().get(NAME).unwrap();
}
const NAME: &str = "net";

#[cfg(test)]
#[test]
pub fn test() {
    println!("get() = {:?}", get());
    let expected_regex = Regex::new(r"\^snet\^\^c#ff79c6\^\^b#282a360xff\^ .*").unwrap();
    assert!(expected_regex.is_match(&get().data));
}

pub fn get() -> PackageData {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = SYSTEM.write().unwrap();

    // First we update all information of our `System` struct.
    sys.refresh_networks_list();

    let current_interface_name = get_current_interface_name();
    let mut rc = 0;
    let mut tr = 0;

    // Network interfaces name, data received and data transmitted:
    for (interface_name, data) in sys.networks() {
        if interface_name.eq(&current_interface_name) {
            rc = data.received();
            tr = data.transmitted();
            break;
        }
    }
    let net = format!("{} {}", format_bytes(tr), format_bytes(rc));
    let text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, net);


    PackageData::new(NAME, text)
}

pub fn api(button: Button) {
    match button {
        Button::LEFT => {}
        Button::RIGHT => {}
        Button::MIDDLE => {}
        Button::UP => {}
        Button::DOWN => {}
    }
}

fn get_current_interface_name() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ip route | grep default | cut -d' ' -f5")
        .output()
        .unwrap();

    let trimmed_output = String::from_utf8(output.stdout).unwrap();

    // println!("{}", trimmed_output);

    let interface_name = trimmed_output.trim().to_string();
    interface_name
}

fn format_bytes(bytes: u64) -> String {
    let units = [" B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    let formatted_size = if size.fract() < 0.01 {
        format!("{:.0}", size.round() as u64)
    } else {
        format!("{:.2}", size)
    };

    let padding = 6 - formatted_size.len();
    let formatted_unit = units[unit_index];

    format!("{}{} {}", " ".repeat(padding), formatted_size, formatted_unit)
}