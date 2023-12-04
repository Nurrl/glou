use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(std::io::Error),

    #[error("{0}")]
    Gl(String),

    #[error("{0}")]
    Compile(String),
}
