use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::Duration;

/// Controls the virutal cursor, which is made my the framework, while the real is hidden.
#[derive(Debug, PartialEq)]
pub enum Virtualcursor {
    Position { pos: Pos },
    NotEnabled,
}

#[derive(Clone, Copy)]
pub enum ChangeColor {
    On { color: Color },
    No,
}

#[derive(Clone, Copy)]
pub enum ChangeCh {
    On { ch: char },
    No,
}

pub enum ChangeStyle {
    On { style: Style },
    No,
}

pub struct VirtualCursorTheme {
    pub style: ChangeStyle,
    pub fg_code: ChangeColor,
    pub bg_code: ChangeColor,
    pub ch: ChangeCh,
}

/// Controls if the text should be shown under another if outside the closure.
#[derive(Clone, Debug, PartialEq)]
pub enum LeadOnly {
    ShownLead { key: String },
    AlwaysShown,
}

#[derive(Debug)]
pub struct Letter {
    pub ch: char,
    pub fg_code: i8,
    pub bg_code: i8,
    pub style: i8,
    pub when: LeadOnly,
}

#[derive(Debug)]
pub struct LeadkeySequence {
    pub lead_key: String,
    pub following_sequence: Vec<String>,
}

/// Global app memory access point, where program data is stored, for various perposes.
pub struct App {
    pub key_buffer: [u8; 3],
    pub keypressed: String,
    pub enable_f_row_and_arrow: bool,
    pub unknown_not_asci_code: bool,
    pub virtual_cursor: Virtualcursor,
    pub virtual_cursor_theme: VirtualCursorTheme,
    pub letter_grid: Vec<Vec<Letter>>,
}

impl App {
    pub fn new() -> Self {
        App {
            key_buffer: [0; 3],
            keypressed: String::new(),
            enable_f_row_and_arrow: false,
            unknown_not_asci_code: false,
            virtual_cursor: Virtualcursor::Position { pos: pos!(0, 0) },
            virtual_cursor_theme: VirtualCursorTheme {
                style: ChangeStyle::No,
                fg_code: ChangeColor::No,
                bg_code: ChangeColor::No,
                ch: ChangeCh::No,
            },
            letter_grid: vec![],
        }
    }
}

/// For proper functioning please enable before app loop,
pub fn raw_mode(enabled: bool) {
    if enabled {
        //real_cursor_visibility(false);
        std::process::Command::new("stty")
            .arg("-echo")
            .arg("raw")
            .status()
            .unwrap();
    } else {
        //real_cursor_visibility(true);
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
        app.keypressed = find_key_pressed_f_row_and_arrow(app).to_string();
    } else {
        app.keypressed = find_key_pressed_no_special(app).to_string();
    }
}

#[deprecated(note = "Use `Pressed` instead. This will be removed in version 0.0.5.")]
pub fn key_press(app: &App, key: &str) -> bool {
    if app.keypressed == key.to_string() {
        true
    } else {
        false
    }
}

/// Same as key_press() method, but is not case sensitive.
#[deprecated(note = "Use `Pressed_letter` instead. This will be removed in version 0.0.5.")]
pub fn key_press_not_case_sen(app: &App, key: &str) -> bool {
    if app.keypressed.eq_ignore_ascii_case(key) {
        true
    } else {
        false
    }
}

/// Used for ideomatics.
pub struct Key {
    pub case_sen: bool,
    pub clear_non_lead_text: bool,
}

impl Key {
    /// Required to default optional functional para for case sensitivity, without an extra bool para in pressed.
    pub fn o() -> Self {
        Key {
            case_sen: false,
            clear_non_lead_text: true,
        }
    }

    /// in order to use you must `collect_presses()`, before calling this method
    /// refer to `find_key_pressed_no_special`, and `find_key_pressed_f_row_and_arrow`  to see all key names for key &str
    /// This is the main way to check for input.
    /// to collect full input for typing you will need to make a loop within the loop.
    /// otherwise everyother key will be missing from `collect_presses()` method.
    pub fn pressed(self, app: &mut App, key: &str) -> bool {
        if self.case_sen == true {
            if app.keypressed.eq_ignore_ascii_case(key) {
                if self.clear_non_lead_text == true {
                    clear_nonlead(app);
                }
                true
            } else {
                false
            }
        } else {
            if app.keypressed == key.to_string() {
                if self.clear_non_lead_text == true {
                    clear_nonlead(app);
                }
                true
            } else {
                false
            }
        }
    }

