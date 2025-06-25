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
    pub keypressed: KeyType,
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
            keypressed: KeyType::Unknown,
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
//`if halt_press_check(&mut app, KeyType::q) {
///    clear();
///    break;
///}`
pub fn halt_press_check(app: &mut App, key: KeyType) -> bool {
    let pressed: bool;
    let pressed_key: KeyType;

    if app.enable_f_row_and_arrow == true {
        pressed_key = find_key_pressed_f_row_and_arrow(/*app*/);
    } else {
        pressed_key = find_key_pressed_no_special(app);
    }

    if pressed_key == key {
        pressed = true;
    } else if pressed_key == KeyType::Unknown {
        pressed = false;
    } else {
        pressed = false;
    }

    app.key_buffer = [0; 3];
    pressed
}

/// For idiomatically storing postion vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Pos { x: $x, y: $y }
    };
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

/// Refer to this enum for keys working under `pressed()` method.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum KeyType {
    // Escape Keys
    Esc,
    // Note: F row requires
    // Set app.enable_f_row_and_arrow = true, if you wish for all function keys.
    // This will come at the cost of halting till Esc is presed three times.
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Arrow Keys
    UpArrow,
    DownArrow,
    RightArrow,
    LeftArrow,

    // Letter Keys
    a,
    A,
    b,
    B,
    c,
    C,
    d,
    D,
    e,
    E,
    f,
    F,
    g,
    G,
    h,
    H,
    i,
    I,
    j,
    J,
    k,
    K,
    l,
    L,
    m,
    M,
    n,
    N,
    o,
    O,
    p,
    P,
    q,
    Q,
    r,
    R,
    s,
    S,
    t,
    T,
    u,
    U,
    v,
    V,
    w,
    W,
    x,
    X,
    y,
    Y,
    z,
    Z,

    // Numbers
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    // Special characters
    Space,
    Tab,
    Enter,
    Backspace,
    ExclamationMark, // !
    Quote,           // "
    Hash,            // #
    Dollar,          // $
    Percent,         // %
    Ampersand,       // &
    Apostrophe,      // '
    LeftParen,       // (
    RightParen,      // )
    Asterisk,        // *
    Plus,            // +
    Comma,           // ,
    Minus,           // -
    Dot,             // .
    Slash,           // /
    Colon,           // :
    Semicolon,       // ;
    LessThan,        // <
    Equal,           // =
    GreaterThan,     // >
    QuestionMark,    // ?
    At,              // @
    LeftBracket,     // [
    Backslash,       // \
    RightBracket,    // ]
    Caret,           // ^
    Underscore,      // _
    Backtick,        // `
    LeftBrace,       // {
    Pipe,            // |
    RightBrace,      // }
    Tilde,           // ~

    Unknown, // Missing key on the framework end.
    Null, // A null key press in the app variable means the collect_presses method as not been called.
}

