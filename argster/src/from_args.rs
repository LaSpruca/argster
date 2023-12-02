use thiserror::Error;

use crate::ArgsItem;

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
    const TYPE_REF: &'static str;
    const TYPE_EXTRA: &'static str;

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error>
    where
        Self: Sized;
}

impl FromArgsItem for String {
    const TYPE_REF: &'static str = "<string>";
    const TYPE_EXTRA: &'static str = "";

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error> {
        match item {
            Some(ArgsItem::String(s)) => Ok(s.to_string()),
            Some(ArgsItem::Present) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "string".into(),
                found: "flag".into(),
            }),
            Some(ArgsItem::Many(_)) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "string".into(),
                found: "list".into(),
            }),
            Some(ArgsItem::PresentTimes(_)) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "string".into(),
                found: "flag ".into(),
            }),
            None => Err(Error::NotFound("string".into())),
        }
    }
}

impl FromArgsItem for usize {
    const TYPE_REF: &'static str = "<number>";
    const TYPE_EXTRA: &'static str = "";

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error> {
        match item {
            Some(ArgsItem::String(s)) => s.parse().map_err(|ex| Error::InvalidType {
                arg: "".into(),
                expected: "usize".into(),
                found: format!("string: {}", ex),
            }),
            Some(ArgsItem::Present) => Err(Error::InvalidType {
                arg: "".into(),
                expected: "usize".into(),
                found: "flag".into(),
            }),
            Some(ArgsItem::Many(_)) => Err(Error::InvalidType {
                arg: "".into(),
                expected: "usize".into(),
                found: "list".into(),
            }),
            Some(ArgsItem::PresentTimes(_)) => Err(Error::InvalidType {
                arg: "".into(),
                expected: "string".into(),
                found: "flag ".into(),
            }),
            None => Err(Error::NotFound("string".into())),
        }
    }
}

impl<T> FromArgsItem for Option<T>
where
    T: FromArgsItem,
{
    const TYPE_REF: &'static str = T::TYPE_REF;
    const TYPE_EXTRA: &'static str = "optinal";

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        item.map(|x| T::from_args_item(Some(x))).transpose()
    }
}
