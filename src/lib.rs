mod api;
mod character;
mod error;
mod history;
#[cfg(feature="cli")]
mod cli;

pub use api::{ChatApi, ModelApi};
pub use character::Character;
pub use error::{Result, Error};
pub use history::History;

#[cfg(feature="cli")]
pub use cli::Cli;