pub fn find_key_pressed_f_row_and_arrow(/*app: &App*/) -> KeyType {
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

    let pressed_key: KeyType = match &key_buffer[..total_read] {
        // function keys
        [27, 79, 80] => KeyType::F1,
        [27, 79, 81] => KeyType::F2,
        [27, 79, 82] => KeyType::F3,
        [27, 79, 83] => KeyType::F4,
        [27, 91, 49, 53, 126] => KeyType::F5,
        [27, 91, 49, 55, 126] => KeyType::F6,
        [27, 91, 49, 56, 126] => KeyType::F7,
        [27, 91, 49, 57, 126] => KeyType::F8,
        [27, 91, 50, 48, 126] => KeyType::F9,
        [27, 91, 50, 49, 126] => KeyType::F10,
        [27, 91, 50, 51, 126] => KeyType::F11,
        [27, 91, 50, 52, 126] => KeyType::F12,
        [27, 91, 65] => KeyType::UpArrow,
        [27, 91, 66] => KeyType::DownArrow,
        [27, 91, 67] => KeyType::RightArrow,
        [27, 91, 68] => KeyType::LeftArrow,

        // escape sequences
        [27, 27, 27] => KeyType::Esc, // Note: you have to press esc threee times, due to design.

        // lowercase letter keys
        [97] => KeyType::a,
        [98] => KeyType::b,
        [99] => KeyType::c,
        [100] => KeyType::d,
        [101] => KeyType::e,
        [102] => KeyType::f,
        [103] => KeyType::g,
        [104] => KeyType::h,
        [105] => KeyType::i,
        [106] => KeyType::j,
        [107] => KeyType::k,
        [108] => KeyType::l,
        [109] => KeyType::m,
        [110] => KeyType::n,
        [111] => KeyType::o,
        [112] => KeyType::p,
        [113] => KeyType::q,
        [114] => KeyType::r,
        [115] => KeyType::s,
        [116] => KeyType::t,
        [117] => KeyType::u,
        [118] => KeyType::v,
        [119] => KeyType::w,
        [120] => KeyType::x,
        [121] => KeyType::y,
        [122] => KeyType::z,

        // uppercase letter keys
        [65] => KeyType::A,
        [66] => KeyType::B,
        [67] => KeyType::C,
        [68] => KeyType::D,
        [69] => KeyType::E,
        [70] => KeyType::F,
        [71] => KeyType::G,
        [72] => KeyType::H,
        [73] => KeyType::I,
        [74] => KeyType::J,
        [75] => KeyType::K,
        [76] => KeyType::L,
        [77] => KeyType::M,
        [78] => KeyType::N,
        [79] => KeyType::O,
        [80] => KeyType::P,
        [81] => KeyType::Q,
        [82] => KeyType::R,
        [83] => KeyType::S,
        [84] => KeyType::T,
        [85] => KeyType::U,
        [86] => KeyType::V,
        [87] => KeyType::W,
        [88] => KeyType::X,
        [89] => KeyType::Y,
        [90] => KeyType::Z,

        // numbers
        [48] => KeyType::Zero,
        [49] => KeyType::One,
        [50] => KeyType::Two,
        [51] => KeyType::Three,
        [52] => KeyType::Four,
        [53] => KeyType::Five,
        [54] => KeyType::Six,
        [55] => KeyType::Seven,
        [56] => KeyType::Eight,
        [57] => KeyType::Nine,

        // special characters
        [32] => KeyType::Space,
        [9] => KeyType::Tab,
        [10] => KeyType::Enter,
        [13] => KeyType::Enter,
        [127] => KeyType::Backspace,
        [33] => KeyType::ExclamationMark,
        [34] => KeyType::Quote,
        [35] => KeyType::Hash,
        [36] => KeyType::Dollar,
        [37] => KeyType::Percent,
        [38] => KeyType::Ampersand,
        [39] => KeyType::Apostrophe,
        [40] => KeyType::LeftParen,
        [41] => KeyType::RightParen,
        [42] => KeyType::Asterisk,
        [43] => KeyType::Plus,
        [44] => KeyType::Comma,
        [45] => KeyType::Minus,
        [46] => KeyType::Dot,
        [47] => KeyType::Slash,
        [58] => KeyType::Colon,
        [59] => KeyType::Semicolon,
        [60] => KeyType::LessThan,
        [61] => KeyType::Equal,
        [62] => KeyType::GreaterThan,
        [63] => KeyType::QuestionMark,
        [64] => KeyType::At,
        [91] => KeyType::LeftBracket,
        [92] => KeyType::Backslash,
        [93] => KeyType::RightBracket,
        [94] => KeyType::Caret,
        [95] => KeyType::Underscore,
        [96] => KeyType::Backtick,
        [123] => KeyType::LeftBrace,
        [124] => KeyType::Pipe,
        [125] => KeyType::RightBrace,
        [126] => KeyType::Tilde,

        // fail case
        _ => KeyType::Unknown,
    };
    pressed_key
}

