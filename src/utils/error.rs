#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
}
