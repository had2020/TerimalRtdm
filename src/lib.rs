use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::Duration;

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
    pub enable_f_row_and_arrow: bool,
    pub unknown_not_asci_code: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            key_buffer: [0; 3],
            keys_pressed: String::new(),
            enable_f_row_and_arrow: false, //TODO demo case in docs
            unknown_not_asci_code: false,  //TODO
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

/// mainly not used, holds program till key press and does not save for other if statements
/// Usage
//if halt_press_check(&mut app, "q") {
///    clear();
///    break;
///}
pub fn halt_press_check(app: &mut App, key: &str) -> bool {
    let pressed: bool;
    let pressed_key: String;

    if app.enable_f_row_and_arrow == true {
        pressed_key = find_key_pressed_f_row_and_arrow(app).to_string();
    } else {
        pressed_key = find_key_pressed_no_special(app).to_string();
    }

    if pressed_key == key {
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

/// Returns the code of a extra key that is pressed.
/// The app loop will take on input for the cycle, and then
/// another being one linearly.
/// So you will need to press the same key twice and then it will return, enabling the process to continue.
/// Use this function to debug for keys not included in the find_key_pressed method.
pub fn debug_code_pressed(app: &mut App) -> u8 {
    io::stdin().read(&mut app.key_buffer).unwrap();
    app.key_buffer[0]
}

pub fn find_key_pressed_f_row_and_arrow(app: &mut App) -> &'static str {
    let mut key_buffer = [0u8; 3];
    let mut total_read = 0;
    let stdin = io::stdin();

    let read_now = stdin.lock().read(&mut key_buffer[total_read..]).unwrap();
    total_read += read_now;

    if key_buffer[..total_read] == [27] {
        for _ in 0..2 {
            sleep(Duration::from_millis(30));
            if let Ok(n) = stdin.lock().read(&mut key_buffer[total_read..]) {
                if n == 0 {
                    break;
                }
                total_read += n;
            } else {
                break;
            }
        }
    }

    //println!("Final key_buffer: {:?}", &key_buffer[..total_read]); // Debug

    let pressed_key: &str = match &key_buffer[..total_read] {
        // escape sequences
        [27, 27, 27] => "Esc", // Note: you have to press esc threee times, due to design.

        // function keys
        [27, 79, 80] => "F1",
        [27, 79, 81] => "F2",
        [27, 79, 82] => "F3",
        [27, 79, 83] => "F4",
        [27, 91, 49, 53, 126] => "F5",
        [27, 91, 49, 55, 126] => "F6",
        [27, 91, 49, 56, 126] => "F7",
        [27, 91, 49, 57, 126] => "F8",
        [27, 91, 50, 48, 126] => "F9",
        [27, 91, 50, 49, 126] => "F10",
        [27, 91, 50, 51, 126] => "F11",
        [27, 91, 50, 52, 126] => "F12",

        // arrow keys
        [27, 91, 65] => "Up",
        [27, 91, 66] => "Down",
        [27, 91, 67] => "Right",
        [27, 91, 68] => "Left",

        // lowercase letter keys
        [97] => "a",
        [98] => "b",
        [99] => "c",
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
        [13] => "Enter",
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

pub fn find_key_pressed_no_special(app: &mut App) -> &'static str {
    let bytes_read = io::stdin().read(&mut app.key_buffer).unwrap();

    let pressed_key: &str = match &app.key_buffer[..bytes_read] {
        // escape sequences
        [27] => "Esc",

        // lowercase letter keys
        [97] => "a",
        [98] => "b",
        [99] => "c",
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
        [13] => "Enter",
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

/// will still halt but collect one input for the whole loop, each loop being for one input
/// Set app.enable_f_row_and_arrow = true, if you wish for all function keys.
pub fn collect_presses(app: &mut App) {
    if app.enable_f_row_and_arrow == true {
        app.keys_pressed = find_key_pressed_f_row_and_arrow(app).to_string();
    } else {
        app.keys_pressed = find_key_pressed_no_special(app).to_string();
    }
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

/// Move to cursor to a position
pub fn move_cursor(position: Position) {
    println!("\033[{};{}H", position.x, position.y);
}
