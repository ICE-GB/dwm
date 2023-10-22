use std::time::Duration;

use lazy_static::lazy_static;

use crate::theme::catppuccin_frappe::*;

const ICON_FG: &str = PINK;
const ICON_BG: &str = CRUST;
const ICON_TR: &str = "0xff";
const TEXT_FG: &str = PINK;
const TEXT_BG: &str = CRUST;
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