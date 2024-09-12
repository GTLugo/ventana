use ventana::window::Window;
use ventana_backend_win32::Win32;

fn main() {
  let window = Window::builder().with_backend(Win32).build();

  println!("{}", window.title());
}
