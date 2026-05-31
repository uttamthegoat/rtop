use thiserror::Error;

#[derive(Error, Debug)]
pub enum RtopError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("System error: {0}")]
    System(String),

    #[error("Signal error: {0}")]
    Signal(String),
}
