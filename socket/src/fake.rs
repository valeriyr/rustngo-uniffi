use std::sync::{
    mpsc::{self, Receiver, SyncSender},
    Arc, Mutex,
};

use crate::{error::SocketError, socket::Socket};

macro_rules! rustln {
    () => {
        println!("<rust>")
    };
    ($($arg:tt)*) => {
        print!("<rust> ");
        println!($($arg)*)
    };
}

#[derive(uniffi::Object)]
pub struct FakeSocket {
    ip: String,

    tx: SyncSender<String>,
    rx: Mutex<Receiver<String>>,
}

#[uniffi::export]
impl FakeSocket {
    #[uniffi::constructor]
    pub fn new(ip: String) -> Arc<Self> {
        let (tx, rx) = mpsc::sync_channel(10);
        Arc::new(Self {
            ip: ip.to_owned(),
            tx,
            rx: Mutex::new(rx),
        })
    }
}

#[uniffi::export]
impl Socket for FakeSocket {
    fn write(&self, data: String) -> Result<(), SocketError> {
        match self.tx.send(String::from("Hello from Rust!")) {
            Ok(_) => {
                rustln!("writing: ip = {}, data = {data}", self.ip);
                Ok(())
            }
            Err(e) => {
                rustln!("writing error: {e}");
                Err(SocketError::SendError { e: e.to_string() })
            }
        }
    }

    fn read(&self) -> Option<String> {
        match self.rx.lock() {
            Ok(rx) => match rx.recv() {
                Ok(data) => {
                    rustln!("read: ip = {}, data = {data}", self.ip);
                    Some(data)
                }
                Err(e) => {
                    rustln!("reading error: {e}");
                    None
                }
            },
            Err(e) => {
                rustln!("reading error: {e}");
                None
            }
        }
    }
}
