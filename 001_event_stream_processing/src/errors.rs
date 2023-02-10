/*
    This file contains the custom error types for the project

*/

use std::fmt::{Display, Formatter, Debug, Result};

// Custom error types extending the reqwest::Error type
pub enum KeyError {
    InvalidKey,
    MissingKey,
    NotFound,
}

impl Display for KeyError {
   fn fmt(&self, f: &mut Formatter) -> Result {
       write!(f, "{}", self.message())
   }

}

impl Debug for KeyError {
   fn fmt(&self, f: &mut Formatter) -> Result {
       write!(f, "{}", self.message())
   }

}

impl KeyError {
   fn message(&self) -> &str {
       match self {
           Self::InvalidKey => "Invalid Key, please check for typos",
           Self::MissingKey => "Missing Key, got empty string",
           Self::NotFound => "Not Found, please check if entry exists",
       }
   }
}