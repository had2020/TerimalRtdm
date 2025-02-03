## TerimalRtdm
Lightwight, Simple, Easy to read, Ideomatic Rust Terimal Interface Library for CUIs

![MSRV](https://img.shields.io/badge/Rust%20MSRV-1.78.0-brightgreen)
[![crates.io](https://img.shields.io/crates/v/TerimalRtdm.svg)](https://crates.io/crates/TerimalRtdm/0.0.1)
[![Downloads](https://img.shields.io/crates/d/laststraw.svg)](https://crates.io/crates/laststraw)

## Why use it 
This framework provides many usful functions that abstract ACSI escape keys,
allowing for easy implementation for a Terimal Interface. 

- Many features like key_press(), allows you to check a key without halting for a check.
- You can write Terimal GUIs much faster!
- Lots of functions allowing for more customizability, while keeping code readable and ideomatic.

# Example app
This is an example of using the framework,
involving two key checks that don't halt the program twice.
``` rust
clear(); // clear any debug logs
let mut app = App::new(); // store the variables relating to the terimal app

raw_line("q <- to quit"); // displays on a new line
raw_line("w <- to show lines"); // use only for startup text, relay on line method

raw_mode(true); // for propper input enable

// app loop
loop {
    clear(); // clear the screen, last loop, or Rust debug logs
    collect_presses(&mut app); // store the current key, in a variable for the loop

    if key_press(&app, "q") { // checks the stored current key, if it's "q"
        clear(); // clear the sceen
        break; // exits the terimal app
    }

    if key_press(&app, "w") {
        // displays First Sec, with Position struct
        // You can use position macro to shortten
        line(Position { x: 0, y: 5 }, "First", "blue");
        line(Position { x: 0, y: 11 }, "Sec", "red");
    }
}

raw_mode(false); // disable to release the user terimal
```

## Core functions 
App Struct

- Used to automatically hold variables outside the app loop.
- Holds the key buffer, from input and the key pressed for input checking in app. loop.
``` Rust
let mut app = App::new();
```

clear()

- Used to clear the screen.
- Tip: follow the structure in the later section of the Docs.
``` Rust
clear();
```

cursor_state() 

- Used to toggle on and off the Cursor
``` Rust
cursor_state(false);
```

raw_mode()

- Used to enable raw input, which is required for most Terimal apps.
- Tip: Should toggled true before app loop, and toggled false after, to free the user.
``` Rust
raw_mode(true)
```

raw_line()

- displays on a new line before app loop
- Use only for startup text, relay on line method
``` Rust
raw_line("q <- to quit");
```

halt_press_check()

- Halts the program until the user has pressed a key
- will not be recored into app variable.
- So input will only be used for this if statement.
``` Rust
if halt_press_check(&mut app, "q") {
    break;
}
```

Position Struct 

- Used as argument in many functions.
- Holds x and y position in the Terimal.
- X number of spaced words to the right
- Y number pf spaced workds Down
- Tip: You can use the position! macro to easly make a Position Structed
``` Rust
position!(0, 0)
```

collect_presses() 

- collects the current press in a single halt, and stores it in App variable.
- Later inout statements refer to this value, to prevent unneeded halts, within the main loop for app.
- Used after clear, before any input if statements.
``` Rust
collect_presses(&mut app);
```

key_press()

- You must have one collect_presses() before at the start of the loop!
- Returns true if &str equals the last &str of input.
``` Rust
if key_press(&app, "q") { // checks the stored current key, if it's "q"
    clear(); // clear the sceen
    break; // exits the terimal app
}
```

key_press_not_case_sen()

- Refer to key_press(), for usage.
- Its the same, just not case sensitive.
``` Rust
if key_press(&app, "Q") { // checks the stored current key, if it's "q" or "Q"
    break; // exits the terimal app
}
```


## How it works

## Color Map

| Color    | ANSI Code    |
|----------|-------------|
| red      | `\x1B[31m`  |
| green    | `\x1B[32m`  |
| yellow   | `\x1B[33m`  |
| blue     | `\x1B[34m`  |
| magenta  | `\x1B[35m`  |
| cyan     | `\x1B[36m`  |
| white    | `\x1B[37m`  |
| _ (default) | `\x1B[0m`  |


## I used it in another project
for a terimal based web browser.

https://github.com/had2020/RusosBr


