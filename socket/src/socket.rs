use crate::error::SocketError;

#[uniffi::export]
pub trait Socket: Sync + Send {
    fn write(&self, data: String) -> Result<(), SocketError>;
    fn read(&self) -> Option<String>;
}
