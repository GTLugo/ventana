pub mod iter;
pub mod raw;

use {
  hal::{
    backend::Backend,
    dpi::{
      PhysicalPosition,
      PhysicalSize,
      Position,
      Size,
    },
    error::RequestError,
    event::Event,
    monitor::Monitor,
    rgb::RGB8,
    settings::WindowSettings,
    types::{
      Flow,
      Visibility,
    },
    window::{
      BackendWindow,
      WindowId,
    },
  },
  std::sync::Arc,
};

#[derive(Clone)]
pub struct Window
where
  Self: Send + Sync,
{
  backend: &'static dyn Backend,
  window: Arc<dyn BackendWindow>,
}

impl std::fmt::Debug for Window {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Window")
      .field("backend", &self.backend.name())
      .field("window", &self.window.id())
      .finish()
  }
}

impl std::fmt::Display for Window {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{backend}({id})", backend = self.backend.name(), id = self.window.id())
  }
}

impl Window {
  pub fn new(options: WindowOptions) -> Result<Self, RequestError> {
    let settings: WindowSettings = options.clone().into();
    let Some(backend) = options.backend else {
      log::warn!("No supported backend selected; window creation request ignored.");
      return Err(RequestError::Ignored);
    };

    log::trace!("Creating window `{}`", settings.title);

    let window = backend.create_window(settings)?;
    Ok(Self { backend, window })
  }

  pub fn backend(&self) -> &'static dyn Backend {
    self.backend
  }

  pub fn id(&self) -> WindowId {
    self.window.id()
  }

  // It's giving me headaches and apparently not all platforms might support this anyways
  // pub fn title(&self) -> String {
  //   self.window.title()
  // }

  pub fn set_title(&self, title: impl Into<String>) {
    self.window.set_title(title.into())
  }

  pub fn scale_factor(&self) -> f64 {
    self.window.scale_factor()
  }

  pub fn monitor(&self) -> Monitor {
    Monitor::new(self.window.monitor())
  }

  pub fn inner_size(&self) -> PhysicalSize<u32> {
    self.window.inner_size()
  }

  pub fn outer_size(&self) -> PhysicalSize<u32> {
    self.window.outer_size()
  }

  pub fn inner_position(&self) -> PhysicalPosition<i32> {
    self.window.inner_position()
  }

  pub fn outer_position(&self) -> PhysicalPosition<i32> {
    self.window.outer_position()
  }

  pub fn request_redraw(&self) {
    self.window.request_redraw()
  }

  pub fn next_event(&self) -> Option<Event> {
    self.window.next()
  }

  // TODO: This should probably be named `Exit` or something similar as it exits the loop rather
  //      than actually closing the window. The backend will handle closing the window when the
  //      internal window is dropped.
  pub fn close(&self) {
    self.window.close()
  }
}

#[derive(Clone)]
pub struct WindowOptions {
  pub backend: Option<&'static dyn Backend>,
  pub title: &'static str,
  pub size: Size, // Maybe should make this optional and have backend handle None case
  pub position: Option<Position>,
  pub visibility: Visibility,
  pub flow: Flow,
  pub close_on_x: bool,
  pub clear_color: Option<RGB8>,
  // pub reveal_delay_frames: Option<u32>,
}

impl Default for WindowOptions {
  fn default() -> Self {
    Self {
      backend: cfg_select! {
        feature = "auto-backend" => backend::AutoBackend::backend(),
        _ => None,
      },
      title: "Window",
      size: Size::Logical((800.0, 500.0).into()),
      position: None,
      visibility: Default::default(),
      flow: Default::default(),
      close_on_x: true,
      clear_color: None,
      // reveal_delay_frames: None,
    }
  }
}

impl WindowOptions {
  pub const BLACK: RGB8 = RGB8::new(0, 0, 0);
  pub const BLUE: RGB8 = RGB8::new(0, 0, 255);
  pub const DARK_GRAY: RGB8 = RGB8::new(192, 192, 192);
  pub const GRAY: RGB8 = RGB8::new(127, 127, 127);
  pub const GREEN: RGB8 = RGB8::new(0, 255, 0);
  pub const LIGHT_GRAY: RGB8 = RGB8::new(63, 63, 63);
  pub const RED: RGB8 = RGB8::new(255, 0, 0);
  pub const WHITE: RGB8 = RGB8::new(255, 255, 255);

  pub fn with_backend(mut self, backend: Option<&'static impl Backend>) -> Self {
    self.backend = backend.map(|b| b as _);
    self
  }

  pub fn with_title(mut self, title: impl Into<&'static str>) -> Self {
    self.title = title.into();
    self
  }

  pub fn with_size(mut self, size: impl Into<Size>) -> Self {
    self.size = size.into();
    self
  }

  pub fn with_position(mut self, position: Option<impl Into<Position>>) -> Self {
    self.position = position.map(Into::into);
    self
  }

  pub fn with_visibility(mut self, visibility: Visibility) -> Self {
    self.visibility = visibility;
    self
  }

  pub fn with_flow(mut self, flow: Flow) -> Self {
    self.flow = flow;
    self
  }

  pub fn with_close_on_x(mut self, close_on_x: bool) -> Self {
    self.close_on_x = close_on_x;
    self
  }

  pub fn with_clear_color(mut self, clear_color: impl Into<RGB8>) -> Self {
    self.clear_color = Some(clear_color.into());
    self
  }
}

impl From<WindowOptions> for WindowSettings {
  fn from(options: WindowOptions) -> Self {
    Self {
      title: options.title.into(),
      size: options.size,
      position: options.position,
      visibility: options.visibility,
      flow: options.flow,
      close_on_x: options.close_on_x,
      clear_color: options.clear_color,
      // reveal_delay_frames: options.reveal_delay_frames,
    }
  }
}
