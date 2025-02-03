# TerimalRtdm
 Rust Terimal Display Manipulator

# Example app
``` rust
fn main() {
    clear();
    let mut app = App::new(5, 5);

    raw_line("q <- to quit");

    raw_mode(true);
    loop {
        clear();
        // app loop
        if key_pressed(&mut app, "q") {
            clear();
            break;
        }

        if key_pressed(&mut app, "w") {
            line(Position { x: 0, y: 5 }, "First", "blue");
            line(Position { x: 0, y: 11 }, "Sec", "red");
        }
    }

    raw_mode(false);
}
```
