use std::time::Duration;

use lazy_static::lazy_static;

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

#[derive(Clone)]
pub struct Package {
    pub name: &'static str,
    pub delay_time: Duration,
    pub fuc: fn() -> PackageData,
    pub control_fuc: fn(Button),
}

#[derive(Debug)]
pub struct PackageData {
    pub module_name: &'static str,
    pub data: String,
}

impl Package {
    pub fn new(name: &'static str, delay_time: Duration, fuc: fn() -> PackageData, control_fuc: fn(Button)) -> Self {
        Self {
            name,
            delay_time,
            fuc,
            control_fuc,
        }
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

#[derive(Debug, Clone)]
pub enum Button {
    LEFT,
    RIGHT,
    MIDDLE,
    UP,
    DOWN,
}

impl Button {
    pub(crate) fn from_str(button: &str) -> Button {
        match button {
            "L" => { Button::LEFT }
            "M" => { Button::MIDDLE }
            "R" => { Button::RIGHT }
            "U" => { Button::UP }
            "D" => { Button::DOWN }
            _ => Button::LEFT
        }
    }
}

pub(crate) trait Control {
    fn control(&self, button: Button) {
        match button {
            Button::LEFT => {}
            Button::RIGHT => {}
            Button::MIDDLE => {}
            Button::UP => {}
            Button::DOWN => {}
        }
    }
}

impl Control for Package {
    fn control(&self, button: Button) {
        (self.control_fuc)(button)
    }
}

#[derive(Clone, Debug)]
pub struct Command {
    pub(crate) name: String,
    pub(crate) button: Button,
}

impl Command {
    pub(crate) fn new(name: String, button: Button) -> Command {
        Command {
            name,
            button,
        }
    }
}