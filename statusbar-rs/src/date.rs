use chrono::Local;
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
const NAME: &str = "date";

pub fn get() -> PackageData {
    let current_time = Local::now();
    let formatted_time = current_time.format("%H:%M:%S");
    let text = format!("^s{}^{}{}{} {} ", NAME, *ICON_COLOR, "", *TEXT_COLOR, formatted_time);


    PackageData::new(NAME, text)
}

pub fn api(button: Button) {
    match button {
        Button::LEFT => {
            let cmd = format!("notify-send \"{}\" \"{}\" -r {}",
                              " Calendar",
                              "\nData: $(date \'+%y-%m-%d \\nTime: %T\')",
                              9540
            );
            common::cmd(&cmd);
        }
        Button::RIGHT => {}
        Button::MIDDLE => {}
        Button::UP => {}
        Button::DOWN => {}
    }
}

#[cfg(test)]
#[test]
pub fn test() {
    println!("get() = {:?}", get());
    let expected_regex = Regex::new(r"\^sdate\^\^c#ff79c6\^\^b#282a360xff\^\^c#ff79c6\^\^b#282a360xff\^ ..:..:.. ").unwrap();
    assert!(expected_regex.is_match(&get().data));
}