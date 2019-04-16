use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyErr {
    details: String,
}

impl MyErr {
    pub fn new(msg: &str) -> MyErr {
        MyErr {
            details: String::from(msg),
        }
    }
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyErr {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&Error> {
        None //Generic error, underlying cause isn't tracked
    }
}
