use lazy_static::lazy_static;

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
const NAME: &str = "battery";


pub fn get() -> PackageData {
    let output = std::process::Command::new("acpi")
        .output()
        .expect("failed to execute process");

    let mut output = String::from_utf8(output.stdout).unwrap();
    output = output.trim().to_string();

    // 从字符串中"Battery 0: Not charging, 99%"提取出是否正在充电的信息(Not charging)
    let not_charging: bool = (output.contains("Not charging") || output.contains("Discharging"));

    if !not_charging {
        // 如果正在充电，那么就显示充电图标
        let text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, "");
        return PackageData::new(NAME, text);
    }

    let mut output = output.split(',').collect::<Vec<&str>>();
    let mut output = output[1].to_string();
    output = output.trim().to_string();
    // 从99%中提取出99
    let mut output = output.split('%').collect::<Vec<&str>>();
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

#[cfg(test)]
#[test]
pub fn test() {
    assert_eq!(get().data, "^sbattery^^c#ff79c6^^b#282a360xff^  ");
}