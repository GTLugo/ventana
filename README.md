# <div align="center">`Ventana`</div>

[![Crates.io Version](https://img.shields.io/crates/v/ventana?style=for-the-badge)](https://crates.io/crates/ventana)
[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/R6R8PGIU6)

```rust
use ventana::prelude::*;

let window = Window::new(WindowOptions {
  title: "Example",
  size: Size::Logical((800, 500).into()),
  ..Default::default()
})?;

for event in &window {
  println!("{event:?}");
}
```

## An iterator-based windowing library built in Rust

The two key features for this library are the iterator API and backend extensibility. As there are many different platforms, each with their own unique windowing APIs, Ventana is designed such that users may implement their own backends to replace the ones built into the core library.

Please note, as I am only one person working on this in his free time, Ventana is likely hilariously unoptimized in certain places. Certain performance liberties are taken in the name of maintainability and ease-of-use, but I am completely open to feedback concerning particularly problematic code.

## Backends

> [!IMPORTANT]
> Alternate platforms not listed below can be implemented as third-party backends and fed into the `backend` field of `WindowOptions`. While the library is designed to be cross-platform, the overall library is still in its infancy and support across the board is still work-in-progress. MacOS support is currently unplanned as I do not own any MacOS devices.

Platform | First-Party Support
-|-
Windows | 🚧
X11 | 🚧
Wayland | 🚧
MacOS | ❌

###### ✅ - Implemented
###### ⚠️ - Partial
###### 🚧 - In development
###### ❌ - No first-party support planned

## Credits

Ventana stands upon the shoulders of giants. It takes heavy inspiration from works such as Piston and Winit; in some cases it directly incorporates code from them. In such instances, I have tried to take care to document what was taken alongside the licenses, but please file an issue if I have missed anything. I try to take plagiarism seriously, but a lot of this was written late at night after work and on my 10th cup of coffee.

----

> [!NOTE]
> No AI-generated code.
