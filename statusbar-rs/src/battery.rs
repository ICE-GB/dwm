use lazy_static::lazy_static;
#[cfg(test)]
use regex::Regex;

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
}
const NAME: &str = "battery";


pub fn get() -> PackageData {
    let output = std::process::Command::new("acpi")
        .output()
        .expect("failed to execute process");

    let mut output = String::from_utf8(output.stdout).unwrap();
    output = output.trim().to_string();

    if output.contains("Charging") {
        // 如果正在充电，那么就显示充电图标
        let text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, "");
        return PackageData::new(NAME, text);
    }

    let output = output.split(',').collect::<Vec<&str>>();
    let mut output = output[1].to_string();
    output = output.trim().to_string();
    // 从99%中提取出99
    let output = output.split('%').collect::<Vec<&str>>();
    let percent = output[0].to_string().parse::<i32>().unwrap();


    let battery_icon = match percent {
        0..=10 => "",
        11..=25 => "",
        26..=50 => "",
        51..=75 => "",
        76..=100 => "",
        _ => "",
    };

    let text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, battery_icon);

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

#[cfg(test)]
#[test]
pub fn test() {
    let expected_regex = Regex::new(r"\^sbattery\^\^c#ff79c6\^\^b#282a360xff\^ . ").unwrap();
    assert!(expected_regex.is_match(&get().data));
}