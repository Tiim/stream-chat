mod config;
mod dest_console;
mod dest_web;
mod middleware_cmd;
mod source;
mod src_dummy;
mod src_irc;
mod src_twitch;
mod src_yt;

use anyhow::Result;
use clap::{command, Command};
use dest_web::WebDestination;

use middleware_cmd::{ActivatedCommands, CommandMiddleware};
use serde::{Deserialize, Serialize};
use src_dummy::DummySource;
use src_irc::IrcSource;
use src_twitch::TwitchSource;
use src_yt::YoutubeSource;
use tokio::{sync::broadcast::channel, task::JoinSet};

use crate::dest_console::ConsoleDestination;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "module", content="value")]
pub enum ModuleConfig {
    YoutubeSource(String),
    TwitchSource(String),
    IrcSource {
        nick_name: String,
        server: String,
        channel: String,
    },
    DummySource,
    ConsoleDest,
    WebDest {
        interface: String,
        port: u16,
    },
    CommandMiddleware(Vec<ActivatedCommands>),
}

#[tokio::main]
async fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand(Command::new("init").about("Initialize stream-chat.toml config file"))
        .get_matches();

    let res = match matches.subcommand() {
        None => run().await,
        Some(("init", _)) => config::init(),
        _ => unreachable!(""),
    };
    eprintln!("DONE: {:?}", res);
}

async fn run() -> Result<()> {
    let config = config::load_config()?;

    let (tx, rx) = channel(128);

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
            ModuleConfig::WebDest { interface, port } => {
                let termjs = WebDestination::new(tx.clone(), &interface, port).run();
                join_set.spawn(termjs);
            }
            ModuleConfig::ConsoleDest => {
                let console = ConsoleDestination::new(rx.resubscribe()).run();
                join_set.spawn(console);
            }
            ModuleConfig::CommandMiddleware(cmds) => {
                let cmd = CommandMiddleware::new(tx.clone(), rx.resubscribe(), cmds).run();
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
