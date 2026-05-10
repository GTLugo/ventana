#![cfg(target_os = "macos")]

use {
  dispatch2::MainThreadBound,
  objc2::{
    ClassType,
    MainThreadMarker,
    Message,
    define_class,
    msg_send,
    rc::Retained,
  },
  objc2_app_kit::{
    NSBackingStoreType,
    NSColor,
    NSResponder,
    NSScreen,
    NSView,
    NSWindow,
    NSWindowStyleMask,
  },
  objc2_core_foundation::{
    CGFloat,
    CGPoint,
    CGSize,
  },
  objc2_foundation::{
    NSObject,
    NSRect,
    NSString,
  },
  std::sync::Arc,
  ventana_hal::{
    dpi::{
      PhysicalPosition,
      PhysicalSize,
      Position,
      Size,
    },
    error::RequestError,
    event::Event,
    keyboard::{
      Code,
      KeyState,
    },
    pointer::{
      ButtonState,
      button::PointerButton,
    },
    raw_window_handle::{
      AppKitDisplayHandle,
      AppKitWindowHandle,
      RawDisplayHandle,
      RawWindowHandle,
    },
    rgb::RGB8,
    settings::WindowSettings,
    window::{
      BackendWindow,
      WindowId,
    },
  },
};

define_class!(
  #[unsafe(super(NSWindow, NSResponder, NSObject))]
  #[name = "VentanaWindow"]
  #[derive(Debug)]
  pub struct VentanaWindow;

  impl VentanaWindow {
    #[unsafe(method(canBecomeMainWindow))]
    fn can_become_main_window(&self) -> bool {
      true
    }

    #[unsafe(method(canBecomeKeyWindow))]
    fn can_become_key_window(&self) -> bool {
      true
    }
  }
);

pub struct AppKitWindow {
  screen: MainThreadBound<Retained<NSScreen>>,
  window: MainThreadBound<Retained<NSWindow>>,
  size: PhysicalSize<u32>,
}

impl AppKitWindow {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    let _ = settings;

    let mtm =
      MainThreadMarker::new().ok_or(RequestError::NotSupported("Marker must be on main thread.".into()))?;

    let screen =
      NSScreen::mainScreen(mtm).ok_or(RequestError::NotSupported("No main screen found.".into()))?;

    let style = NSWindowStyleMask::Closable
      | NSWindowStyleMask::Titled
      | NSWindowStyleMask::Miniaturizable
      | NSWindowStyleMask::Resizable;

    // log::info!("Backing scale factor: {}", screen.backingScaleFactor());

    let scale_factor = screen.backingScaleFactor();

    let physical_size = settings.size.to_physical(scale_factor);
    let cgsize = CGSize::new(physical_size.width, physical_size.height);

    let physical_pos = match settings.position {
      Some(pos) => {
        let mut pos = pos.to_physical(scale_factor);

        pos.x += (physical_size.width / 2.0) as i32;
        pos.y += (physical_size.height / 2.0) as i32;

        Position::Physical(pos)
      },
      None => Position::Physical(
        (
          (screen.frame().size.width - cgsize.width) / 2.0,
          (screen.frame().size.height - cgsize.height) / 2.0,
        )
          .into(),
      ),
    }
    .to_physical(scale_factor);
    let origin = CGPoint::new(physical_pos.x, physical_pos.y);

    let content_rect = NSRect::new(origin, cgsize).standardize();

    let window: Retained<NSWindow> = unsafe {
      let window: Option<Retained<VentanaWindow>> = msg_send![
        super(mtm.alloc().set_ivars(())),
        initWithContentRect: content_rect,
        styleMask: style,
        backing: NSBackingStoreType::Buffered,
        defer: false
      ];

      window.ok_or(RequestError::NotSupported("Failed to create window.".into()))?.as_super().retain()
    };

    unsafe { window.setReleasedWhenClosed(false) };

    window.setTitle(&NSString::from_str(&settings.title));
    window.setAcceptsMouseMovedEvents(true);

    let (r, g, b) = settings
      .clear_color
      .map(|RGB8 { r, g, b }| (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0))
      .unwrap_or_else(|| {
        let clear = NSColor::clearColor();
        (clear.redComponent(), clear.greenComponent(), clear.blueComponent())
      });

    fn init_color(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Retained<NSColor> {
      unsafe {
        msg_send![
          NSColor::class(),
          colorWithCalibratedRed: red,
          green: green,
          blue: blue,
          alpha: alpha
        ]
      }
    }

    let nscolor = init_color(r, g, b, 1.0);
    window.setBackgroundColor(Some(nscolor.as_ref()));

    let screen = MainThreadBound::new(screen, mtm);
    let window = MainThreadBound::new(window, mtm);

    Ok(Self { screen, window, size: physical_size.cast() })
  }

  fn view(&self) -> Retained<NSView> {
    let mtm = MainThreadMarker::new().unwrap();
    self.window.get(mtm).contentView().unwrap().downcast().unwrap()
  }
}

impl BackendWindow for AppKitWindow {
  fn id(&self) -> WindowId {
    todo!()
  }

  fn raw_window_handle(&self) -> ventana_hal::raw_window_handle::RawWindowHandle {
    let window_handle = AppKitWindowHandle::new({
      let ptr = Retained::as_ptr(&self.view()) as *mut _;
      std::ptr::NonNull::new(ptr).unwrap()
    });
    RawWindowHandle::AppKit(window_handle)
  }

  fn raw_display_handle(&self) -> ventana_hal::raw_window_handle::RawDisplayHandle {
    RawDisplayHandle::AppKit(AppKitDisplayHandle::new())
  }

  fn monitor(&self) -> Arc<dyn ventana_hal::monitor::BackendMonitor> {
    todo!()
  }

  fn next(&self) -> Option<Event> {
    None
  }

  fn try_next(&self) -> Option<Event> {
    None
  }

  fn request_redraw(&self) {
    todo!()
  }

  fn close(&self) {
    todo!()
  }

  fn is_closing(&self) -> bool {
    todo!()
  }

  fn title(&self) -> String {
    todo!()
  }

  fn set_title(&self, title: String) {
    let _ = title;
    todo!()
  }

  fn scale_factor(&self) -> f64 {
    todo!()
  }

  fn inner_size(&self) -> PhysicalSize<u32> {
    self.size
  }

  fn outer_size(&self) -> PhysicalSize<u32> {
    todo!()
  }

  fn inner_position(&self) -> PhysicalPosition<i32> {
    todo!()
  }

  fn outer_position(&self) -> PhysicalPosition<i32> {
    todo!()
  }

  fn key(&self, keycode: Code) -> KeyState {
    let _ = keycode;
    todo!()
  }

  fn pointer(&self, button: PointerButton) -> ButtonState {
    let _ = button;
    todo!()
  }

  fn shift_key(&self) -> KeyState {
    todo!()
  }

  fn ctrl_key(&self) -> KeyState {
    todo!()
  }

  fn alt_key(&self) -> KeyState {
    todo!()
  }

  fn meta_key(&self) -> KeyState {
    todo!()
  }
}
