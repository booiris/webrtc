use std::collections::HashMap;
use std::sync::Arc;

use crate::consts::*;
use crate::model::{ClientReq, ClientResp, DbData};
use log::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

type Db = Arc<Mutex<HashMap<i64, DbData>>>;

pub struct Server {
    db: Db,
}

impl Server {
    pub fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Server {
    pub async fn run(&self) {
        let listener = TcpListener::bind(format!("{}:{}", IP, SERVER_PORT))
            .await
            .unwrap();
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("new client: {:?}", addr);
                    let db = self.db.clone();
                    tokio::spawn(async move {
                        process(socket, db).await;
                    });
                }
                Err(e) => error!("couldn't get client: {:?}", e),
            }
        }
    }
}

async fn process(socket: TcpStream, db: Db) {
    let (reader, mut writer) = socket.into_split();
    let data = read_from_client(reader).await;
    if let Some(client_msg) = serde_json::from_slice::<ClientReq>(&data).ok() {
        let mut db = db.lock().await;
        let from_id = client_msg.client.id;
        let db_data = DbData {
            id_data: client_msg.client,
        };
        (*db).insert(from_id, db_data);
        let data = match (*db).get(&client_msg.aim_user) {
            Some(data) => Some(data),
            None => None,
        };
        let data = match data {
            Some(data) => ClientResp {
                aim_user: Some(data.id_data.clone()),
            },
            None => ClientResp { aim_user: None },
        };
        drop(db);
        debug!("{:?}", data);
        let data = serde_json::to_vec(&data).unwrap();
        send_to_client(&mut writer, &data)
            .await
            .expect("msg send error");
    }
}

async fn read_from_client(reader: OwnedReadHalf) -> Vec<u8> {
    let mut buf_reader = tokio::io::BufReader::new(reader);
    let mut buffer = Vec::new();
    let mut data = Vec::new();
    loop {
        match buf_reader.read_buf(&mut buffer).await {
            Err(err) => {
                error!("read from client error, err: {}", err);
                break;
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                data.append(&mut buffer);
            }
        }
    }
    data
}

async fn send_to_client(writer: &mut OwnedWriteHalf, data: &[u8]) -> Result<(), std::io::Error> {
    writer.write_all(data).await
}
