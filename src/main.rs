mod config;
mod dest_console;
mod dest_sqlite;
mod dest_web;
mod middleware_cmd;
mod source;
mod sqlite;
mod src_dummy;
mod src_irc;
mod src_stdin;
mod src_twitch;
mod src_yt;

use anyhow::Result;
use clap::{command, Arg, Command};
use dest_sqlite::SqliteDestination;
use dest_web::WebDestination;

use middleware_cmd::{ActivatedCommands, CommandMiddleware};
use serde::{Deserialize, Serialize};
use sqlite::get_database;

use src_dummy::DummySource;
use src_irc::IrcSource;
use src_stdin::StdinSource;
use src_twitch::TwitchSource;
use src_yt::YoutubeSource;
use tokio::{sync::broadcast::channel, task::JoinSet};

use crate::dest_console::ConsoleDestination;

use strum_macros::Display;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(tag = "module", content = "value")]
pub enum ModuleConfig {
    YoutubeSource(String),
    TwitchSource(String),
    IrcSource {
        nick_name: String,
        server: String,
        channel: String,
    },
    DummySource,
    StdinSource,
    ConsoleDest,
    WebDest {
        interface: String,
        port: u16,
    },
    SqliteDest,
    CommandMiddleware(Vec<ActivatedCommands>),
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .propagate_version(true)
        .arg(
            Arg::new("config_file")
                .short('c')
                .long("config")
                .value_name("FILE"),
        )
        .arg(Arg::new("db_file").long("db").value_name("FILE"))
        .subcommand(Command::new("init").about("Initialize stream-chat.toml config file"))
        .get_matches();

    let config_file = matches.get_one::<String>("config_file").map(|x| &**x);
    let db_file = matches.get_one::<String>("db_file").map(|x| &**x);

    let res = match matches.subcommand() {
        None => run(config_file, db_file).await,
        Some(("init", _)) => config::init(config_file),
        _ => unreachable!(""),
    };
    eprintln!("DONE: {:?}", res);
    Ok(())
}

async fn run(config_file: Option<&str>, db_file: Option<&str>) -> Result<()> {
    let config = config::load_config(config_file)?;
    let db = get_database(db_file).await?;
    let (tx, rx) = channel(128);

    let mut join_set = JoinSet::new();

    for c in config {
        match c {
            ModuleConfig::TwitchSource(ref channel_name) => {
                let twitch = TwitchSource::new(tx.clone(), channel_name.to_string())
                    .await?
                    .run();
                join_set.spawn(twitch);
            }
            ModuleConfig::YoutubeSource(ref channel_name) => {
                let yt = YoutubeSource::new(tx.clone(), channel_name.to_string())
                    .await?
                    .run();
                join_set.spawn(yt);
            }
            ModuleConfig::IrcSource {
                ref nick_name,
                ref server,
                ref channel,
            } => {
                let irc = IrcSource::new(
                    tx.clone(),
                    nick_name.to_string(),
                    server.to_string(),
                    channel.to_string(),
                )
                .await?
                .run();
                join_set.spawn(irc);
            }
            ModuleConfig::DummySource => {
                let dummy = DummySource::new(tx.clone()).await?.run();
                join_set.spawn(dummy);
            }
            ModuleConfig::StdinSource => {
                let stdin = StdinSource::new(tx.clone()).await?.run();
                join_set.spawn(stdin);
            }
            ModuleConfig::WebDest { ref interface, ref port } => {
                let termjs = WebDestination::new(tx.clone(), interface, port.clone()).run();
                join_set.spawn(termjs);
            }
            ModuleConfig::ConsoleDest => {
                let console = ConsoleDestination::new(rx.resubscribe()).run();
                join_set.spawn(console);
            }
            ModuleConfig::SqliteDest => {
                let sqlite = SqliteDestination::new(rx.resubscribe(), &db).run();
                join_set.spawn(sqlite);
            }
            ModuleConfig::CommandMiddleware(ref cmds) => {
                let cmd = CommandMiddleware::new(tx.clone(), rx.resubscribe(), &cmds).run();
                join_set.spawn(cmd);
            }
        }
        println!(" - Loaded module {}", c);
    }

    while let Some(res) = join_set.join_next().await {
        let res: Result<String> = res.map_err(|e| anyhow::Error::from(e)).and_then(|v| v);
        match res {
            Err(e) => eprintln!("error: {}", e),
            Ok(s) => eprintln!("source {:?} finished running", s),
        }
    }

    join_set.shutdown().await;
    return Ok(());
}
