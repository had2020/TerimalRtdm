use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::Duration;

/// Clears the screen.
/// All text will be pushed out of view.
/// The letter_grid property whill be reset.
pub fn clear(app: &mut App) {
    print!("\x1B[2J\x1B[1;1H");
    app.letter_grid = vec![];
}

pub fn move_cursor(app: &mut App, position: Pos) {
    app.virtual_cursor = Virtualcursor::Position { pos: position };
}

#[derive(Debug)]
pub enum Virtualcursor {
    Position { pos: Pos },
    NotEnabled,
}

#[derive(Debug)]
pub struct Letter {
    pub ch: char,
    pub fg_code: i8,
    pub bg_code: i8,
    pub style: i8,
}

#[derive(Debug)]
pub struct App {
    pub key_buffer: [u8; 3],
    pub keys_pressed: String,
    pub enable_f_row_and_arrow: bool,
    pub unknown_not_asci_code: bool,
    pub virtual_cursor: Virtualcursor,
    pub letter_grid: Vec<Vec<Letter>>,
}

impl App {
    pub fn new() -> Self {
        App {
            key_buffer: [0; 3],
            keys_pressed: String::new(),
            enable_f_row_and_arrow: false,
            unknown_not_asci_code: false,
            virtual_cursor: Virtualcursor::Position { pos: pos!(0, 0) },
            letter_grid: vec![],
        }
    }
}

/// For proper functioning please enable before app loop,
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
//`if halt_press_check(&mut app, "q") {
///    clear();
///    break;
///}`
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

/// DEPRECATED! use Pos
#[deprecated(note = "Use `Pos` instead. This will be removed in version 0.0.5.")]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// For idiomatically storing postion vector
#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

/// DEPRECATED! use Pos Marco
#[deprecated(note = "Use `pos!()` instead. This will be removed in version 0.0.5.")]
#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr) => {
        Position { x: $x, y: $y }
    };
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Pos { x: $x, y: $y }
    };
}

/// DEPRECATED! use enum "color", this will still work for the old "line" method
#[deprecated(note = "Use `color` instead. This will be removed in version 0.0.5.")]
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

/// Useage:
/// `Text::new()
///.foreground(Color::Green)
///.background(Color::White)
///.show("Normal", pos!(0, 0));
pub struct Text {
    pub settings: TextColorOption,
}

/// "fore" corresponds to foreground, and "back" to background accordingly.
pub struct TextColorOption {
    pub fore: Color,
    pub back: Color,
    pub style: Style,
}

///Custom goes from 0-255:
/// 0-15 is standard bright colors;
/// 16-231 is custom colors from 6 levels of reds, greens, and blues;
/// 232-255 is a grayscale ramp, having 24 shades of gray from dark to light.
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Default { back: bool },
    Custom { id: i8 },
}

impl Default for TextColorOption {
    fn default() -> Self {
        Self {
            fore: Color::White,
            back: Color::Default { back: true },
            style: Style::Reset,
        }
    }
}

/// Converts the Color enum into appropriate codes for fore and back grounds
pub fn color_to_ansi_code(color: &Color, is_bg: bool) -> i8 {
    match color {
        Color::Red => {
            if is_bg {
                41
            } else {
                31
            }
        }
        Color::Green => {
            if is_bg {
                42
            } else {
                32
            }
        }
        Color::Blue => {
            if is_bg {
                44
            } else {
                34
            }
        }
        Color::Yellow => {
            if is_bg {
                43
            } else {
                33
            }
        }
        Color::Magenta => {
            if is_bg {
                45
            } else {
                35
            }
        }
        Color::Cyan => {
            if is_bg {
                46
            } else {
                36
            }
        }
        Color::White => {
            if is_bg {
                47
            } else {
                37
            }
        }
        Color::Default { .. } => {
            if is_bg {
                49
            } else {
                39
            }
        }
        Color::Custom { id } => *id,
    }
}

pub enum Style {
    Reset,
    Bold,
    Italic,
    Underline,
}

impl Text {
    pub fn new() -> Self {
        Text {
            settings: TextColorOption::default(),
        }
    }

