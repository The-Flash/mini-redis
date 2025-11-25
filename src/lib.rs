pub mod server;

pub const DEFAULT_PORT: u16 = 6380;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
