use std::sync::Arc;

use fake::FakeSocket;
use socket::Socket;

mod error;
mod fake;
mod socket;

uniffi::include_scaffolding!("socket");

#[uniffi::export]
pub fn create_socket(ip: String) -> Arc<dyn Socket> {
    FakeSocket::new(ip)
}
