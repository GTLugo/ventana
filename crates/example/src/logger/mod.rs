mod formatter;

use {
  std::{
    collections::VecDeque,
    sync::{
      LazyLock,
      Mutex,
      MutexGuard,
    },
  },
  tracing::level_filters::LevelFilter,
  tracing_appender::{
    non_blocking::{
      NonBlocking,
      WorkerGuard,
    },
    rolling::Rotation,
  },
  tracing_log::LogTracer,
  tracing_subscriber::{
    EnvFilter,
    fmt::{
      self,
      Layer,
      format::{
        DefaultFields,
        Format,
      },
    },
    layer::SubscriberExt,
  },
};

pub struct Logger {
  guards: VecDeque<WorkerGuard>,
}

impl Logger {
  pub fn init() -> anyhow::Result<()> {
    let mut logger = Self::instance();

    let filter = EnvFilter::builder()
      .with_default_directive(LevelFilter::TRACE.into())
      .from_env()?
      .add_directive("wgpu=off".parse()?)
      .add_directive("naga=off".parse()?);

    let stdout_layer = logger.add_layer(tracing_appender::non_blocking(std::io::stdout())).pretty();
    let file_layer = logger
      .add_layer(tracing_appender::non_blocking(
        tracing_appender::rolling::Builder::new()
          .filename_prefix("out")
          .filename_suffix("log")
          .max_log_files(1)
          .rotation(Rotation::NEVER)
          .build("./logs")?,
      ))
      .with_ansi(false);

    let subscriber = tracing_subscriber::registry().with(filter).with(stdout_layer).with(file_layer);

    LogTracer::init()?;

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
  }

  fn instance() -> MutexGuard<'static, Self> {
    static INSTANCE: LazyLock<Mutex<Logger>> =
      LazyLock::new(|| Mutex::new(Logger { guards: VecDeque::default() }));
    INSTANCE.lock().unwrap()
  }

  fn add_layer<T>(
    &mut self,
    (writer, guard): (NonBlocking, WorkerGuard),
  ) -> Layer<T, DefaultFields, Format, NonBlocking> {
    self.guards.push_back(guard);
    fmt::Layer::new().with_writer(writer).with_thread_names(true)
  }
}
