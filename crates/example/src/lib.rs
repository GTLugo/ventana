pub fn initialize_logger() {
  env_logger::builder()
    .filter(None, log::LevelFilter::Trace)
    .format_source_path(true)
    .init();
}
