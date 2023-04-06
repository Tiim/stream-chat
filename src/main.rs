mod dest_console;
mod source;
mod src_twitch;
mod src_yt;

use std::env;

use anyhow::Result;
use src_twitch::TwitchSource;
use src_yt::YoutubeSource;
use tokio::sync::mpsc::unbounded_channel;

use crate::dest_console::ConsoleDestination;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let stream_url: String;
    if args.len() == 1 {
        println!("WARNING: using default stream url!");
        stream_url = "https://www.youtube.com/watch?v=jfKfPfyJRdk".to_string()
    } else if args.len() > 2 {
        println!("Usage {} <Stream Url>", args[0]);
        return Err(anyhow::format_err!("too many args"));
    } else {
        stream_url = args[1].to_string();
    }
    let (tx, rx) = unbounded_channel();

    let yt = YoutubeSource::new(tx.clone(), stream_url.as_str())?.run();
    let twitch = TwitchSource::new(tx, "xrohat".to_string())?.run();
    let console = ConsoleDestination::new(rx).run();

    let res = tokio::try_join!(yt, twitch, console); // add other sources and destinations here too.

    if let Err(e) = res {
        println!("error: {}", e)
    }

    return Ok(());
}
