use super::command::CommandId;

#[derive(Debug, Clone)]
pub struct ResponseEnvelope<ThreadResponse> {
  pub id: CommandId,
  pub response: ThreadResponse,
}
