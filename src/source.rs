use chrono::Utc;

#[derive(Debug)]
pub enum ChatSource {
    YoutubeLive,
    Twitch,
    IRC
}

#[derive(Debug)]
pub enum Event {
    Chat { chat: ChatEvent },
    Info { msg: String, src: Option<ChatSource> },
    Error { err: String },
}

#[derive(Debug)]
pub struct ChatEvent {
    pub src: ChatSource,
    pub ts: chrono::DateTime<Utc>,
    pub author: String,
    pub message: String,
}
