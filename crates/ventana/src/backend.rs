#[cfg(wayland_platform)]
pub use ventana_backend_wayland as wayland;
#[cfg(windows_platform)]
pub use ventana_backend_win32 as win32;
#[cfg(wayland_platform)]
pub use wayland::Wayland;
#[cfg(windows_platform)]
pub use win32::Win32;
