pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    DatabaseConnectionError(String),
    WebBindError(String),
    WebRuntimeError(String),
    Unknown(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DatabaseConnectionError(addr) => {
                write!(f, "Unable to connect to database: {}", addr)
            }
            Self::WebBindError(addr) => write!(f, "Failed to bind web server to {}", addr),
            Self::WebRuntimeError(err) => write!(f, "Web server runtime error: {}", err),
            Self::Unknown(err) => unimplemented!("Unknown error: {}", err),
        }
    }
}
