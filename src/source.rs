use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatSource {
    YoutubeLive,
    Twitch,
    IRC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Chat {
        chat: ChatEvent,
    },
    Info {
        msg: String,
        src: Option<ChatSource>,
    },
    Error {
        err: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEvent {
    pub src: ChatSource,
    pub ts: chrono::DateTime<Utc>,
    pub author: String,
    pub message: String,
}
