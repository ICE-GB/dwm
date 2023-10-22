#[cfg(test)]
use regex::Regex;

use crate::common::{Button, cmd, ICON_COLOR, PackageData, TEXT_COLOR};

const NAME: &str = "icon";

pub fn get() -> PackageData {
    let text = format!("^s{}^{} {} {}{} ", NAME, *ICON_COLOR, "", *TEXT_COLOR, "");


    PackageData::new(NAME, text)
}

pub fn api(button: Button) {
    match button {
        Button::LEFT => {
            cmd("~/.config/rofi/scripts/powermenu_t2");
        }
        Button::RIGHT => {
            cmd("feh --randomize --bg-fill ~/Pictures/wallpaper/*.*");
        }
        Button::MIDDLE => {}
        Button::UP => {}
        Button::DOWN => {}
    }
}

#[cfg(test)]
#[test]
pub fn test() {
    println!("get() = {:?}", get());
    let expected_regex = Regex::new(r"\^sicon\^\^c#ff79c6\^\^b#282a360xff\^  \^c#ff79c6\^\^b#282a360xff\^").unwrap();
    assert!(expected_regex.is_match(&get().data));
}