use lazy_static::lazy_static;
use regex::Regex;

use crate::common;
use crate::common::PackageData;

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
const NAME: &str = "icon";

pub fn get() -> PackageData {
    let text = format!("^s{}^{} {} {}{} ", NAME, *ICON_COLOR, "", *TEXT_COLOR, "");


    PackageData::new(NAME, text)
}

#[cfg(test)]
#[test]
pub fn test() {
    println!("get() = {:?}", get());
    let expected_regex = Regex::new(r"\^sicon\^\^c#ff79c6\^\^b#282a360xff\^  \^c#ff79c6\^\^b#282a360xff\^").unwrap();
    assert!(expected_regex.is_match(&get().data));
}