    /// Used to make a text appear, a a position,
    /// each position being a different sectioned &str on screen.
    /// Usage:
    /// `Text::new()
    ///.foreground(Color::Green) // <- Extra optional changes.
    ///.background(Color::White) // </
    ///.show("Normal", pos!(0, 0));`
    /// Or:
    ///`Text::new().show("Test", pos!(0, 1));`
    pub fn show(self, app: &mut App, text: &str, pos: Pos) {
        // cols
        while app.letter_grid.len() < pos.y + 1 {
            app.letter_grid.push(vec![]);
        }

        // rows
        while app.letter_grid[pos.y].len() < pos.x + 1 + text.len() {
            app.letter_grid[pos.y].push(Letter {
                ch: ' ',
                fg_code: 39,
                bg_code: 49,
                style: 0,
            });
        }

        let fg_code = color_to_ansi_code(&self.settings.fore, false);
        let bg_code = color_to_ansi_code(&self.settings.back, true);

        let style_code: i8 = match self.settings.style {
            Style::Reset => 0,
            Style::Bold => 1,
            Style::Italic => 3,
            Style::Underline => 4,
        };

        for (i, ch) in text.chars().enumerate() {
            app.letter_grid[pos.y][pos.x + i] = (Letter {
                ch: ch,
                fg_code: fg_code,
                bg_code: bg_code,
                style: style_code,
            });
        }

        //let reset_code = "\x1B[0m";

        /*
        print!(
            "\x1B[{};{}H\x1B[{};{};{}m{}{}",
            pos.y + 1,
            pos.x + 1,
            style_code,
            fg_code,
            bg_code,
            text,
            reset_code
        );
        io::stdout().flush().unwrap();
        */
    }

    /// Set the text/font color.
    pub fn foreground(mut self, color: Color) -> Self {
        self.settings.fore = color;
        self
    }

    /// Set the background color.
    pub fn background(mut self, color: Color) -> Self {
        self.settings.back = color;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.settings.style = style;
        self
    }
}

/// DEPRECATED! Use Text struct with set impl
/// Used to make a text appear, a a position,
/// each position being a different sectioned &str on screen.
/// Usage
///line(Position { x: 0, y: 5 }, "First", "blue");
///line(Position { x: 0, y: 11 }, "Sec", "red");
#[deprecated(note = "Use `Text::show()` instead. This will be removed in version 0.0.5.")]
pub fn line(position: Position, text: &str, color: &str) {
    let x = position.x;
    let y = position.y;
    let letter = text;
    let color_code = color_code(color);
    let reset_code = "\x1B[0m";
    print!("\x1B[{};{}H{}{}{}", x, y, color_code, letter, reset_code);
    io::stdout().flush().unwrap();
}

/// Toggle line wrapping in the terimal.
pub fn line_wrapping(enable: bool) {
    if enable {
        println!("\x1b[?7h");
    } else {
        println!("\x1b[?7l");
    }
    io::stdout().flush().unwrap();
}

pub fn real_cursor_up(x: usize) {
    print!("\x1B[{}A", x);
    io::stdout().flush().unwrap();
}

pub fn real_cursor_down(x: usize) {
    print!("\x1B[{}B", x);
    io::stdout().flush().unwrap();
}

pub fn real_cursor_left(x: usize) {
    print!("\x1B[{}C", x);
    io::stdout().flush().unwrap();
}

pub fn real_cursor_right(x: usize) {
    print!("\x1B[{}D", x);
    io::stdout().flush().unwrap();
}

pub fn real_cursor_move(position: Pos) {
    print!("\x1B[{};{}H", position.x, position.y);
    io::stdout().flush().unwrap();
}

pub fn real_cursor_visibility(visable: bool) {
    if visable == true {
        print!("\x1B[?25h");
    } else {
        print!("\x1B[?25l");
    }
    io::stdout().flush().unwrap();
}

pub fn refresh() {
    io::stdout().flush().unwrap();
}

/// draws/prints the stored data within letter_grid of app based on all changes.
pub fn render(app: &App) {
    let reset_code = "\x1B[0m";

    for row in 0..app.letter_grid.len() {
        for col in 0..app.letter_grid[row].len() {
            print!(
                "\x1B[{};{}H\x1B[{};{};{}m{}{}",
                row + 1,
                col + 1,
                app.letter_grid[row][col].style,
                app.letter_grid[row][col].fg_code,
                app.letter_grid[row][col].bg_code,
                app.letter_grid[row][col].ch,
                reset_code
            );
        }
    }

    io::stdout().flush().unwrap();
}
