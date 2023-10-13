use std::collections::HashMap;
use std::time::Duration;

use lazy_static::lazy_static;

pub const DWM_PATH: &str = "/home/gb/.dwm/";
pub const PACKAGES_PATH: &str = "/home/gb/.dwm/statusbar/";
pub const TEMP_FILE: &str = "/home/gb/python_tmp";

pub const MUSIC_PROGRAM: &str = "yesplaymusic";

pub const BLACK: &str = "#282a36";
pub const WHITE: &str = "#f8f8f2";
pub const GREY: &str = "#44475a";
pub const BLUE: &str = "#6272a4";
pub const BLUE2: &str = "#bd93f9";
pub const BLUE3: &str = "#8be9fd";
pub const BLUE4: &str = "#50fa7b";
pub const RED: &str = "#ff5555";
pub const GREEN: &str = "#50fa7b";
pub const PINK: &str = "#ff79c6";
pub const YELLOW: &str = "#f1fa8c";
pub const ORANGE: &str = "#ffb86c";
pub const DARKBLUE: &str = "#6272a4";

pub fn packages_lists() -> HashMap<&'static str, i32> {
    let mut packages_lists: HashMap<&str, i32> = HashMap::new();
    packages_lists.insert("music", 1);
    packages_lists.insert("wifi", 5);
    packages_lists.insert("net", 1);
    packages_lists.insert("cpu", 2);
    packages_lists.insert("memory", 2);
    packages_lists.insert("vol", 2);
    packages_lists.insert("battery", 10);
    packages_lists.insert("date", 1);
    packages_lists.insert("icon", 100);
    return packages_lists;
}

const ICON_FG: &str = PINK;
const ICON_BG: &str = BLACK;
const ICON_TR: &str = "0xff";
const TEXT_FG: &str = PINK;
const TEXT_BG: &str = BLACK;
const TEXT_TR: &str = "0xff";

lazy_static! {
    pub static ref ICON_COLOR: String = format!("^c{}^^b{}{}^", ICON_FG, ICON_BG, ICON_TR);
    pub static ref TEXT_COLOR: String = format!("^c{}^^b{}{}^", TEXT_FG, TEXT_BG, TEXT_TR);
}


pub struct Package {
    pub name: &'static str,
    pub delay_time: Duration,
    pub fuc: fn() -> PackageData,
    pub text: String,
}

#[derive(Debug)]
pub struct PackageData {
    pub module_name: &'static str,
    pub data: String,
}

impl Package {
    pub fn new(name: &'static str, delay_time: Duration, fuc: fn() -> PackageData) -> Self {
        Self {
            name,
            delay_time,
            fuc,
            text: String::new(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl PackageData {
    pub fn new(module_name: &'static str, data: String) -> Self {
        Self {
            module_name,
            data,
        }
    }
}

pub fn cmd(cmd: &str) -> String {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    let mut output = String::from_utf8(output.stdout).unwrap();
    output = output.trim().to_string();
    output
}