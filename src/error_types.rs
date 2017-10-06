
use std::error;
use std::fmt;


#[derive(Debug)]
pub struct ErrorType1;
impl error::Error for ErrorType1 {
    fn description(&self) -> &str {
        "Error 1!"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
impl fmt::Display for ErrorType1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", error::Error::description(self))
    }
}

#[derive(Debug)]
pub struct ErrorType2;
impl error::Error for ErrorType2 {
    fn description(&self) -> &str {
        "Error 2!"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
impl fmt::Display for ErrorType2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", error::Error::description(self))
    }
}

