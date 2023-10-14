use lazy_static::lazy_static;
#[cfg(test)]
use regex::Regex;

use crate::common;
use crate::common::{Button, cmd, PackageData};

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

const WIFI_ICON: &str = "󰤨";
const ETHERNET_ICON: &str = "󰈀";
const OFFLINE_ICON: &str = "󰤭";

pub enum NetType {
    WIFI(String),
    ETHERNET(String),
    OFFLINE,
}

impl NetType {
    pub fn get_icon(&self) -> &str {
        match self {
            NetType::WIFI(_) => WIFI_ICON,
            NetType::ETHERNET(_) => ETHERNET_ICON,
            NetType::OFFLINE => OFFLINE_ICON,
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



    // let net_type = get_network_type();
    // println!("{}", net_type.get_network_name().unwrap_or("无网络连接"));

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

pub fn api(button: Button) {
    match button {
        Button::LEFT => {
            let network = get_network_type();
            match network {
                NetType::WIFI(name) => {
                    cmd(&format!("notify-send '已连接到 {} {}'", WIFI_ICON, name));
                }
                NetType::ETHERNET(name) => {
                    cmd(&format!("notify-send '已连接到 {} {}'", ETHERNET_ICON, name));
                }
                NetType::OFFLINE => {
                    cmd(&format!("notify-send '无网络连接 {}'", OFFLINE_ICON));
                }
            }
        }
        Button::RIGHT => {}
        Button::MIDDLE => {}
        Button::UP => {}
        Button::DOWN => {}
    }
}



