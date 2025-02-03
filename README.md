## TerimalRtdm
Ideomatic Rust Terimal Interface Library for CUIs

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
