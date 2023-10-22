use chrono::Local;
#[cfg(test)]
use regex::Regex;

use crate::common;
use crate::common::{Button, ICON_COLOR, PackageData, TEXT_COLOR};

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