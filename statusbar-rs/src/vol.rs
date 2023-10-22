use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;
#[cfg(test)]
use regex::Regex;

use crate::common::{Button, cmd, ICON_COLOR, PackageData, TEXT_COLOR};

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
    let text: String;
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