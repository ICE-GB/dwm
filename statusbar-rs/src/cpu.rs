use std::sync::RwLock;

use lazy_static::lazy_static;
use regex::Regex;
use sysinfo::{CpuExt, System, SystemExt};

use crate::common;
use crate::common::{Button, PackageData};

lazy_static! {
    static ref DELAY_TIME: i32 = *common::packages_lists().get(NAME).unwrap();
    static ref SYSTEM: RwLock<System> = RwLock::new(System::new_all());
}

const NAME: &str = "cpu";

pub fn get() -> PackageData {
    let mut system = SYSTEM.write().unwrap();
    system.refresh_cpu();
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let icon = if cpu_usage > 50.0 {
        ""
    } else {
        ""
    };

    let output = std::process::Command::new("cat")
        .arg("/sys/class/thermal/thermal_zone0/temp")
        .output()
        .expect("failed to execute process");
    let mut output = String::from_utf8(output.stdout).unwrap();
    output = output.trim().to_string();

    let temperature: f64 = output.trim().parse().expect("Failed to parse temperature");
    let temperature = (temperature / 1000.0) as i32;

    let mut text: String;

    if cpu_usage < 10.0 {
        text = format!(" {:.0}% {}", cpu_usage, temperature);
    } else { text = format!("{:.0}% {}", cpu_usage, temperature); }

    // Assuming name and text_color are defined elsewhere
    let text = format!("^s{}^{} {} {}{} ", NAME, *common::ICON_COLOR, icon, *common::TEXT_COLOR, text);

    PackageData::new(NAME, text)
}

pub fn api(button: Button) {
    match button {
        Button::LEFT => {
            let cmd = format!("notify-send \"{}\" \"{}\" -r {}",
                              "  CPU tops",
                              "$(ps axch -o cmd:15,%cpu --sort=-%cpu | head  | sed \'s/$/&\\%\\n/g\')",
                              1014
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
    let expected_regex = Regex::new(r"\^scpu\^\^c#ff79c6\^\^b#282a360xff\^ .+ \^c#ff79c6\^\^b#282a360xff\^.+% .+ ").unwrap();
    assert!(expected_regex.is_match(&get().data));
}