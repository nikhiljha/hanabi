use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    #[error("unable to parse websocket message")]
    WebsocketParseError,
}
