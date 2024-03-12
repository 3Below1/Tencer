use tokio::{
    net::{TcpListener, TcpStream},
};
use tokio_tungstenite::{
    tungstenite::protocol::Message,
    WebSocketStream,
};
use futures_util::{
    future,
    stream::TryStreamExt,
    SinkExt, 
    StreamExt
};
use futures::channel::mpsc::{channel, Sender};
use serde::{
    Serialize, 
    Deserialize,
    de,
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    collections::HashMap,
    str::FromStr,
    fmt::Display,
};

pub type PlayerMap = Arc<Mutex<HashMap<i32, PlayerData>>>;

mod api;

pub async fn start(bind_addr: String, player_map: PlayerMap) {
    let listener = TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind websocket address");
    
    loop {
        if let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(start_connection(stream, addr, player_map.clone()));
        } else {
            println!("Failed to accept websocket connection");
        }
    }
}


async fn start_connection(stream: TcpStream, addr: SocketAddr, players: PlayerMap) {
    let mut ws;
    match tokio_tungstenite::accept_async(stream).await {
        Ok(stream) => ws = stream,
        Err(e) => {
            println!("Websocket handshake failed with {}: {:?}", addr, e);
            return;
        }
    }

    let handshake = ws.next().await;

    if handshake.is_none() {
        return;
    }
    let handshake = handshake.unwrap();
    if let Err(e) = handshake {
        println!("Error on initial websocket msg from {}: {:?}", addr, e);
        return;
    }
    let handshake = handshake.unwrap();
    if let Message::Text(handshake) = handshake {
        if let Ok(handshake) = serde_json::from_str::<HandshakeMessage>(&handshake) {
            // do we need a session id when we have the socket?
            match ws.send(Message::Text("{\"SessionId\":567}".to_owned())).await {
                // TODO check auth header (?) for player id
                Ok(_) => return connection(ws, handshake.PlayerId, handshake.AppVersion, players).await,
                Err(_) => return,
            }
        }
        
        println!("Failed to parse ws handshake from {}", addr);
        return;
    } else {
        println!("Received non-text ws handshake from {}", addr);
        return;
    }
}


async fn connection(ws: WebSocketStream<TcpStream>, account_id: i32, version: String, players: PlayerMap) {
    if players.lock().unwrap().contains_key(&account_id) {
        println!("Got ws connection from {}, but they were already in the player map", account_id);
        return;
    }

    let (tx, rx) = channel(64);
    players.lock().unwrap().insert(account_id, PlayerData::new(tx, version));

    let (outgoing, incoming) = ws.split();

    let receive = incoming.try_for_each(|msg| {
        handle_incoming(msg, account_id, &players);
        future::ok(())
    });

    let send = rx.map(Ok).forward(outgoing);

    future::select(receive, send).await;

    // println!("player disconnected: {}", account_id);

    let mut lock = players.lock().unwrap();
    // remove subbed_by for all players
    for p in lock.iter_mut() {
        p.1.subbed_by.retain(|s| *s != account_id);
    }
    lock.remove(&account_id);
}


fn handle_incoming(msg: Message, account_id: i32, players: &PlayerMap) {
    let msg = match msg {
        Message::Text(msg) => msg,
        _ => return,
    };

    if let Ok(api) = serde_json::de::from_str::<ApiRequest>(&msg) {
        if let Err(e) = api::execute(api, account_id, players) {
            println!("Error executing websocket api: {:?} msg: {}", e, msg);
        }
    } else {
        println!("could not parse ws msg from {}: {}", account_id, msg);
    }
}



#[derive(Debug)]
pub struct PlayerData {
    pub ws_tx: Sender<Message>,
    pub subbed_by: Vec<i32>,
    pub game_version: String,
}

impl PlayerData {
    pub fn new(tx: Sender<Message>, game_version: String) -> PlayerData {
        PlayerData {
            ws_tx: tx,
            subbed_by: vec![],
            game_version: game_version,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct HandshakeMessage {
    #[serde(deserialize_with = "from_str")]
    PlayerId: i32,
    AppVersion: String,
    // IpAddress: String,
    // VRDevice: String,
    // IsDevelopment: String,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: de::Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}


#[derive(Deserialize)]
pub struct ApiRequest {
    pub api: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub param: Option<serde_json::Value>,
}