pub fn find_key_pressed_no_special(app: &mut App) -> KeyType {
    let bytes_read = io::stdin().read(&mut app.key_buffer).unwrap();

    let pressed_key: KeyType = match &app.key_buffer[..bytes_read] {
        // escape sequences
        [27] => KeyType::Esc,

        // lowercase letter keys
        [97] => KeyType::a,
        [98] => KeyType::b,
        [99] => KeyType::c,
        [100] => KeyType::d,
        [101] => KeyType::e,
        [102] => KeyType::f,
        [103] => KeyType::g,
        [104] => KeyType::h,
        [105] => KeyType::i,
        [106] => KeyType::j,
        [107] => KeyType::k,
        [108] => KeyType::l,
        [109] => KeyType::m,
        [110] => KeyType::n,
        [111] => KeyType::o,
        [112] => KeyType::p,
        [113] => KeyType::q,
        [114] => KeyType::r,
        [115] => KeyType::s,
        [116] => KeyType::t,
        [117] => KeyType::u,
        [118] => KeyType::v,
        [119] => KeyType::w,
        [120] => KeyType::x,
        [121] => KeyType::y,
        [122] => KeyType::z,

        // uppercase letter keys
        [65] => KeyType::A,
        [66] => KeyType::B,
        [67] => KeyType::C,
        [68] => KeyType::D,
        [69] => KeyType::E,
        [70] => KeyType::F,
        [71] => KeyType::G,
        [72] => KeyType::H,
        [73] => KeyType::I,
        [74] => KeyType::J,
        [75] => KeyType::K,
        [76] => KeyType::L,
        [77] => KeyType::M,
        [78] => KeyType::N,
        [79] => KeyType::O,
        [80] => KeyType::P,
        [81] => KeyType::Q,
        [82] => KeyType::R,
        [83] => KeyType::S,
        [84] => KeyType::T,
        [85] => KeyType::U,
        [86] => KeyType::V,
        [87] => KeyType::W,
        [88] => KeyType::X,
        [89] => KeyType::Y,
        [90] => KeyType::Z,

        // numbers
        [48] => KeyType::Zero,
        [49] => KeyType::One,
        [50] => KeyType::Two,
        [51] => KeyType::Three,
        [52] => KeyType::Four,
        [53] => KeyType::Five,
        [54] => KeyType::Six,
        [55] => KeyType::Seven,
        [56] => KeyType::Eight,
        [57] => KeyType::Nine,

        // special characters
        [32] => KeyType::Space,
        [9] => KeyType::Tab,
        [10] => KeyType::Enter,
        [13] => KeyType::Enter,
        [127] => KeyType::Backspace,
        [33] => KeyType::ExclamationMark,
        [34] => KeyType::Quote,
        [35] => KeyType::Hash,
        [36] => KeyType::Dollar,
        [37] => KeyType::Percent,
        [38] => KeyType::Ampersand,
        [39] => KeyType::Apostrophe,
        [40] => KeyType::LeftParen,
        [41] => KeyType::RightParen,
        [42] => KeyType::Asterisk,
        [43] => KeyType::Plus,
        [44] => KeyType::Comma,
        [45] => KeyType::Minus,
        [46] => KeyType::Dot,
        [47] => KeyType::Slash,
        [58] => KeyType::Colon,
        [59] => KeyType::Semicolon,
        [60] => KeyType::LessThan,
        [61] => KeyType::Equal,
        [62] => KeyType::GreaterThan,
        [63] => KeyType::QuestionMark,
        [64] => KeyType::At,
        [91] => KeyType::LeftBracket,
        [92] => KeyType::Backslash,
        [93] => KeyType::RightBracket,
        [94] => KeyType::Caret,
        [95] => KeyType::Underscore,
        [96] => KeyType::Backtick,
        [123] => KeyType::LeftBrace,
        [124] => KeyType::Pipe,
        [125] => KeyType::RightBrace,
        [126] => KeyType::Tilde,

        // fail case
        _ => KeyType::Unknown,
    };
    pressed_key
}

