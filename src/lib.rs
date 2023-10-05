mod api;
mod character;
mod error;
mod history;
#[cfg(feature="cli")]
mod cli;

pub use api::ChatApiBuilder;
pub use character::Character;
pub use error::{Result, Error};
pub use history::History;

#[cfg(feature="cli")]
pub use cli::Cli;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
