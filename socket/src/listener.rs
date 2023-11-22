#[uniffi::export(callback_interface)]
pub trait Listener: Send + Sync {
    fn on_received(&self, data: String);
}
