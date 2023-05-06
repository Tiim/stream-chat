use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatSource {
    YoutubeLive,
    Twitch,
    IRC,
    Dummy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
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
    Command {
        cmd: Command
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cmd", content = "value")]
pub enum Command {
    TTS(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEvent {
    pub src: ChatSource,
    pub ts: chrono::DateTime<Utc>,
    pub author: String,
    pub message: String,
}
