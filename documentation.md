<details>
<summary>Table of Contents</summary>

- [Basics](#basics)
- [Structuring](#structuring)
- [App Variable](#app-variable)
- [Raw Mode](#raw-mode)
- [Cursor Visibility](#cursor-visibility)
- [Input Loop](#input-loop)
- [Collecting Loop Press](#collecting-loop-press)
- [Text Elements](#text-elements)
- [Checking Key Inputs](#checking-key-inputs)
- [Rendering](#rendering)
- [Moving The Cursor](#moving-the-cursor)
- [Restore Terminal](#restore-terminal)
- [Advanced Features](#advanced-features)

</details>

## <img src="https://fonts.gstatic.com/s/e/notoemoji/latest/1f44b/512.gif" alt="👋" width="32" height="32"> Welcome to the TerimalRtdm Docs
<p> This file covers all the need to know features, to get your cli programs working fast. <img src="https://fonts.gstatic.com/s/e/notoemoji/latest/26a1/512.gif" alt="⚡" width="24" height="24" style="vertical-align: middle;"> </p>

## Basics
TerimalRtdm is an intermediate mode terminal UI framework. This means each update to the interface is triggered by user input. It works in any Rust binary, but requires some light setup and a sequence of steps to initialize properly. That said, the setup process is simpler, and more versatile than most other UI crates.

## Structuring
You can click each instruction to open, and see code changes.

<details>
  
<summary>1.) First off we need add the crate and use it in our code, in case you have not done that already.</summary>

Add our crate to your toml:
```shell
cargo add TerimalRtdm 
```

Then declare it's usage:
```rust
use TerimalRtdm::*;
```

</details>

<details>

<summary>2.) You will need to setup the `App` variable, which is a struct that holds the core information, such as the key pressed, cursor position used to move the terminal cursor, and the rendering pool.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new(); // 👈 New
  // Rest of the code ...
}
```

</details>

<details>

<summary>3.) Rust prints to the terimal on start up, so let's `clear` that.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new();
  clear(&mut app); // 👈 New
  // Rest of the code ...
}
```

</details>

<details>

<summary>4.) You can choose the show or hide the cursor.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new();
  clear(&mut app);
  show_cursor(false); // 👈 New
  // Rest of the code ...
}
```

</details>

<details>

<summary>5.) The majority of the program will run within a single `loop`, that iterates per each input.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new();
  clear(&mut app);
  show_cursor(false);

  loop { // 👈 New
    // Rest of the code ...
  }
}
```

</details>

<details>

<summary>6.) Showing some basic text. This crate provides futher text features then this, see `Text Elements`.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new();
  clear(&mut app);
  show_cursor(false);

  loop {
    Text::new().show(&mut app, "Hello world", pos!(0, 0)); // 👈 New
    // Rest of the code ...
  }
}
```

</details>

<details>

<summary>7.) Simple check for input on our escape, this is optional, but something tells me, you want a way to leave program eventually no matter how good it is.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new();
  clear(&mut app);
  show_cursor(false);

  loop {
    Text::new().show(&mut app, "Hello world", pos!(0, 0));

    if Key::o().pressed(&mut app, KeyType::Esc) { // 👈 New
      break;
    }
    // Rest of the code ...
  }
}
```

</details>

<details>

<summary>8.) Then at the end of the loop we can render the text.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new();
  clear(&mut app);
  show_cursor(false);

  loop {
    Text::new().show(&mut app, "Hello world", pos!(0, 0));

    if Key::o().pressed(&mut app, KeyType::Esc) { 
      break;
    }

    render(&app); // 👈 New
    // Rest of the code ...
  }
}
```

</details>

<details>

<summary>9.) Last but not least we should collect the current press in our iteration of the loop.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new();
  clear(&mut app);
  show_cursor(false);

  loop {
    Text::new().show(&mut app, "Hello world", pos!(0, 0));

    if Key::o().pressed(&mut app, KeyType::Esc) { 
      break;
    }

    render(&app); 
    collect_presses(&mut app); // 👈 New
  }
}
```

</details>

<details>

<summary>10.) Last, we should have our last line restore the terminal settings like raw capture mode and cursor visibility.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new();
  clear(&mut app);
  show_cursor(false);

  loop {
    Text::new().show(&mut app, "Hello world", pos!(0, 0));

    if Key::o().pressed(&mut app, KeyType::Esc) { 
      break;
    }

    render(&app); 
    collect_presses(&mut app); // 👈 New 
  }

  restore_terminal();
}
```

</details>

## App Variable

## Raw Mode

## Cursor Visibility

## Input Loop

## Collecting Loop Press

## Text Elements

## Checking Key Inputs

## Rendering

## Moving The Cursor

## Restore Terminal

## Advanced Features

## ⭐️ Support futher development, with a <mark>star</mark>!

[![GitHub](https://img.shields.io/badge/github-had2020%2FTerimalRtdm-blue?logo=github)](https://github.com/had2020/TerimalRtdm)

https://github.com/had2020/TerimalRtdm

## License

This project is licensed under the MIT License.

Copyright © Hadrian Lazic

See the [LICENSE](./LICENSE) file for details.
