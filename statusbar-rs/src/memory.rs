use std::sync::RwLock;

use lazy_static::lazy_static;
use regex::Regex;
use sysinfo::{System, SystemExt};

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
    static ref SYSTEM: RwLock<System> = RwLock::new(System::new_all());
    static ref DELAY_TIME: i32 = *common::packages_lists().get(NAME).unwrap();
}
const NAME: &str = "memory";

pub fn get() -> PackageData {
    // 获取内存占用率
    let mut system = SYSTEM.write().unwrap();
    system.refresh_memory();

    let mem_usage = (system.used_memory() as f64 / system.total_memory() as f64) * 100.0;
    let mem_usage = mem_usage as i32;

    if mem_usage > 90 {
        // kill_some_thing();
    }

    if mem_usage > 95 {
        // notify();
    }

    let text = format!("^s{}^{} {} {}{}% ", NAME, *ICON_COLOR, "", *TEXT_COLOR, mem_usage);


    PackageData::new(NAME, text)
}

pub fn api(button: Button) {
    match button {
        Button::LEFT => {}
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
    let expected_regex = Regex::new(r"\^smemory\^\^c#ff79c6\^\^b#282a360xff\^ . \^c#ff79c6\^\^b#282a360xff\^.+% ").unwrap();
    assert!(expected_regex.is_match(&get().data));
}