#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),
    #[error(transparent)]
    Tera(#[from] tera::Error),

    #[error("environemnt variable {0} has error: {1}")]
    Env(String, #[source] std::env::VarError),
}
