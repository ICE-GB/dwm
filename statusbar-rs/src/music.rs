use lazy_static::lazy_static;
#[cfg(test)]
use regex::Regex;

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

    static ref TITLE:std::sync::Mutex<MusicTitle> = std::sync::Mutex::new(MusicTitle::new("".to_string()));
}
const NAME: &str = "music";


#[cfg(test)]
#[test]
pub fn test() {
    println!("get() = {:?}", get());
    let expected_regex = Regex::new(r"\^smusic\^\^c#ff79c6\^\^b#282a360xff\^ 󰝚.+").unwrap();
    assert!(expected_regex.is_match(&get().data));
}

pub fn get() -> PackageData {
    let playing = cmd("mpc status | grep playing | wc -l");
    let playing = playing == "1";
    let title = common::cmd("mpc current");
    if title.is_empty() {
        let text = format!("^s{}^{} {} ", NAME, *ICON_COLOR, "󰝚");
        return PackageData::new(NAME, text);
    }
    let mut title_s = TITLE.lock().unwrap();

    if title_s.title == title {
        let text = format!("^s{}^{} {}{} {}", NAME, *ICON_COLOR, "󰝚", *TEXT_COLOR, title_s.get_rolling_title(playing));
        return PackageData::new(NAME, text);
    }

    title_s.title = title;
    title_s.current_pos = 0;

    let text;

    text = format!("^s{}^{} {}{} {}", NAME, *ICON_COLOR, "󰝚", *TEXT_COLOR, title_s.get_rolling_title(playing));


    PackageData::new(NAME, text)
}

pub fn api(button: Button) {
    match button {
        Button::LEFT => { cmd("mpc -q toggle"); }
        Button::RIGHT => { cmd("xdotool keydown Super m keyup m Super"); }
        Button::MIDDLE => { cmd("mpc -q stop"); }
        Button::UP => { cmd("mpc -q prev"); }
        Button::DOWN => { cmd("mpc -q next"); }
    }
}

struct MusicTitle {
    title: String,
    current_pos: usize,
}

impl MusicTitle {
    fn new(title: String) -> Self {
        MusicTitle {
            title,
            current_pos: 0,
        }
    }

    fn get_rolling_title(&mut self, playing: bool) -> String {
        let title_len = self.title.chars().count();
        if title_len <= 20 {
            let rolling_title = &self.title;
            if playing { self.current_pos = (self.current_pos + 1) % title_len; }
            rolling_title.to_string()
        } else {
            let mut char_indices = self.title.char_indices();
            let start_index = char_indices.nth(self.current_pos).map(|(i, _)| i).unwrap_or(0);
            let end_index = char_indices.nth(19).map(|(i, _)| i).unwrap_or(self.title.len());
            let rolling_title = &self.title[start_index..end_index];
            if playing { self.current_pos = (self.current_pos + 1) % (title_len - 19); }
            rolling_title.to_string()
        }
    }
}