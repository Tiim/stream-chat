# Chat in Console

`stream-chat` is a tiny rust program that connects to various chat systems and outputs the chat log either 
to the console or in a web page.
It's built to be modular. It is easy to extend addidional chat systems and places to output the chats.
Currently the following modules are implemented:

- Youtube live chat source
- Twitch chat source
- IRC source
- Terminal output
- Web output


The configuration currently lives inside of [`main.rs`](src/main.rs). Reading the configuration from a config file is planned.


## Usage

- clone this repo using `git clone`
- edit the settings in `main.rs` to fit your needs.
- compile and install with `cargo install`.
- run the program: `stream-chat`.
