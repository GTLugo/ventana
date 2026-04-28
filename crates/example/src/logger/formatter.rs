#![allow(unused)]

use {
  core::fmt,
  tracing_subscriber::fmt::{
    format::Writer,
    time::SystemTime,
  },
};

#[derive(Debug, Clone)]
pub struct VentanaFormat;

impl VentanaFormat {
  // #[inline]
  // fn format_timestamp(&self, writer: &mut Writer<'_>) -> fmt::Result {
  //   if writer.has_ansi_escapes() {
  //     let style = Style::new().dimmed();
  //     write!(writer, "{}", style.prefix())?;

  //     // If getting the timestamp failed, don't bail --- only bail on
  //     // formatting errors.
  //     if self.timer.format_time(writer).is_err() {
  //       writer.write_str("<unknown time>")?;
  //     }

  //     write!(writer, "{} ", style.suffix())?;
  //     return Ok(());
  //   }

  //   // Otherwise, just format the timestamp without ANSI formatting.
  //   // If getting the timestamp failed, don't bail --- only bail on
  //   // formatting errors.
  //   if self.timer.format_time(writer).is_err() {
  //     writer.write_str("<unknown time>")?;
  //   }
  //   writer.write_char(' ')
  // }
}
