mod planet;
mod sector;
mod star;
mod planetsurface;
mod helpers;
mod task;
mod tile;
mod sectormap;
extern crate common;

use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::io::Read;
use tokio_native_tls::{TlsStream};
use tokio_util::codec::{Framed, LinesCodec};
use tokio_stream::StreamExt;
use futures::sink::SinkExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    log::set_max_level(log::LevelFilter::Trace);
    let addr = "0.0.0.0:28097".to_string();
    
    let listener = TcpListener::bind(&addr).await?;

    let der = include_bytes!("../identity.pfx");
    let cert = native_tls::Identity::from_pkcs12(der, "").unwrap();

    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(
        native_tls::TlsAcceptor::builder(cert).build().unwrap()
    );

    tokio::spawn(async move {
        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            let tls_acceptor = tls_acceptor.clone();

            tokio::spawn(async move {
                let tls_stream = tls_acceptor.accept(stream).await.expect("Accept error");
                if let Err(e) = handle_client(tls_stream, addr).await {
                    println!("An error occurred: {:?}", e);
                }
            });
        }
    });
    loop {}
}

fn handle_request(request: &json::JsonValue) -> json::JsonValue {
    json::object!{}
}

async fn handle_client(stream: TlsStream<TcpStream>, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    log::info!("Connected by {:?}", addr);
    let mut lines = Framed::new(stream, LinesCodec::new());
    
    while let Some(Ok(result)) = lines.next().await {
        let parsed = json::parse(&result);
        log::debug!("Read {} bytes", result.len());
        if let Ok(parsed) = parsed {

            let mut responses = json::object!{
                requests: parsed["requests"].clone(),
                results: json::array![]
            };

            if !parsed["requests"].is_array() {
                log::error!("requests attribute was not an array!");
            } else {
                for request in parsed["requests"].members() {
                    let response = handle_request(request);
                    responses["results"].push(response);
                }
            }
            lines.send(responses.dump()).await?;
        } else {
            log::error!("Parse error in request json");
            lines.send(json::object!{status: -1}.dump()).await?;
        }
    }
    Ok(())
}
