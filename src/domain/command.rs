use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum Command {
    Set { key: String, value: Bytes },
    Get { key: String },
    Unknown(String),
}
