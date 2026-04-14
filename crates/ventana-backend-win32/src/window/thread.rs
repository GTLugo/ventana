use {
  super::{
    command::Command,
    state::SharedInternal,
  },
  crate::{
    event::map_native_event,
    window::command::CommandResponse,
  },
  std::sync::Arc,
  ventana_hal::{
    event::{
      Event,
      WindowEvent,
    },
    thread::{
      command::CommandEnvelope,
      response::ResponseEnvelope,
      server::Server,
    },
  },
  win64::prelude::*,
};

pub struct Procedure {
  pub server: Arc<Server<Command, CommandResponse>>,
  pub internal: Arc<SharedInternal>,
}

impl Procedure {
  fn send_event(&self, event: Event) {
    self.server.send_event(event, self.internal.is_ready()).unwrap();
  }
}

impl WindowProcedure for Procedure {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    // log::trace!("Received message: `{:?}`", message);

    let event = map_native_event(message);

    if self.server.are_commands_pending() {
      // log::trace!("Commands are pending, processing them before handling the message");
      while let Some(CommandEnvelope { id, command }) = self.server.receive_command() {
        log::trace!("Received command in window procedure: `{command:?}`");
        match command {
          Command::Destroy => {
            self.internal.set_ready(false);
            window.destroy().unwrap();
            self
              .server
              .send_response(ResponseEnvelope {
                id,
                response: CommandResponse::Success,
              })
              .unwrap();
          },
          Command::Redraw => {
            window.redraw().unwrap();
            self
              .server
              .send_response(ResponseEnvelope {
                id,
                response: CommandResponse::Success,
              })
              .unwrap();
          },
          Command::GetWindowText => {
            self
              .server
              .send_response(ResponseEnvelope {
                id,
                response: CommandResponse::GetWindowText(window.get_window_text().unwrap_or_default()),
              })
              .unwrap();
          },
          _ => (),
        }
      }
    }

    match (message, event) {
      (Message::Create(_), _) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      },
      (Message::SettingChange(_), _) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      },
      (Message::Close, _) => {
        self.send_event(Event::Window(WindowEvent::CloseRequest));
        Some(LResult(0)) // We don't want defwindowproc to run since it'll auto-destroy the window
      },
      (Message::Destroy, _) => {
        window.quit();
        None
      },
      (_, Some(event)) => {
        // log::trace!("{window:?} | {message:?} | {event:?}");
        self.send_event(Event::Window(event));
        None
      },
      _ => None,
    }
  }
}