    /// Toggle the case sensitive optional functional para.
    pub fn case_sen(self, on: bool) -> Self {
        Key {
            case_sen: on,
            clear_non_lead_text: self.clear_non_lead_text,
        }
    }

    /// Toggle the clearing of so called non-lead text or vanishing text,
    /// which are texts specifed to be only shown under certain keys.
    pub fn no_clear(self) -> Self {
        Key {
            case_sen: self.case_sen,
            clear_non_lead_text: false,
        }
    }
}

/// Clears non-lead key letters if not AlwaysShown.
pub fn clear_nonlead(app: &mut App) {
    for row in 0..app.letter_grid.len() {
        for col in 0..app.letter_grid[row].len() {
            if app.letter_grid[row][col].when
                != (LeadOnly::ShownLead {
                    key: app.keypressed.clone(),
                })
                && app.letter_grid[row][col].when != LeadOnly::AlwaysShown
            {
                app.letter_grid[row][col] = Letter {
                    ch: ' ',
                    fg_code: 39,
                    bg_code: 49,
                    style: 0,
                    when: LeadOnly::AlwaysShown,
                }
            }
        }
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
    pub vanish: bool,
}

///Custom goes from 0-255:
/// 0-15 is standard bright colors;
/// 16-231 is custom colors from 6 levels of reds, greens, and blues;
/// 232-255 is a grayscale ramp, having 24 shades of gray from dark to light.
#[derive(Clone, Copy)]
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
            vanish: true,
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
    ///
    /// Usage:
    ///
    /// `Text::new()
    ///.foreground(Color::Green) // <- Extra optional changes.
    ///.background(Color::White) // </
    ///.show("Normal", pos!(0, 0));`
    ///
    /// Or, without the extra styling/para:
    ///
    ///`Text::new().show("Test", pos!(0, 1));`
    pub fn show(self, app: &mut App, text: &str, pos: Pos) {
        // cols
        while app.letter_grid.len() < pos.y + 2 {
            app.letter_grid.push(vec![]);
        }

        // rows
        while app.letter_grid[pos.y].len() < pos.x + 1 + text.len() {
            app.letter_grid[pos.y].push(Letter {
                ch: ' ',
                fg_code: 39,
                bg_code: 49,
                style: 0,
                when: LeadOnly::AlwaysShown,
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

        let when = if self.settings.vanish == true {
            LeadOnly::ShownLead {
                key: app.keypressed.to_string(),
            }
        } else {
            LeadOnly::AlwaysShown
        };

        for (i, ch) in text.chars().enumerate() {
            app.letter_grid[pos.y][pos.x + i] = Letter {
                ch,
                fg_code,
                bg_code,
                style: style_code,
                when: when.clone(),
            };
        }
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

    /// By default text will vanish that are under other inputs to prevent overlap.
    /// Set as false to keep your text safe!
    pub fn vanish(mut self, vanish: bool) -> Self {
        if vanish {
            self.settings.vanish = true;
        } else {
            self.settings.vanish = false;
        }
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

    if let Virtualcursor::Position { pos } = app.virtual_cursor {
        if pos.y < app.letter_grid.len() && pos.x < app.letter_grid[pos.y].len() {
            let cell = &app.letter_grid[pos.y][pos.x];

            let style: i8 = match app.virtual_cursor_theme.style {
                ChangeStyle::On {
                    style: Style::Reset,
                } => 0,
                ChangeStyle::On { style: Style::Bold } => 1,
                ChangeStyle::On {
                    style: Style::Italic,
                } => 3,
                ChangeStyle::On {
                    style: Style::Underline,
                } => 4,
                ChangeStyle::No => cell.style,
            };

            let fg_code: i8 = match app.virtual_cursor_theme.fg_code {
                ChangeColor::No => cell.fg_code,
                ChangeColor::On { color } => color_to_ansi_code(&color, false),
            };

            let bg_code: i8 = match app.virtual_cursor_theme.bg_code {
                ChangeColor::No => cell.bg_code,
                ChangeColor::On { color } => color_to_ansi_code(&color, false),
            };

            let ch: char = match app.virtual_cursor_theme.ch {
                ChangeCh::No => cell.ch,
                ChangeCh::On { ch } => ch,
            };

            print!(
                "\x1B[{};{}H\x1B[{};{};{}m{}{}",
                pos.y + 1,
                pos.x + 1,
                style,
                fg_code,
                bg_code,
                ch,
                reset_code
            );
        } else {
            print!(
                "\x1B[{};{}H\x1B[{};{};{}m{}{}",
                pos.y + 1,
                pos.x + 1,
                " ",
                39,
                49,
                " ",
                reset_code
            );
        }
    }

    io::stdout().flush().unwrap();
}

/// The virtual cursor offers a more interactive and fluid experience across most systems.
/// It is enabled by default.
pub fn toggle_virtual_cursor(app: &mut App, on: bool) {
    if on == true {
        app.virtual_cursor = Virtualcursor::Position { pos: pos!(0, 0) };
    } else {
        app.virtual_cursor = Virtualcursor::NotEnabled;
    }
}

/// Clears the screen.
/// All text will be pushed out of view.
/// The letter_grid property whill be reset.
pub fn clear(app: &mut App) {
    print!("\x1B[2J\x1B[1;1H");
    app.letter_grid = vec![];
}

/// move cursor directly towards a coordinate position.
pub fn mov_cur_to(app: &mut App, position: Pos) {
    if app.virtual_cursor != Virtualcursor::NotEnabled {
        app.virtual_cursor = Virtualcursor::Position { pos: position };
    }
}

/// Direction enum
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

/// Quick reminder: this is not an assembly instruction!
/// This a optinal setting shared by the cursor move pattern of functions.
/// Therefore it's a trait of this class, with implications of methods to move the curosr.
pub struct Mov {
    pub wrap: bool, // Warpping such as when you try to move the carvet off a line.
}

impl Mov {
    pub fn cur() -> Self {
        Mov { wrap: false }
    }

    /// Move Cursor up (units).
    pub fn up(self, app: &mut App, units: usize) {
        if app.virtual_cursor != Virtualcursor::NotEnabled {
            let last_pos: Pos = match app.virtual_cursor {
                Virtualcursor::NotEnabled => pos!(0, 0),
                Virtualcursor::Position { pos } => pos,
            };

            if last_pos.y != 0 {
                app.virtual_cursor = Virtualcursor::Position {
                    pos: pos!(last_pos.x, last_pos.y - units),
                };
            }
        }
    }

    /// Move Cursor down (units).
    pub fn down(self, app: &mut App, units: usize) {
        if app.virtual_cursor != Virtualcursor::NotEnabled {
            let last_pos: Pos = match app.virtual_cursor {
                Virtualcursor::NotEnabled => pos!(0, 0),
                Virtualcursor::Position { pos } => pos,
            };

            app.virtual_cursor = Virtualcursor::Position {
                pos: pos!(last_pos.x, last_pos.y + units),
            };
        }
    }

    /// Move Cursor left (units).
    pub fn left(self, app: &mut App, units: usize) {
        if app.virtual_cursor != Virtualcursor::NotEnabled {
            let last_pos: Pos = match app.virtual_cursor {
                Virtualcursor::NotEnabled => pos!(0, 0),
                Virtualcursor::Position { pos } => pos,
            };

            if last_pos.x > 0 {
                app.virtual_cursor = Virtualcursor::Position {
                    pos: pos!(last_pos.x - units, last_pos.y),
                };
            } else {
                app.virtual_cursor = Virtualcursor::Position {
                    pos: pos!(0, last_pos.y),
                };
            }
        }
    }

    /// Move Cursor right (units).
    pub fn right(self, app: &mut App, units: usize) {
        if app.virtual_cursor != Virtualcursor::NotEnabled {
            let last_pos: Pos = match app.virtual_cursor {
                Virtualcursor::NotEnabled => pos!(0, 0),
                Virtualcursor::Position { pos } => pos,
            };

            app.virtual_cursor = Virtualcursor::Position {
                pos: pos!(last_pos.x + units, last_pos.y),
            };
        }
    }

    /// Move cursor in a (direction) of (units).
    pub fn dir(self, app: &mut App, directon: Dir, units: usize) {
        if app.virtual_cursor != Virtualcursor::NotEnabled {
            match directon {
                Dir::Up => self.up(app, units),
                Dir::Down => self.down(app, units),
                Dir::Left => self.left(app, units),
                Dir::Right => self.right(app, units),
            }
        }
    }
}

/// Returns Pos (Position) of the virtual cursor.
pub fn get_cur_pos(app: &mut App) -> Pos {
    match app.virtual_cursor {
        Virtualcursor::NotEnabled => pos!(0, 0),
        Virtualcursor::Position { pos } => pos,
    }
}

/// Returns the key which is pressed under this iteration.
pub fn key_pressed(app: &App) -> String {
    app.keypressed.clone()
}
