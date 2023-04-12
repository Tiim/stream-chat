mod dest_console;
mod source;
mod src_irc;
mod src_twitch;
mod src_yt;

use anyhow::Result;
use source::ChatSource;
use src_irc::IrcSource;
use src_twitch::TwitchSource;
use src_yt::YoutubeSource;
use tokio::{sync::mpsc::unbounded_channel, task::JoinSet};

use crate::dest_console::ConsoleDestination;

enum SourceConfig {
    Youtube(String),
    Twitch(String),
    IRC {
        nick_name: String,
        server: String,
        channel: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // let config = vec![
    //     SourceConfig::Youtube("@LofiGirl".to_string()),
    //     SourceConfig::Twitch("shroud".to_string()),
    // ];

    let config = vec![
        SourceConfig::Youtube("@Tiim".to_string()),
        SourceConfig::IRC {
            nick_name: "stream-chat".to_owned(),
            server: "irc.libera.chat".to_owned(),
            channel: "##tiim".to_owned(),
        },
        SourceConfig::Twitch("tiim_b".to_string()),
    ];

    let (tx, rx) = unbounded_channel();

    let mut join_set = JoinSet::new();

    for c in config {
        match c {
            SourceConfig::Twitch(channel_name) => {
                let twitch = TwitchSource::new(tx.clone(), channel_name).await?.run();
                join_set.spawn(twitch);
            }
            SourceConfig::Youtube(channel_name) => {
                let yt = YoutubeSource::new(tx.clone(), channel_name).await?.run();
                join_set.spawn(yt);
            }
            SourceConfig::IRC {
                nick_name,
                server,
                channel,
            } => {
                let irc = IrcSource::new(tx.clone(), nick_name, server, channel)
                    .await?
                    .run();
                join_set.spawn(irc);
            }
        }
    }

    let console = ConsoleDestination::new(rx).run();
    join_set.spawn(console);

    while let Some(res) = join_set.join_next().await {
        let res: Result<String> = res.map_err(|e| anyhow::Error::from(e) ).and_then(|v| v);
        match res {
            Err(e) => eprintln!("error: {}", e),
            Ok(s) => eprintln!("source {:?} finished running", s),
        }
    }

    return Ok(());
}
