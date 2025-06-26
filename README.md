<details>
<summary>Table of Contents</summary>

- [⏱️ Quickstart](#quickstart)
- [📚 Documentation](#documentation)
- [🧩 Templates](#templates)
- [👁️ Built with TerimalRtdm](#built-with-TerimalRtdm)
- [⭐️ Leave a star!](#⭐️-Support-futher-development,-with-a-<mark>star</mark>!)
- [⌨️ Contributing](#contributing)
- [📄 License](#license)

</details>

## TerimalRtdm
Dependencless, Ideomatic, Rust Terimal Interface Library for quick CUIs when you need a tool ready pronto!

<p align="center">
  <img src="https://github.com/user-attachments/assets/f0b8b933-910f-473c-8d1f-ef9c9d54ca1c" alt="demo" />
</p>

<div align="center">
  <!-- MSRV -->
  <img src="https://img.shields.io/badge/Rust%20MSRV-1.78.0-brightgreen" style="zoom:150%;" />

  <!-- crates.io -->
  <a href="https://crates.io/crates/TerimalRtdm">
    <img src="https://img.shields.io/crates/v/TerimalRtdm.svg" style="zoom:150%;" />
    <img src="https://img.shields.io/crates/d/TerimalRtdm.svg" style="zoom:150%;" />
  </a>

  <!-- docs.rs -->
  <a href="https://docs.rs/TerimalRtdm">
    <img src="https://docs.rs/TerimalRtdm/badge.svg" style="zoom:150%;" />
  </a>

  <!-- License -->
  <img src="https://img.shields.io/crates/l/TerimalRtdm" style="zoom:150%;" />

  <!-- GitHub stars -->
  <a href="https://github.com/had2020/TerimalRtdm/stargazers">
    <img src="https://img.shields.io/github/stars/had2020/TerimalRtdm?style=social" style="zoom:150%;" />
  </a>

  [Report a Bug](https://github.com/had2020/TerimalRtdm/issues/new?labels=bug&template=bug_report.md) · [Request a Feature](https://github.com/had2020/TerimalRtdm/issues/new?labels=enhancement&template=feature_request.md) 

</div>

<mark>TerimalRtdm<mark> Which stands for: Terminal, Rust, Text, Display Manager.

- Display colored text at absolute positions
- Read key‑presses easily
- Simple app loop with App, and ideomatic methods
- No heavy dependencies, just ANSI escapes

## Quickstart

TerimalRtdm works in Rust binarys, follow these steps to setup a boiler plate, or check out the [TerimalRtdm Examples Repository](https://github.com/had2020/TerimalRtdm-examples)

<mark>1.) Add Crate.<mark> 

```shell
cargo add TerimalRtdm
```

<mark>2.) Then use this boiler plate.</mark>

```rust
use ::TerimalRtdm::*;

fn main() {
    let mut app = App::new(); // Holds all interface state variables.
    clear(&mut app); // Clear the screen competely.
    raw_mode(true); // Enabled for correct showing of elements at specific positions.
    show_cursor(false); // By default it is set to show. The cursor is off, since we don't need to move it.

    loop { // Each iteration is run by each input. As the crates are designed as a intermediate type UI.
        Text::new().show(&mut app, "Hello world", pos!(0, 0));

        // Example exit key
        if Key::o().pressed(&mut app, KeyType::Esc) {
            break;
        }

        render(&app);
        collect_presses(&mut app);
    }

    restore_terminal(); // Should be done at any exit of your program to restore the terminal defaults.
}
```

## Documentation

Todo documentation.md

## ⭐️ Support futher development, with a <mark>star</mark>!

[![GitHub](https://img.shields.io/badge/github-had2020%2FTerimalRtdm-blue?logo=github)](https://github.com/had2020/TerimalRtdm)

## Built with TerimalRtdm
Note these projects are outdated, but still demonstrate capabilities of the crate.
Both projects were built with verison 0.0.3, which is less ideomatic, and missing a lot of higher level features.

- For a Nano like text editer
https://github.com/had2020/Runo

- For a terimal based web browser.
https://github.com/had2020/RusosBr

## License

This project is licensed under the MIT License.

Copyright © Hadrian Lazic

See the [LICENSE](./LICENSE) file for details.

