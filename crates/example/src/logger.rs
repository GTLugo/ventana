use {
  std::collections::VecDeque,
  tracing::level_filters::LevelFilter,
  tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::Rotation,
  },
  tracing_log::LogTracer,
  tracing_subscriber::{
    EnvFilter,
    fmt::{
      self,
    },
    layer::SubscriberExt,
  },
};

pub struct Logger {
  _guards: VecDeque<WorkerGuard>,
}

impl Logger {
  pub fn init() -> anyhow::Result<Self> {
    // taken from github https://github.com/rust-cli/env_logger/issues/125
    // let file_target = Box::new(File::create("ventana.log").unwrap());

    let mut _guards = VecDeque::new();

    let filter = EnvFilter::builder()
      .with_default_directive(LevelFilter::TRACE.into())
      .from_env()?
      .add_directive("wgpu=off".parse()?)
      .add_directive("naga=off".parse()?);

    let (std_out, guard) = tracing_appender::non_blocking(std::io::stdout());
    _guards.push_back(guard);
    let (file, guard) = tracing_appender::non_blocking(
      tracing_appender::rolling::Builder::new()
        .filename_prefix("out")
        .filename_suffix("log")
        .max_log_files(1)
        .rotation(Rotation::NEVER)
        .build("./logs")?,
    );
    _guards.push_back(guard);
    // let writers = std_out.and(file);

    let subscriber = tracing_subscriber::registry()
      .with(filter)
      .with(fmt::Layer::new().with_writer(std_out).with_thread_names(true))
      .with(
        fmt::Layer::new()
          .with_writer(file)
          .with_ansi(false)
          .with_thread_names(true),
      );

    LogTracer::init()?;

    tracing::subscriber::set_global_default(subscriber)?;

    // tracing_subscriber::fmt()
    //   .with_env_filter(filter)
    //   .with_writer(writers)
    //   .with_ansi(false)
    //   .compact()
    //   .with_thread_names(true)
    //   .init();

    // env_logger::builder()
    // // .target(Target::Pipe(file_target))
    // .filter(None, log::LevelFilter::Trace)
    // .filter(Some("wgpu"), log::LevelFilter::Off)
    // .filter(Some("naga"), log::LevelFilter::Off)
    // .format(|buf, record| {
    //   writeln!(
    //     buf,
    //     "[{} {} {}:{}] {}",
    //     Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
    //     record.level(),
    //     // record.thread_name().unwrap_or("unknown"),
    //     record.file().unwrap_or("unknown"),
    //     record.line().unwrap_or(0),
    //     record.args()
    //   )
    // })
    // .init();

    Ok(Self { _guards })
  }
}
