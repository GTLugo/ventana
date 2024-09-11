use ventana::window::{Backend, Window, WindowSettings};

fn main() {
  // let window = Window::builder().build();
  // let window = Window::new(&WindowSettings::default());
  let window = Window::with_backend(&Backend::default(), &WindowSettings::default());

  println!("{}", window.title());
}
