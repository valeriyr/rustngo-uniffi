use std::sync::Arc;

use fake::FakeSocket;
use listener::Listener;
use socket::Socket;

mod error;
mod fake;
mod listener;
mod socket;

uniffi::include_scaffolding!("socket");

#[uniffi::export]
pub fn create_socket(ip: String, listener: Box<dyn Listener>) -> Arc<dyn Socket> {
    FakeSocket::new(ip, listener)
}
