use chrono::Utc;
use tokio::task;

pub enum Event {
    Chat { chat: ChatEvent },
    Error { err: String },
    FatalError { err: String },
}

#[derive(Debug)]
pub struct ChatEvent {
    pub ts: chrono::DateTime<Utc>,
    pub author: String,
    pub message: String,
}

pub trait Source {
    fn run(self) -> task::JoinHandle<()>;
}
