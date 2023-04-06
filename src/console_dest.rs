use crate::destination::Dest;
use crate::source::{Event, ChatEvent};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task;

pub struct ConsoleDestination {
    rx: UnboundedReceiver<Event>,
}

impl ConsoleDestination {
    pub fn new(rx: UnboundedReceiver<Event>) -> Self {
        ConsoleDestination { rx }
    }
}

impl Dest for ConsoleDestination {
    fn run(mut self) -> task::JoinHandle<()> {
        task::spawn(async move {
            while let Some(event) = self.rx.recv().await {
                match event {
                    Event::Chat { chat } => print_chat(chat),
                    Event::Error { err } => println!("{}", err),
                    Event::FatalError { err } => todo!("{}", err),
                }
            }
        })
    }
}


fn print_chat( ce: ChatEvent) {
    println!("<{}> {}", ce.author, ce.message);
}
