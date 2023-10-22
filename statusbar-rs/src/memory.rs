#[cfg(test)]
use regex::Regex;

use crate::common;
use crate::common::{Button, ICON_COLOR, PackageData, TEXT_COLOR};

const NAME: &str = "memory";

pub fn get() -> PackageData {
    let memory = psutil::memory::virtual_memory().unwrap();
    let percent_used = memory.percent();
    // println!("Memory usage: {:.2}%", percent_used);
    let mem_usage = percent_used as i32;

    if mem_usage > 90 {
        common::cmd("pkill -f barrier");
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
