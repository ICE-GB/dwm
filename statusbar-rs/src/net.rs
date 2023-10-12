use std::sync::RwLock;

use lazy_static::lazy_static;
use regex::Regex;
use sysinfo::{NetworkExt, System, SystemExt};

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

    // Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }
    let text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, "");


    PackageData::new(NAME, text)
}