use ventana::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let window = Window::builder()
    .with_backend(ventana::backend::Win32) // if omitted, it will auto-select from built-in backends
    .with_settings(WindowSettings { size: Size::Logical((400.0, 250.0).into()), ..Default::default() })
    .build()?;

  while let Some(msg) = window.next_message() {
    println!("{msg:?}");
  }

  Ok(())
}
