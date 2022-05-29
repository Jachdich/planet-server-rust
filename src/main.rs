mod planet;
mod sector;
mod star;
mod planetsurface;
mod helpers;
mod task;
mod tile;
mod sectormap;
mod generation;
extern crate common;

use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::io::Read;
use tokio_native_tls::{TlsStream};
use tokio_util::codec::{Framed, LinesCodec};
use tokio_stream::StreamExt;
use futures::sink::SinkExt;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::sectormap::SectorMap;
use crate::sector::Sector;

//TODO in enums
const ERR_OK: i32 = 0;
const ERR_MALFORMED_JSON: i32 = -1;
const ERR_INVALID_REQUEST: i32 = -2;
const ERR_OUT_OF_BOUNDS: i32 = -3;
const ERR_INVALID_ACTION: i32 = -4;
const ERR_INVALID_CREDENTIALS: i32 = -5;
const ERR_NOT_AUTHENTICATED: i32 = -6;
const ERR_NOT_AUTHORISED: i32 = -7;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let gencfg = generation::GenParams::load_from("generation.json")?;
    println!("{:?}", gencfg);
    env_logger::init();
    log::set_max_level(log::LevelFilter::Trace);
    let addr = "0.0.0.0:28097".to_string();

    let listener = TcpListener::bind(&addr).await?;
    log::info!("Listening on {}", &addr);
    
    let der = include_bytes!("../identity.pfx");
    let cert = native_tls::Identity::from_pkcs12(der, "").unwrap();

    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(
        native_tls::TlsAcceptor::builder(cert).build().unwrap()
    );

    let state = Arc::new(Mutex::new(SectorMap::new()));
    
    let net_state = Arc::clone(&state);
    tokio::spawn(async move {
        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            let tls_acceptor = tls_acceptor.clone();

            let connection_state = Arc::clone(&net_state);
            tokio::spawn(async move {
                let tls_stream = tls_acceptor.accept(stream).await.expect("Accept error");
                if let Err(e) = handle_client(connection_state, tls_stream, addr).await {
                    println!("An error occurred: {:?}", e);
                }
            });
        }
    });
    loop {}
}

async fn handle_request(request: &json::JsonValue, map: &Arc<Mutex<SectorMap>>) -> json::JsonValue {
    if !request["request"].is_string() {
        log::warn!("'request' attribute is not a string");
        return json::object!{
            status: ERR_MALFORMED_JSON
        };
    }
    match request["request"].as_str().unwrap() {
        "getSector" => {
            if !request["x"].is_number() || !request["y"].is_number() {
                return json::object!{ status: ERR_MALFORMED_JSON };
            }
            
            let x = request["x"].as_i32().unwrap();
            let y = request["y"].as_i32().unwrap();
            let mut map_lock = map.lock().await;
            let sec = map_lock.get_sector_at(x, y);
            //TODO any way to do this without dumping and parsing the data?
            let sec_json_str = serde_json::to_string(&sec).unwrap();
            let res = json::parse(&sec_json_str).unwrap();
            json::object!{
                status: ERR_OK,
                result: res
            }
        }
        _ => json::object!{ status: ERR_INVALID_REQUEST }
    }
}

async fn handle_client(map: Arc<Mutex<SectorMap>>, 
                       stream: TlsStream<TcpStream>,
                       addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    log::info!("Connected by {:?}", addr);
    let mut lines = Framed::new(stream, LinesCodec::new());
    
    while let Some(Ok(result)) = lines.next().await {
        log::debug!("{}", result);
        let parsed = json::parse(&result);
        log::debug!("Read {} bytes", result.len());
        
        let parsed = if let Ok(parsed) = parsed { 
            parsed
        } else {
            log::warn!("Parse error in request json");
            lines.send(json::object!{status: -1}.dump()).await?;
            json::object!{}
        };

        if !parsed["requests"].is_array() {
            log::warn!("'requests' attribute was not an array");
            lines.send(json::object!{status: -1}.dump()).await?;
            continue;
        }
        
        let mut responses = json::object!{
            requests: parsed["requests"].clone(),
            results: json::array![]
        };
        
        for request in parsed["requests"].members() {
            let response = handle_request(request, &map).await;
            responses["results"].push(response);
        }
        
        lines.send(responses.dump()).await?;
    }
    Ok(())
}
