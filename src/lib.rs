
#[macro_use]
mod error_group;

pub mod error_types;
pub use error_types::{ErrorType1, ErrorType2};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
