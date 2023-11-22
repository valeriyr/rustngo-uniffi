#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum SocketError {
    #[error("send error: `{e}`")]
    SendError { e: String },
}
