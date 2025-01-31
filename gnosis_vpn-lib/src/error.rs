use std::io;
use std::path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("service not running")]
    ServiceNotRunning,
    #[error("error accessing socket at `{socket_path}`: {error}")]
    SocketPathIO {
        socket_path: path::PathBuf,
        error: io::Error,
    },
    #[error("error connecting socket at `{socket_path:?}`: {error:?}")]
    ConnectSocketIO {
        socket_path: path::PathBuf,
        error: io::Error,
    },
    #[error("failed serializing command: {0}")]
    CommandSerialization(serde_json::Error),
    #[error("error writing to socket at: {0}")]
    WriteSocketIO(io::Error),
    #[error("error reading from socket: {0}")]
    ReadSocketIO(io::Error),
}
