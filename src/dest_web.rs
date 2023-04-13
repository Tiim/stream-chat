use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use crate::source::Event;
use anyhow::Result;
use axum::{response::sse::Event as SSEvent, Extension};
use axum::{
    response::{sse::KeepAlive, Html, Sse},
    routing::get,
    Router,
};
use futures::stream::{unfold, Stream};
use futures::StreamExt;
use std::str::FromStr;
use tokio::sync::broadcast::Sender;

pub struct WebDestination {
    tx: Sender<Event>,
    address: SocketAddr,
}

impl WebDestination {
    pub fn new(tx: Sender<Event>, host: &str, port: u16) -> Self {
        let ip_addr = IpAddr::V4(Ipv4Addr::from_str(host).unwrap());
        let address = SocketAddr::new(ip_addr, port);
        WebDestination { tx, address }
    }
    pub async fn run(self) -> Result<String> {
        let app = Router::new()
            .route("/", get(index))
            .route("/sse", get(sse_handler))
            .route("/script.js", get(js))
            .layer(Extension(self.tx));

        axum::Server::bind(&self.address)
            .serve(app.into_make_service())
            .await?;
        return Ok("".to_owned());
    }
}

async fn js() -> &'static [u8] {
    include_bytes!("dest_web_index.js")
}
async fn index() -> Html<&'static [u8]> {
    Html(include_bytes!("dest_web_index.html"))
}

// debug with curl -N http://localhost:8080/sse
async fn sse_handler(
    Extension(tx): Extension<Sender<Event>>,
) -> Sse<impl Stream<Item = Result<SSEvent, serde_json::Error>>> {
    // let rx = frx();
    let rx = tx.subscribe();
    let stream = unfold((true, rx), |(first, mut r)| async move {
        if first {
            return Some((Event::Info { msg: "Connected".to_string(), src: None }, (false, r)));
        }
        match r.recv().await {
            Ok(value) => Some((value, (false,r))),
            Err(_) => None,
        }
    })
    .map(|e| serde_json::to_string(&e))
    .map(|e| e.map(|ev|SSEvent::default().data(ev)));
    Sse::new(stream).keep_alive(KeepAlive::default())
}
