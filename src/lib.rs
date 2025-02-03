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
    pub key_buffer: [u8; 3],
    pub keys_pressed: String,
}

impl App {
    pub fn new(x: usize, y: usize) -> Self {
        App {
            key_buffer: [0; 3],
            keys_pressed: String::new(),
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

// Usage raw_line("q <- to quit");
pub fn raw_line(message: &str) {
    let mut stdout = io::stdout();
    stdout.write_all(format!("{message}\n").as_bytes()).unwrap();
    stdout.flush().unwrap();
}

use std::io::{self, Read, Write};

/// mainly not used, holds program till key press and does not save for other if statements
/// Usage
//if halt_press_check(&mut app, "q") {
///    clear();
///    break;
///}
pub fn halt_press_check(app: &mut App, key: &str) -> bool {
    let pressed: bool;

    let pressed_key = find_key_pressed(app);

    if pressed_key == key {
        // .eq_ignore_ascii_case(key) for removing case sensitivity.
        pressed = true;
    } else if pressed_key == "unknown" {
        pressed = false;
    } else {
        pressed = false;
    }

    app.key_buffer = [0; 3];
    pressed
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr) => {
        Position { x: $x, y: $y }
    };
}

/// Please refer to this for color names
pub fn color_code(color: &str) -> &'static str {
    match color {
        "red" => "\x1B[31m",
        "green" => "\x1B[32m",
        "yellow" => "\x1B[33m",
        "blue" => "\x1B[34m",
        "magenta" => "\x1B[35m",
        "cyan" => "\x1B[36m",
        "white" => "\x1B[37m",
        _ => "\x1B[0m", // incorrect name
    }
}

/// Used to make a text appear, a a position,
/// each position being a different sectioned &str on screen.
/// Usage
///line(Position { x: 0, y: 5 }, "First", "blue");
///line(Position { x: 0, y: 11 }, "Sec", "red");
pub fn line(position: Position, text: &str, color: &str) {
    let x = position.x;
    let y = position.y;
    let letter = text;
    let color_code = color_code(color);
    let reset_code = "\x1B[0m";
    print!("\x1B[{};{}H{}{}{}", x, y, color_code, letter, reset_code);
    io::stdout().flush().unwrap();
}

pub fn find_key_pressed(app: &mut App) -> &'static str {
    let bytes_read = io::stdin().read(&mut app.key_buffer).unwrap();
    let pressed_key: &str = match &app.key_buffer[..bytes_read] {
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
    pressed_key
}

/// not used, due to key trottling issues
/// Usage
///all_presses = format!("{}{}", all_presses, collected_key_presses(&mut app));
///println!("{}", all_presses);
pub fn collected_key_presses(app: &mut App) -> &'static str {
    find_key_pressed(app)
}

/// will still halt but collect one input for the whole loop, each loop being for one input
pub fn collect_presses(app: &mut App) {
    app.keys_pressed = find_key_pressed(app).to_string();
}

/// to use you must collect_presses(), before calling this method
/// refer to find_key_pressed() to see all key names for key &str
/// This is the main way to check for input.
/// to collect full input for typing you will need to make a loop within the loop.
/// otherwise everyother key will be missing from collect_presses() method.
pub fn key_press(app: &App, key: &str) -> bool {
    if app.keys_pressed == key.to_string() {
        true
    } else {
        false
    }
}

/// Same as key_press() method, but is not case sensitive.
pub fn key_press_not_case_sen(app: &App, key: &str) -> bool {
    if app.keys_pressed.eq_ignore_ascii_case(key) {
        true
    } else {
        false
    }
}
