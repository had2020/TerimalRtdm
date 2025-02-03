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

pub struct Window {
    x: usize,
    z: usize,
}

pub struct App {
    pub key_buffer: [u8; 3],
    pub multi_line: Vec<Vec<String>>,
}

impl App {
    pub fn new(window: Window) -> Self {
        let mut intial_screen: Vec<Vec<String>> = vec![];
        for row in 0..window.x {}

        App {
            key_buffer: [0; 3],
            multi_line: vec![
                vec![String::new(), String::new()],
                vec![String::new(), String::new()],
            ],
        }
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

pub fn key_pressed(app: &mut App, key: &str) -> bool {
    let bytes_read = io::stdin().read(&mut app.key_buffer).unwrap();
    let pressed: bool;

    let pressed_key = match &app.key_buffer[..bytes_read] {
        // escape sequences
        [27, 91, 27] => "Esc",

        // function keys
        [27, 79, 80] => "F1",
        [27, 79, 81] => "F2",
        [27, 79, 82] => "F3",
        [27, 79, 83] => "F4",

        // arrow keys
        [27, 91, 65] => "Up",
        [27, 91, 66] => "Down",
        [27, 91, 67] => "Right",
        [27, 91, 68] => "Left",

        // lowercase letter keys
        [97] => "a",
        [98] => "b",
        [99] => "C",
        [100] => "d",
        [101] => "e",
        [102] => "f",
        [103] => "g",
        [104] => "h",
        [105] => "i",
        [106] => "j",
        [107] => "k",
        [108] => "l",
        [109] => "m",
        [110] => "n",
        [111] => "o",
        [112] => "p",
        [113] => "q",
        [114] => "r",
        [115] => "s",
        [116] => "t",
        [117] => "u",
        [118] => "v",
        [119] => "w",
        [120] => "x",
        [121] => "y",
        [122] => "z",

        // uppercase letter keys
        [65] => "A",
        [66] => "B",
        [67] => "C",
        [68] => "D",
        [69] => "E",
        [70] => "F",
        [71] => "G",
        [72] => "H",
        [73] => "I",
        [74] => "J",
        [75] => "K",
        [76] => "L",
        [77] => "M",
        [78] => "N",
        [79] => "O",
        [80] => "P",
        [81] => "Q",
        [82] => "R",
        [83] => "S",
        [84] => "T",
        [85] => "U",
        [86] => "V",
        [87] => "W",
        [88] => "X",
        [89] => "Y",
        [90] => "Z",

        // numbers
        [48] => "0",
        [49] => "1",
        [50] => "2",
        [51] => "3",
        [52] => "4",
        [53] => "5",
        [54] => "6",
        [55] => "7",
        [56] => "8",
        [57] => "9",

        // special characters
        [32] => "Space",
        [9] => "Tab",
        [10] => "Enter",
        [127] => "Backspace",
        [33] => "!",
        [34] => "\"",
        [35] => "#",
        [36] => "$",
        [37] => "%",
        [38] => "&",
        [39] => "'",
        [40] => "(",
        [41] => ")",
        [42] => "*",
        [43] => "+",
        [44] => ",",
        [45] => "-",
        [46] => ".",
        [47] => "/",
        [58] => ":",
        [59] => ";",
        [60] => "<",
        [61] => "=",
        [62] => ">",
        [63] => "?",
        [64] => "@",
        [91] => "[",
        [92] => "\\",
        [93] => "]",
        [94] => "^",
        [95] => "_",
        [96] => "`",
        [123] => "{",
        [124] => "|",
        [125] => "}",
        [126] => "~",

        // fail case
        _ => "unknown",
    };

    if pressed_key == key {
        // .eq_ignore_ascii_case(key)
        pressed = true;
    } else if pressed_key == "unknown" {
        //println!("Other key: {:?}", &app.buffer[..bytes_read]); // add key if missing
        pressed = false;
    } else {
        pressed = false;
        //println!("Not correct key") // used to check if key is added
    }

    app.key_buffer = [0; 3];
    pressed
}