/// will still halt but collect one input for the whole loop, each loop being for one input
/// Set app.enable_f_row_and_arrow = true, if you wish for all function keys.
pub fn collect_presses(app: &mut App) {
    if app.enable_f_row_and_arrow == true {
        app.keypressed = find_key_pressed_f_row_and_arrow(/*app*/);
    } else {
        app.keypressed = find_key_pressed_no_special(app);
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
    pub fn pressed(self, app: &mut App, key: KeyType) -> bool {
        if self.case_sen == false {
            if format!("{:?}", app.keypressed).eq_ignore_ascii_case(&format!("{:?}", key)) {
                if self.clear_non_lead_text == true {
                    clear_nonlead(app);
                }
                true
            } else {
                false
            }
        } else {
            if app.keypressed == key {
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
    /// On by default.
    pub fn case_sen(self, state: bool) -> Self {
        Key {
            case_sen: state,
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
                    key: format!("{:?}", &app.keypressed.clone()),
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Default { back: bool },
    Custom { id: i8 }, // Max is 255
}

/// Converts the Color enum into appropriate codes for fore and back grounds
pub fn color_to_ansi_code(color: &Color, is_bg: bool) -> i8 {
    match color {
        Color::Black => {
            if is_bg {
                40
            } else {
                30
            }
        }
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
        Color::Yellow => {
            if is_bg {
                43
            } else {
                33
            }
        }
        Color::Blue => {
            if is_bg {
                44
            } else {
                34
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

        Color::BrightBlack => {
            if is_bg {
                100
            } else {
                90
            }
        }
        Color::BrightRed => {
            if is_bg {
                101
            } else {
                91
            }
        }
        Color::BrightGreen => {
            if is_bg {
                102
            } else {
                92
            }
        }
        Color::BrightYellow => {
            if is_bg {
                103
            } else {
                93
            }
        }
        Color::BrightBlue => {
            if is_bg {
                104
            } else {
                94
            }
        }
        Color::BrightMagenta => {
            if is_bg {
                105
            } else {
                95
            }
        }
        Color::BrightCyan => {
            if is_bg {
                106
            } else {
                96
            }
        }
        Color::BrightWhite => {
            if is_bg {
                107
            } else {
                97
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
                key: format!("{:?}", app.keypressed),
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

#[derive(PartialEq)]
pub enum CurMovType {
    Wrap,
    Freefloat,
    Block,
}

/// Quick reminder: this is not an assembly instruction!
/// This a optinal setting shared by the cursor move pattern of functions.
/// Therefore it's a trait of this class, with implications of methods to move the curosr.
pub struct Mov {
    pub cursor_movement_bound_type: CurMovType, // Warpping such as when you try to move the carvet off a line.
}

impl Mov {
    /// Sets up `Mov` struct with text editor like cursor jumping, when at end of line at a false condition.
    pub fn cur() -> Self {
        Mov {
            cursor_movement_bound_type: CurMovType::Wrap,
        }
    }

    /// Move Cursor up (units).
    pub fn up(self, app: &mut App, units: usize) {
        if app.virtual_cursor != Virtualcursor::NotEnabled {
            let last_pos: Pos = match app.virtual_cursor {
                Virtualcursor::NotEnabled => pos!(0, 0),
                Virtualcursor::Position { pos } => pos,
            };

            if app.letter_grid.len() > 0 {
                if last_pos.y != 0 {
                    app.virtual_cursor = Virtualcursor::Position {
                        pos: pos!(last_pos.x, last_pos.y - units),
                    };
                }
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

            if app.letter_grid.len() > 0 {
                if app.letter_grid[last_pos.y].len() > last_pos.y {
                    app.virtual_cursor = Virtualcursor::Position {
                        pos: pos!(last_pos.x, last_pos.y + units),
                    };
                }
            }
        }
    }

    /// Move Cursor left (units).
    pub fn left(self, app: &mut App, units: usize) {
        if app.virtual_cursor != Virtualcursor::NotEnabled {
            let last_pos: Pos = match app.virtual_cursor {
                Virtualcursor::NotEnabled => pos!(0, 0),
                Virtualcursor::Position { pos } => pos,
            };

            if app.letter_grid.len() > 0 {
                if self.cursor_movement_bound_type == CurMovType::Wrap {
                    if last_pos.x > 0 {
                        app.virtual_cursor = Virtualcursor::Position {
                            pos: pos!(last_pos.x - units, last_pos.y),
                        };
                    } else if last_pos.y != 0 {
                        app.virtual_cursor = Virtualcursor::Position {
                            pos: pos!(0, last_pos.y - 1),
                        };
                    }
                } else if self.cursor_movement_bound_type == CurMovType::Block {
                    if last_pos.y > 0 {
                        app.virtual_cursor = Virtualcursor::Position {
                            pos: pos!(0, last_pos.y),
                        };
                    }
                } else {
                    app.virtual_cursor = Virtualcursor::Position {
                        pos: pos!(0, last_pos.y),
                    };
                }
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

            if app.letter_grid.len() > 0 {
                if self.cursor_movement_bound_type == CurMovType::Wrap {
                    // not at end of line check
                    if app.letter_grid[last_pos.y].len() > 2 {
                        if last_pos.x < app.letter_grid[last_pos.y].len() - 2 {
                            app.virtual_cursor = Virtualcursor::Position {
                                pos: pos!(last_pos.x + units, last_pos.y),
                            };
                        // reached end of line
                        } else {
                            app.virtual_cursor = Virtualcursor::Position {
                                pos: pos!(0, last_pos.y + 1),
                            };
                        }
                    }
                } else if self.cursor_movement_bound_type == CurMovType::Block {
                    if app.letter_grid[last_pos.y].len() > 2 {
                        if last_pos.x < app.letter_grid[last_pos.y].len() - 2 {
                            app.virtual_cursor = Virtualcursor::Position {
                                pos: pos!(last_pos.x + units, last_pos.y),
                            };
                        }
                    }
                } else {
                    app.virtual_cursor = Virtualcursor::Position {
                        pos: pos!(last_pos.x + units, last_pos.y),
                    };
                }
            }
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

    /// Enable text editor like wrapping.
    /// If the cursor hits the end of the line,
    /// it will jump to the start of the next line.
    pub fn wrap(self) -> Self {
        Mov {
            cursor_movement_bound_type: CurMovType::Wrap,
        }
    }

    pub fn freefloat(self) -> Self {
        Mov {
            cursor_movement_bound_type: CurMovType::Freefloat,
        }
    }

    pub fn block(self) -> Self {
        Mov {
            cursor_movement_bound_type: CurMovType::Block,
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
    format!("{:?}", &app.keypressed)
}

/*
pub struct Px {
    ch: char,
    fg: i8,
    bg: i8,
}

pub fn px_to_letter(ch: char, foreground_code: i8, background_code: i8) -> Letter {
    Letter {
        ch: ch,
        fg_code: foreground_code,
        bg_code: background_code,
        style: 0,
        when: LeadOnly::AlwaysShown,
    }
}

pub struct Keyframe {
    pub render_map: Vec<Vec<Letter>>,
}

pub struct Animation {
    pub step_duration: usize,
    pub timestep: usize,
    pub size: Pos,
    pub keyframes: Vec<Keyframe>,
}

impl Animation {
    pub fn new(step_duration: usize, size: Pos) -> Animation {
        Animation {
            step_duration: step_duration,
            timestep: 0,
            size: size,
            keyframes: vec![],
        }
    }

    pub fn set_frame(map: Vec<Vec<Px>>) {
        println!("dd");
    }
}
*/
