mod dest_console;
mod dest_web;
mod source;
mod src_dummy;
mod src_irc;
mod src_twitch;
mod src_yt;
mod middleware_cmd;

use anyhow::Result;
use dest_web::WebDestination;
use irc::proto::Command;
use middleware_cmd::CommandMiddleware;
use src_dummy::DummySource;
use src_irc::IrcSource;
use src_twitch::TwitchSource;
use src_yt::YoutubeSource;
use tokio::{sync::broadcast::channel, task::JoinSet};

use crate::dest_console::ConsoleDestination;

#[allow(dead_code)]
enum ModuleConfig {
    YoutubeSource(String),
    TwitchSource(String),
    IrcSource {
        nick_name: String,
        server: String,
        channel: String,
    },
    DummySource,
    ConsoleDest,
    WebDest,
    CommandMiddleware,
}

#[tokio::main]
async fn main() {
    let res = run().await;

    eprintln!("DONE: {:?}", res);
}

async fn run() -> Result<()> {
    let config = vec![
        // ModuleConfig::YoutubeSource("@Tiim".to_string()),
        ModuleConfig::IrcSource {
            nick_name: "stream-chat".to_owned(),
            server: "irc.libera.chat".to_owned(),
            channel: "##tiim".to_owned(),
        },
        ModuleConfig::TwitchSource("tiim_b".to_string()),
        // ModuleConfig::DummySource,
        ModuleConfig::WebDest,
        ModuleConfig::ConsoleDest,
        ModuleConfig::CommandMiddleware,
    ];

    // let config = vec![
    //     SourceConfig::Youtube("@LofiGirl".to_string()),
    //     SourceConfig::Twitch("shroud".to_string()),
    // ];

    let (tx, rx) = channel(32);

    let mut join_set = JoinSet::new();

    for c in config {
        match c {
            ModuleConfig::TwitchSource(channel_name) => {
                let twitch = TwitchSource::new(tx.clone(), channel_name).await?.run();
                join_set.spawn(twitch);
            }
            ModuleConfig::YoutubeSource(channel_name) => {
                let yt = YoutubeSource::new(tx.clone(), channel_name).await?.run();
                join_set.spawn(yt);
            }
            ModuleConfig::IrcSource {
                nick_name,
                server,
                channel,
            } => {
                let irc = IrcSource::new(tx.clone(), nick_name, server, channel)
                    .await?
                    .run();
                join_set.spawn(irc);
            }
            ModuleConfig::DummySource => {
                let dummy = DummySource::new(tx.clone()).await?.run();
                join_set.spawn(dummy);
            }
            ModuleConfig::WebDest => {
                let termjs = WebDestination::new(tx.clone(), "127.0.0.1", 10888).run();
                join_set.spawn(termjs);
            }
            ModuleConfig::ConsoleDest => {
                let console = ConsoleDestination::new(rx.resubscribe()).run();
                join_set.spawn(console);
            }
            ModuleConfig::CommandMiddleware => {
                let cmd = CommandMiddleware::new(tx.clone(), rx.resubscribe()).run();
                join_set.spawn(cmd);
            }
        }
    }

    while let Some(res) = join_set.join_next().await {
        let res: Result<String> = res.map_err(|e| anyhow::Error::from(e)).and_then(|v| v);
        match res {
            Err(e) => eprintln!("error: {}", e),
            Ok(s) => eprintln!("source {:?} finished running", s),
        }
    }

    return Ok(());
}
