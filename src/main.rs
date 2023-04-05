mod console_dest;
mod destination;
mod source;
mod yt;

use std::env;

use anyhow::Result;
use source::Source;
use tokio::sync::mpsc::unbounded_channel;

use crate::{console_dest::ConsoleDestination, destination::Dest};

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

    let yt = yt::YoutubeSource::new(tx, stream_url.as_str())?.run();
    let console = ConsoleDestination::new(rx).run();

    tokio::join!(yt, console); // add other sources here too.

    return Ok(());
}
