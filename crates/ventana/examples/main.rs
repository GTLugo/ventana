use ventana::window::Window;

fn main() {
  #[allow(unreachable_code)]
  let window = Window::builder().build();
  // let window = Window::builder().with_backend(Backend::default()).build();
  // let window = Window::new(&WindowSettings::default());
  // let window = Window::with_backend(&Backend::default(), &WindowSettings::default());
  // let window = Window::with_backend(
  //   &Backend::select(|| {
  //     #[cfg(windows_platform)]
  //     return Box::new(ventana::backend::win32::Win32Backend);
  //     #[cfg(x11_platform)]
  //     return Box::new(ventana::backend::x11::X11Backend);
  //     panic!("No valid backend selected");
  //   }),
  //   &WindowSettings::default(),
  // );

  println!("{}", window.title());
}
