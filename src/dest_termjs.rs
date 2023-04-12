use std::{
    convert::Infallible,
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

pub struct TermjsDestination {
    tx: Sender<Event>,
    address: SocketAddr,
}

impl TermjsDestination {
    pub fn new(tx: Sender<Event>, host: &str, port: u16) -> Self {
        let ip_addr = IpAddr::V4(Ipv4Addr::from_str(host).unwrap());
        let address = SocketAddr::new(ip_addr, port);
        TermjsDestination { tx, address }
    }
    pub async fn run(self) -> Result<String> {
        let app = Router::new()
            .route("/", get(index))
            .route("/sse", get(sse_handler))
            .layer(Extension(self.tx));

        axum::Server::bind(&self.address)
            .serve(app.into_make_service())
            .await?;
        return Ok("".to_owned());
    }
}

async fn index() -> Html<&'static [u8]> {
    Html(include_bytes!("dest_termjs_index.html"))
}

// debug with curl -N http://localhost:8080/sse
async fn sse_handler(
    Extension(tx): Extension<Sender<Event>>,
) -> Sse<impl Stream<Item = Result<SSEvent, Infallible>>> {
    // let rx = frx();
    let rx = tx.subscribe();
    let stream = unfold(rx, |mut r| async move {
        match r.recv().await {
            Ok(value) => Some((value, r)),
            Err(_) => None,
        }
    })
    .map(|e| format!("{:?}", e))
    .map(|e| Ok(SSEvent::default().data(e)));
    Sse::new(stream).keep_alive(KeepAlive::default())
}
