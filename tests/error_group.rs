
#[macro_use]
extern crate error_util;

use std::fs::File;
use std::io;

use error_util::{ErrorType1, ErrorType2};


impl_error_group! {
    enum LocalError {
        IoError(io::Error),
        FormatError(std::fmt::Error),
        TestError1(ErrorType1, "Test error description"),
        TestError2(ErrorType2, "Another test error"),
    }
}


#[test]
fn error_group() {
    match fail_to_open_file() {
        Err(err) => {
            println!("Display: {}", err);
            println!("Debug: {:?}", err);
            println!("Description: {}", std::error::Error::description(&err));
        },
        Ok(_) => panic!("fail_to_open_file() did not return an error"),
    }

    match throw_test_error() {
        Err(err) => {
            println!("Test error: {}", err);
        },
        Ok(_) => panic!("throw_test_error() did not return an error"),
    }
}

fn fail_to_open_file() -> Result<(), LocalError> {
    let _f = File::open("doesnt_exist.txt")?;
    Ok(())
}

fn throw_test_error() -> Result<(), LocalError> {
    let err = Err(ErrorType1{});
    Ok(err?)
}

