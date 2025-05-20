# `Ventana`

## An iterator-based windowing library built in Rust

A key feature for this library is extensibility. Users can implement their own backends to replace the ones built into the core library. As I am only one student working on this in his free time, `ventana` is likely hilariously unoptimized in certain places. Certain performance liberties are taken in the name of maintainability and ease-of-use, but I am totally open to feedback concerning problematic code.

`ventana` stands upon the shoulders of giants. It takes heavy inspiration from works such as `piston` and `winit`; in some cases it directly incorporates code from them. In such instances, I have tried to take care to document what was taken alongside the licenses, but please file an issue if I have missed anything!
