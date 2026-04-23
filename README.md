# <div align="center">Ventana</div>

<div align="center">
  <a href="https://crates.io/crates/ventana"><img src="https://img.shields.io/crates/v/ventana?style=for-the-badge" alt="Crates.io"></a>
  <a href="https://docs.rs/ventana"><img src="https://img.shields.io/docsrs/ventana?style=for-the-badge" alt="Docs.rs"></a>
  <img src="https://img.shields.io/crates/l/ventana?style=for-the-badge" alt="License">
  <!-- <a href="https://ko-fi.com/R6R8PGIU6"><img src="https://ko-fi.com/img/githubbutton_sm.svg" alt="ko-fi"></a> -->
</div>

```rust
use ventana::prelude::*;

let window = Window::new(WindowOptions::default())?;

for event in window {
  println!("{event:?}");
}
```

## An iterator-based windowing library built in Rust

A key feature for this library is the iterator API. Working with callbacks and traits is nice, but can be a little intense for a simple application. The iterator API in Rust is easy-to-use and fits very naturally with how one might perceive events as arriving in a window like letters in a mailbox. This library allows users to take advantage of that elegance.

## Bring your own backend (or use Ventana's)

As there are many different platforms, each with their own unique windowing APIs, Ventana is designed such that users may implement their own backends to replace the ones built into the core library. If you don't like the way the default backends work, you can disable the `auto-backend` feature and plug your own into the `WindowOptions` struct. You can also do this if you are working on an unsupported platform and prefer to keep your builds lean.

## Backends

Ventana comes by default with the `AutoBackend` which supports the platforms listed below. While the library is designed to be cross-platform, the overall library is still in its infancy and support across the board is still work-in-progress.

Platform | Auto-backend Support
-|-
Windows | 🚧(WIP)
X11 | 🚧(WIP)
Wayland | 🚧(WIP)
MacOS | ❌(Planned)

> [!IMPORTANT]
> Alternate platforms not listed above can be implemented as third-party backends and fed into the `backend` field of `WindowOptions`. 

## Final words 

Please note, as I am only one person working on this in his free time, Ventana is likely hilariously unoptimized in certain places. Certain performance liberties are taken in the name of maintainability and ease-of-use, but I am completely open to feedback concerning particularly problematic code.

## Credits

Ventana stands upon the shoulders of giants. It takes heavy inspiration from works such as Piston and Winit; in some cases it directly incorporates code from them. In such instances, I have tried to take care to document what was taken alongside the licenses, but please file an issue if I have missed anything. I try to take plagiarism seriously, but a lot of this was written late at night after work and on my 10th cup of coffee.

----

> [!NOTE]
> No AI-generated code.
