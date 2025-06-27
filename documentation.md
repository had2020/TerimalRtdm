## <img src="https://fonts.gstatic.com/s/e/notoemoji/latest/1f44b/512.gif" alt="👋" width="32" height="32"> Welcome to the TerimalRtdm Docs
<p> This file covers all the need to know features, to get your cli programs working fast. <img src="https://fonts.gstatic.com/s/e/notoemoji/latest/26a1/512.gif" alt="⚡" width="24" height="24" style="vertical-align: middle;"> </p>

## Basics
TerimalRtdm is an intermediate mode terminal UI framework. This means each update to the interface is triggered by user input. It works in any Rust binary, but requires some light setup and a sequence of steps to initialize properly. That said, the setup process is simpler, and more versatile than most other UI crates.

## All features, correlated with their placement in the relevant program structure.
🖱️ You can `click` each instruction to open, and see code changes.

<details>
  
<summary>1️⃣ First off we need <mark>add the crate</mark> and use it in our code, in case you have not done that already.</summary>

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

<summary>2️⃣ You will need to <mark>setup the `App` variable</mark>, which is a struct that holds the core information, such as the key pressed, cursor position used to move the terminal cursor, and the rendering pool.</summary>

```rust
use TerimalRtdm::*;

fn main() {
  let mut app = App::new(); // 👈 New
  // Rest of the code ...
}
```

</details>

<details>

<summary>3️⃣ Rust prints to the terimal on start up, so let's <mark>clear</mark> that.</summary>

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

<summary>4️⃣ You can choose the <mark>show or hide the cursor</mark>.</summary>

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

<summary>5️⃣ The majority of the program will run within a single <mark>loop</mark>, that iterates per each input.</summary>

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

<summary>6️⃣ <mark>Showing some basic text.</mark> </summary>

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

Optionally you can change the text's style, position, foreground(color of the text) and background.

```rust
Text::new()
    .foreground(Color::Black)
    .background(Color::Magenta)
    .style(Style::Bold)
    .show(&mut app, "Replace", pos!(0, 13)); // each x and y is one char and "Hello world".len() returns 11, so we start at 13.
```
Keep in mind text is layered with the bottommost being the highest drawn layer, as your code goes down linearly. 

</details>

<details>

<summary>7️⃣ Simple <mark>check for input</mark> on our escape, this is optional, but something tells me, you might want a way to leave program eventually no matter how good it is.</summary>

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

<summary>8️⃣ Then at the end of the loop we can <mark>render</mark> all our text.</summary>

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

<summary>9️⃣ Last but not least we should <mark>collect the current press</mark> in our iteration of the loop.</summary>

This allows us to check for this input next loop.

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

Optional: Enable Escape Sequence Handling

By default, the app does **not** check for escape sequences (such as function keys and arrow keys).  
This is because enabling them requires tracking the `Esc` key being pressed up to **three times**,  
which can interfere with simple `Esc` based interactions.

Unless you **really** need to support function or arrow keys, it's recommended to leave this option disabled, so users can freely press `Esc` without unintended side effects.

If your app needs arrow or function key support, you can enable this behavior by toggling the relevant `bool` flag, and untoggle once the user needs to press escape, like in vim.
```rust
app.enable_f_row_and_arrow = true; 
```

</details>

<details>

<summary>🔟 Last, we should have our last line <mark>restore the terminal settings</mark> like raw capture mode and cursor visibility.</summary>

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

<details>

<summary>🕹️ Moving the Cursor!</summary>

You can easily move the cursor using the `Mov::cur().dir()` method along with key inputs. This snippet shows a typical setup using `Key::o()` with `KeyType` directions.

moving the cursor in any direction

```rust
if Key::o().no_clear().pressed(&mut app, KeyType::k) { // Note: you can replace these example vim keys with any keys you so disire.
    Mov::cur().dir(&mut app, Dir::Up, 1); // the last para 1, is the distance in letter units.
}

if Key::o().no_clear().pressed(&mut app, KeyType::j) {
    Mov::cur().dir(&mut app, Dir::Down, 1);
}

if Key::o().no_clear().pressed(&mut app, KeyType::h) {
    Mov::cur().dir(&mut app, Dir::Left, 1);
}

if Key::o().no_clear().pressed(&mut app, KeyType::l) {
    Mov::cur().dir(&mut app, Dir::Right, 1);
}
```

You can also change the behavior when moving the cursor as well.
By default it is set to wrap like a normal text editor.
but you can optinal apply the impl `block()` or `freefloat()` on `Mov::cur()`.

```rust
if Key::o().no_clear().pressed(&mut app, KeyType::l) {
    Mov::cur().block().dir(&mut app, Dir::Right, 1);
}
```

| Mode       | Description                                                                 |
|------------|-----------------------------------------------------------------------------|
| `block`    | Stops the user from moving out of the current line even if they reach the end. |
| `freefloat`| Lets the user move anywhere within the bounds of the screen.               |
| `wrap`     | The default; acts like nano or a normal text editor, pushing the cursor to the start of the next line once they reach the end. |

</details>


## ⭐️ Support futher development, with a <mark>star</mark>!

[![GitHub](https://img.shields.io/badge/github-had2020%2FTerimalRtdm-blue?logo=github)](https://github.com/had2020/TerimalRtdm)

https://github.com/had2020/TerimalRtdm

## License

This project is licensed under the MIT License.

Copyright © Hadrian Lazic

See the [LICENSE](./LICENSE) file for details.
