#[cfg(windows_platform)]
pub use ventana_backend_win32 as win32;
#[cfg(windows_platform)]
pub use win32::Win32 as Win32;

#[cfg(wayland_platform)]
pub use ventana_backend_wayland as wayland;
#[cfg(wayland_platform)]
pub use wayland::Wayland as Wayland;
