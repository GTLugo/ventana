pub mod logger;

use {
  std::time::Duration,
  ventana::{
    dpi::PhysicalSize,
    window::Window,
  },
};

pub struct State {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  is_surface_configured: bool,
  render_pipeline: wgpu::RenderPipeline,
  window: Window,
  last_frame_time: std::time::Instant,
  delta_time: std::time::Duration,
}

impl State {
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
    let surface_format =
      surface_caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(surface_caps.formats[0]);
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

    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      immediate_size: 0,
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: wgpu::PipelineCompilationOptions::default(),
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
        polygon_mode: wgpu::PolygonMode::Fill,
        // Requires Features::DEPTH_CLIP_CONTROL
        unclipped_depth: false,
        // Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
      },
      depth_stencil: None, // 1.
      multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false },
      multiview_mask: None,
      cache: None,
    });

    Ok(Self {
      surface,
      device,
      queue,
      config,
      is_surface_configured: true,
      render_pipeline,
      window,
      last_frame_time: std::time::Instant::now(),
      delta_time: std::time::Duration::ZERO,
    })
  }

  pub fn resize(&mut self, size: PhysicalSize<u32>) {
    // log::info!("Resize: ({}, {})", size.width, size.height);
    if size.width > 0
      && size.height > 0
      && (size.width != self.config.width || size.height != self.config.height)
    {
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
    self.delta_time = self.last_frame_time.elapsed();
    self.last_frame_time = std::time::Instant::now();
    match self.render() {
      Ok(_) => (),
      Err(e) => {
        log::error!("{e}");
        self.window.close();
      },
    }
  }

  pub fn fps(&self) -> usize {
    (1.0 / self.delta_time.as_secs_f64()) as usize
  }

  pub fn delta_time(&self) -> Duration {
    self.delta_time
  }

  fn render(&mut self) -> anyhow::Result<()> {
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

    let mut encoder =
      self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });

    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          depth_slice: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
        multiview_mask: None,
      });

      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.draw(0..3, 0..1);
    }

    // submit will accept anything that implements IntoIter
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
