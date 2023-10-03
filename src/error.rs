pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Response error")]
    Response(reqwest::Error),
    #[error("anyhow")]
    Unknown(anyhow::Error),
}


