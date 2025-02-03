pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn cursor_state(enabled: bool) {
    if enabled {
        println!("\x1B[?25h");
    } else {
        println!("\x1B[?25l");
    }
}

pub struct App {
    pub buffer: [u8; 3],
}

impl App {
    pub fn new() -> Self {
        App { buffer: [0; 3] }
    }
}

pub fn raw_mode(enabled: bool) {
    if enabled {
        std::process::Command::new("stty")
            .arg("-echo")
            .arg("raw")
            .status()
            .unwrap();
    } else {
        std::process::Command::new("stty")
            .arg("echo")
            .arg("-raw")
            .status()
            .unwrap();
    }
}

use std::io::{self, Read, Write};

pub fn line(message: &str) {
    let mut stdout = io::stdout();
    stdout.write_all(format!("{message}\n").as_bytes()).unwrap();
    stdout.flush().unwrap();
}

/*
use std::collections::HashMap;

pub fn keymapping() -> HashMap<&'static str, [i32; 3]> {
    let mut key_mappings = HashMap::new();
    key_mappings.insert("Up", [27, 91, 65]);
    key_mappings.insert("Down", [27, 91, 66]);
    key_mappings.insert("Right", [27, 91, 67]);
    key_mappings.insert("Left", [27, 91, 68]);
    key_mappings
}
*/

pub fn key_pressed(app: &mut App, key: &str) -> bool {
    let bytes_read = io::stdin().read(&mut app.buffer).unwrap();
    let pressed: bool;

    let pressed_key = match &app.buffer[..bytes_read] {
        [27, 91, 65] => "Up",
        [27, 91, 66] => "Down",
        [27, 91, 67] => "Right",
        [27, 91, 68] => "Left",
        _ => "unknown",
    };

    if pressed_key.eq_ignore_ascii_case(key) {
        pressed = true;
    } else if pressed_key == "unknown" {
        println!("Other key: {:?}", &app.buffer[..bytes_read]); // add key if missing
        pressed = false;
    } else {
        pressed = false;
    }

    app.buffer = [0; 3];
    pressed
}
