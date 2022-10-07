pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    DatabaseConnectionError(String),
    DatabaseExecutionError(String),
    DatabasePoolError(String),
    IODirectoryError(String),
    IOPathError(String),
    IOReadError(String),
    LoggerError(String),
    TemplateActionExecError(String, String),
    TemplateActionNoContextError(String),
    TemplateActionNoOptionError(String, String),
    TemplateActionNoOptionsError(String),
    TemplateDuplicatedError(String),
    TemplateParseError(String, String),
    WebBindError(String),
    WebRuntimeError(String),
    GenericError(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DatabaseConnectionError(addr) => {
                write!(f, "unable to connect to database at {}", addr)
            }
            Self::DatabaseExecutionError(err) => {
                write!(f, "unable to execute the database query: {}", err)
            }
            Self::DatabasePoolError(err) => write!(
                f,
                "unable to retrieve connection from the database pool: {}",
                err
            ),
            Self::IODirectoryError(dir) => write!(f, "unable to read directory {}", dir),
            Self::IOReadError(file) => write!(f, "unable to read file {}", file),
            Self::IOPathError(path) => write!(f, "unable to access {}", path),
            Self::LoggerError(err) => write!(f, "unable to initialize logger: {}", err),
            Self::TemplateActionExecError(action, err) => {
                write!(f, "error running {}: {}", action, err)
            }
            Self::TemplateActionNoContextError(action) => {
                write!(f, "no context provided while {}", action)
            }
            Self::TemplateActionNoOptionError(action, option) => {
                write!(f, "no '{}' provided while {}", option, action)
            }
            Self::TemplateActionNoOptionsError(action) => {
                write!(f, "no options provided while {}", action)
            }
            Self::TemplateDuplicatedError(name) => write!(f, "template {} already exists", name),
            Self::TemplateParseError(file, err) => {
                write!(f, "unable to parse template {}: {}", file, err)
            }
            Self::WebBindError(addr) => write!(f, "failed to bind web server to {}", addr),
            Self::WebRuntimeError(err) => write!(f, "web server runtime error: {}", err),

            Self::GenericError(err) => write!(f, "{}", err),
        }
    }
}
