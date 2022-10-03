pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    DatabaseConnectionError(String),
    IODirectoryError(String),
    IOPathError(String),
    IOReadError(String),
    LoggerError(String),
    TemplateActionExecError(String,String),
    TemplateActionNoContextError(String),
    TemplateActionNoOptionError(String,String),
    TemplateActionNoOptionsError(String),
    TemplateDuplicatedError(String),
    TemplateParseError(String,String),
    WebBindError(String),
    WebRuntimeError(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DatabaseConnectionError(addr) => {
                write!(f, "Unable to connect to database: {}", addr)
            }
            Self::IODirectoryError(dir) => write!(f, "Unable to read directory {}", dir),
            Self::IOReadError(file) => write!(f, "Unable to read file {}", file),
            Self::IOPathError(path) => write!(f, "Unable to access {}", path),
            Self::LoggerError(err) => write!(f, "Unable to initialize logger: {}", err),
            Self::TemplateActionExecError(action, err) => write!(f, "Error running {}: {}", action, err),
            Self::TemplateActionNoContextError(action) => write!(f, "No context provided while {}", action),
            Self::TemplateActionNoOptionError(action,option) => write!(f, "No '{}' provided while {}", option, action),
            Self::TemplateActionNoOptionsError(action) => write!(f, "No options provided while {}", action),
            Self::TemplateDuplicatedError(name) => write!(f, "Template {} already exists", name),
            Self::TemplateParseError(file, err) => write!(f, "Unable to parse template {}: {}", file, err),
            Self::WebBindError(addr) => write!(f, "Failed to bind web server to {}", addr),
            Self::WebRuntimeError(err) => write!(f, "Web server runtime error: {}", err),
        }
    }
}
