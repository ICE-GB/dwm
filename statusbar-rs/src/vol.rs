use lazy_static::lazy_static;
use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;
use regex::Regex;
use sysinfo::SystemExt;

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
    static ref DELAY_TIME: i32 = *common::packages_lists().get(NAME).unwrap();
}
const NAME: &str = "vol";

#[cfg(test)]
#[test]
pub fn test() {
    println!("get() = {:?}", get());
    let expected_regex = Regex::new(r"\^svol\^\^c#ff79c6\^\^b#282a360xff\^ .+").unwrap();
    assert!(expected_regex.is_match(&get().data));
}

pub fn get() -> PackageData {
    // create handler that calls functions on playback devices and apps
    let mut handler = SinkController::create().unwrap();
    let devices = handler.get_default_device().expect("Could not get default device.");
    let mut text: String;
    if devices.mute {
        text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, "󰝟");
    } else {
        text = format!("^s{}^{} {} {}{} ", NAME, *ICON_COLOR, "", *TEXT_COLOR, devices.volume.avg().print());
    }

    PackageData::new(NAME, text)
}

pub fn api(button: Button) {
    match button {
        Button::LEFT => {
            cmd("pactl set-sink-mute @DEFAULT_SINK@ toggle");
        }
        Button::RIGHT => {
            cmd("killall pavucontrol || pavucontrol --class floatingTerminal &");
        }
        Button::MIDDLE => {
            cmd("pactl set-sink-mute @DEFAULT_SINK@ toggle");
        }
        Button::UP => {
            cmd("pactl set-sink-volume @DEFAULT_SINK@ +5%; notify");
        }
        Button::DOWN => {
            cmd("pactl set-sink-volume @DEFAULT_SINK@ -5%; notify");
        }
    }
}