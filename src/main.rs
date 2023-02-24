extern crate lazy_static;
mod generation;
mod helpers;
mod planet;
mod planetsurface;
mod sector;
mod sectormap;
mod star;
mod task;
mod tile;
extern crate common;

use futures::sink::SinkExt;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::{json, Value};
use std::error::Error;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_native_tls::TlsStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use crate::helpers::SurfaceLocator;
use crate::sectormap::SectorMap;

use noise::{Seedable, Perlin, Simplex, OpenSimplex, NoiseFn};

enum Err {
    Ok = 0,
    MalformedJson = -1,
    InvalidRequest = -2,
    OutOfBounds = -3,
    InvalidAction = -4,
    InvalidCredentials = -5,
    NotAuthenticated = -6,
    NotAuthorised = -7,
}

type ArcMap = Arc<Mutex<SectorMap>>;

#[derive(Deserialize)]
pub struct Config {
    pub addr: String,
    pub port: u16,
    pub save_name: String,
}

lazy_static! {
    pub static ref CONF: Config = {
        let mut file = std::fs::File::open("config.json").expect("Couldn't find config.json!");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Couldn't read config.json!");
        let cfg: Config = serde_json::from_str(&data).expect("config.json is not valid json!");
        cfg
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let gencfg = generation::GenParams::load_from("generation.json")?;

    env_logger::init();
    log::set_max_level(log::LevelFilter::Trace);
    let addr = format!("{}:{}", CONF.addr, CONF.port);

    let listener = TcpListener::bind(&addr).await?;
    log::info!("Listening on {}", &addr);

    // let der = include_bytes!("../identity.pfx");
    // let cert = native_tls::Identity::from_pkcs12(der, "").unwrap();
    
    let mut cert_file = std::fs::File::open("cert.pem").unwrap();
    let mut certs = vec![];
    cert_file.read_to_end(&mut certs).unwrap();
    let mut key_file = std::fs::File::open("key.pem").unwrap();
    let mut key = vec![];
    key_file.read_to_end(&mut key).unwrap();

    let pkcs8 = native_tls::Identity::from_pkcs8(&certs, &key).unwrap();
    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(
        native_tls::TlsAcceptor::builder(pkcs8).build().unwrap(),
    );

    let state = Arc::new(Mutex::new(SectorMap::new(gencfg)));

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
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

async fn handle_request(request: &Value, map: &ArcMap) -> Vec<Value> {
    if !request["request"].is_string() {
        log::warn!("'request' attribute is not a string");
        return vec![json!({ "status": Err::MalformedJson as i32 })];
    }
    match request["request"].as_str().unwrap() {
        "getSector" => {
            if !request["x"].is_number() || !request["y"].is_number() {
                return vec![json!({ "status": Err::MalformedJson as i32 })];
            }

            let x = request["x"].as_i64().unwrap() as i32;
            let y = request["y"].as_i64().unwrap() as i32;
            let mut map_lock = map.lock().await;
            let sec = map_lock.get_sector(x, y);
            vec![json!({
                "status": Err::Ok as i32,
                "result": sec,
                "x": x,
                "y": y,
                "request": "getSector",
            })]
        }
        "getPlanets" => {
            let loc: SurfaceLocator = serde_json::from_value(request.clone()).unwrap();
            let mut map_lock = map.lock().await;
            let gen = map_lock.gen.clone();
            let star = map_lock.get_star_mut(&loc).unwrap(); //TODO handle this
            let mut results = Vec::new();
            // for task in &surface.tasks {
            //     let mut res_json = serde_json::to_value(&task).unwrap();
            //     res_json["request"] = "setTimer".into();
            //     results.push(res_json);
            // }
            results.push(json!({
                "request": "getPlanets",
                "loc": serde_json::to_value(&loc).unwrap(),
                "result": serde_json::to_value(star.gen_planets(&gen, loc)).unwrap(),
            }));

            results
        }
        req => {
            log::warn!("Unknown request '{}'", req);
            vec![json!({ "request": req, "status": Err::InvalidRequest as i32 })]
        },
    }
}

async fn handle_client(
    map: ArcMap,
    stream: TlsStream<TcpStream>,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    log::info!("Connected by {:?}", addr);
    let mut lines = Framed::new(stream, LinesCodec::new());

    while let Some(Ok(result)) = lines.next().await {
        log::debug!("{}", result);
        let parsed = serde_json::from_str(&result);
        log::debug!("Read {} bytes", result.len());

        let parsed = if let Ok(parsed) = parsed {
            parsed
        } else {
            log::warn!("Parse error in request json");
            lines
                .send(json!({ "status": Err::MalformedJson as i32 }).to_string())
                .await?;
            json!({})
        };

        if !parsed["requests"].is_array() {
            log::warn!("'requests' attribute was not an array");
            lines
                .send(json!({ "status": Err::MalformedJson as i32 }).to_string())
                .await?;
            continue;
        }

        let mut results_vec: Vec<Value> = Vec::new();
        for request in parsed["requests"].as_array().unwrap() {
            let mut responses = handle_request(request, &map).await;
            results_vec.append(&mut responses);
        }

        let results = json!({ "results": results_vec });

        lines.send(results.to_string()).await?;
    }
    Ok(())
}
