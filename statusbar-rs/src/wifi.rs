use lazy_static::lazy_static;
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
}
const NAME: &str = "wifi";

const CMD: &str = "nmcli -t -f TYPE,STATE,NAME -e no connection show";

pub enum NetType {
    WIFI(String),
    ETHERNET(String),
    OFFLINE,
}

impl NetType {
    pub fn get_icon(&self) -> &str {
        match self {
            NetType::WIFI(_) => "󰤨", // 替换为你实际使用的 Wi-Fi 图标
            NetType::ETHERNET(_) => "󰈀", // 替换为以太网图标
            NetType::OFFLINE => "󰤭", // 替换为离线图标
        }
    }

    pub fn get_network_name(&self) -> Option<&str> {
        match self {
            NetType::WIFI(ssid) => Some(ssid),
            NetType::ETHERNET(name) => Some(name),
            NetType::OFFLINE => None,
        }
    }
}


#[cfg(test)]
#[test]
pub fn test() {
    println!("get() = {:?}", get());
    let expected_regex = Regex::new(r"\^swifi\^\^c#ff79c6\^\^b#282a360xff\^ 󰤨 ").unwrap();
    assert!(expected_regex.is_match(&get().data));
}

pub fn get() -> PackageData {
    // print_interfaces();


    // match get_current_network_name() {
    //     Some(network_name) => println!("Current network name: {}", network_name),
    //     None => eprintln!("Failed to get network name."),
    // }



    let net_type = get_network_type();
    println!("{}", net_type.get_network_name().unwrap_or("无网络连接"));

    let text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, get_network_type().get_icon());


    PackageData::new(NAME, text)
}


fn get_network_type() -> NetType {
    let output = common::cmd(CMD);
    for line in output.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() == 3 {
            let device_type = fields[0];
            let device_state = fields[1];
            let device_name = fields[2];
            if device_type.contains("wireless") && device_state == "activated" {
                return NetType::WIFI(device_name.to_string());
            } else if device_type.contains("ethernet") && device_state == "activated" {
                return NetType::ETHERNET(device_name.to_string());
            }
        }
    }
    NetType::OFFLINE
}



