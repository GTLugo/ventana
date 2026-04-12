use ventana::{
  dpi::PhysicalSize,
  window::Window,
};

pub fn initialize_logger() {
  env_logger::builder()
    .filter(None, log::LevelFilter::Trace)
    .filter(Some("wgpu"), log::LevelFilter::Off)
    .filter(Some("naga"), log::LevelFilter::Off)
    .format_source_path(true)
    .init();
}

pub struct State {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  is_surface_configured: bool,
  window: Window,
}

impl State {
  // We don't need this to be async right now,
  // but we will in the next tutorial
  pub async fn new(window: Window) -> anyhow::Result<Self> {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      flags: Default::default(),
      memory_budget_thresholds: Default::default(),
      backend_options: Default::default(),
      display: None,
    });

    let surface = instance.create_surface(window.clone())?;

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await?;

    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        experimental_features: unsafe { wgpu::ExperimentalFeatures::enabled() },
        required_limits: wgpu::Limits::default(),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
      })
      .await?;

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
      .formats
      .iter()
      .find(|f| f.is_srgb())
      .copied()
      .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::AutoNoVsync,
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    Ok(Self {
      surface,
      device,
      queue,
      config,
      is_surface_configured: true,
      window,
    })
  }

  pub fn resize(&mut self, size: PhysicalSize<u32>) {
    log::info!("Resize: ({}, {})", size.width, size.height);
    if size.width > 0 && size.height > 0 && (size.width != self.config.width || size.height != self.config.height) {
      self.config.width = size.width;
      self.config.height = size.height;
      self.reconfigure();
    }
  }

  fn reconfigure(&mut self) {
    self.surface.configure(&self.device, &self.config);
    self.is_surface_configured = true;
  }

  pub fn update(&mut self) {
    // remove `todo!()`
  }

  pub fn draw(&mut self) {
    match self.render() {
      Ok(_) => (),
      Err(e) => {
        log::error!("{e}");
        self.window.close();
      },
    }
  }

  fn render(&mut self) -> anyhow::Result<()> {
    // self.window.request_redraw();

    // We can't render unless the surface is configured
    if !self.is_surface_configured {
      return Ok(());
    }

    let output = match self.surface.get_current_texture() {
      wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
      wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
        self.is_surface_configured = false; // Cannot reconfigure until current surface texture is dropped
        surface_texture
      },
      wgpu::CurrentSurfaceTexture::Timeout
      | wgpu::CurrentSurfaceTexture::Occluded
      | wgpu::CurrentSurfaceTexture::Validation => {
        // Skip this frame
        return Ok(());
      },
      wgpu::CurrentSurfaceTexture::Outdated => {
        self.reconfigure();
        return Ok(());
      },
      wgpu::CurrentSurfaceTexture::Lost => {
        // You could recreate the devices and all resources
        // created with it here, but we'll just bail
        anyhow::bail!("Lost device");
      },
    };

    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Render Encoder"),
    });

    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          depth_slice: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            }),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
        multiview_mask: None,
      });
    }

    // submit will accept anything that implements IntoIter
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
