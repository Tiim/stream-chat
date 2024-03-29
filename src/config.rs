use std::{
    fs::File,
    io::{Read, Write}, path::PathBuf, str::FromStr,
};

use crate::{middleware_cmd::ActivatedCommands, ModuleConfig};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

const CONFIG_NAME: &str = "stream-chat.toml";

#[derive(Debug, Serialize, Deserialize)]
struct Cfg {
    config: Vec<ModuleConfig>,
}

pub fn load_config(config_file: Option<&str>) -> Result<Vec<ModuleConfig>> {
    let config_file = match config_file {
        None => {
            let dirs = BaseDirectories::new()?;
            let config_file = dirs.place_config_file(CONFIG_NAME)?;
            config_file
        }
        Some(cf) => PathBuf::from_str(cf)?,
    };
    let mut file = File::open(&config_file)?;
    let mut str = String::new();
    file.read_to_string(&mut str)
        .context("Failed to read config. Do you need to initialize with the init subcommand?")?;
    let res: Cfg = toml::from_str(str.as_str()).context("Can't parse TOML config file.")?;

    return Ok(res.config);
}

pub fn init(config_file: Option<&str>) -> Result<()> {
    let config = vec![
        // ModuleConfig::YoutubeSource("@Tiim".to_string()),
        ModuleConfig::IrcSource {
            nick_name: "stream-chat".to_owned(),
            server: "irc.libera.chat".to_owned(),
            channel: "##tiim".to_owned(),
        },
        ModuleConfig::TwitchSource("tiim_b".to_string()),
        // ModuleConfig::DummySource,
        ModuleConfig::WebDest {
            interface: "127.0.0.1".to_string(),
            port: 10888,
        },
        ModuleConfig::ConsoleDest,
        ModuleConfig::SqliteDest,
        ModuleConfig::CommandMiddleware(vec![ActivatedCommands::TTS { max_length: 600 }]),
    ];

    let config_file = match config_file {
        None => {
            let dirs = BaseDirectories::new()?;
            let config_file = dirs.place_config_file(CONFIG_NAME)?;
            config_file
        }
        Some(cf) => PathBuf::from_str(cf)?,
    };

    let mut file = File::create(&config_file)?;
    let res =
        toml::to_string_pretty::<Cfg>(&Cfg { config }).context("Failed to serialize to TOML")?;
    file.write(res.as_bytes())?;

    return Ok(());
}
