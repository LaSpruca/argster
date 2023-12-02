use crate::ArgsItem;
use thiserror::Error;

mod impls;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Required argument {0} not found")]
    NotFound(String),

    #[error("Expected {arg} to be of type {expected}, but found {found}")]
    InvalidType {
        arg: String,
        expected: String,
        found: String,
    },

    #[error("Please enter a commond")]
    NoCommand,
}

impl Error {
    pub fn with_name(self, arg: &str) -> Self {
        match self {
            Error::NotFound(_) => Error::NotFound(arg.into()),
            Error::InvalidType {
                arg: _,
                expected,
                found,
            } => Error::InvalidType {
                arg: arg.into(),
                expected,
                found,
            },
            Error::NoCommand => Error::NoCommand,
        }
    }
}

pub trait FromArgsItem {
    const TYPE_DESC: &'static str;
    const TYPE_EXTRA: &'static str = "required";
    const TYPE_NAME: &'static str;

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error>
    where
        Self: Sized;
}
