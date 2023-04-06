use chrono::Utc;

#[derive(Debug)]
pub enum ChatSource {
    YoutubeLive,
    Twitch,
}

#[derive(Debug)]
pub enum Event {
    Chat { chat: ChatEvent },
    Error { err: String },
    FatalError { err: String },
}

#[derive(Debug)]
pub struct ChatEvent {
    pub src: ChatSource,
    pub ts: chrono::DateTime<Utc>,
    pub author: String,
    pub message: String,
}
