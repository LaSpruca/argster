use std::{path::PathBuf, str::FromStr};

use super::*;

macro_rules! num_from_args {
    ($type:ty, $ref:literal) => {
        impl FromArgsItem for $type {
            const TYPE_NAME: &'static str = stringify!($type);
            const TYPE_DESC: &'static str = $ref;

            fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error> {
                match item {
                    Some(ArgsItem::String(s)) => s.parse().map_err(|ex| Error::InvalidType {
                        arg: "".into(),
                        expected: $ref.into(),
                        found: format!("string: {}", ex),
                    }),
                    Some(ArgsItem::Present) => Err(Error::InvalidType {
                        arg: "".into(),
                        expected: $ref.into(),
                        found: "flag".into(),
                    }),
                    Some(ArgsItem::Many(_)) => Err(Error::InvalidType {
                        arg: "".into(),
                        expected: $ref.into(),
                        found: "list".into(),
                    }),
                    Some(ArgsItem::PresentTimes(n)) => Ok(*n as $type),
                    None => Err(Error::NotFound("".into())),
                }
            }
        }
    };
}

impl FromArgsItem for String {
    const TYPE_DESC: &'static str = "<string>";
    const TYPE_NAME: &'static str = "String";

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

impl FromArgsItem for bool {
    const TYPE_NAME: &'static str = "bool";
    const TYPE_DESC: &'static str = "<true|false>";
    const TYPE_EXTRA: &'static str = "flag";

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error> {
        match item {
            Some(ArgsItem::String(s)) => s.parse().map_err(|ex| Error::InvalidType {
                arg: "string".into(),
                expected: "bool".into(),
                found: format!("string: {}", ex),
            }),
            Some(ArgsItem::Present) => Ok(true),
            Some(ArgsItem::Many(_)) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "true|false".into(),
                found: "list".into(),
            }),
            Some(ArgsItem::PresentTimes(_)) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "true|false".into(),
                found: "flag ".into(),
            }),
            None => Ok(false),
        }
    }
}

impl FromArgsItem for PathBuf {
    const TYPE_DESC: &'static str = "<path>";
    const TYPE_NAME: &'static str = "PathBuf";

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match item {
            Some(ArgsItem::String(s)) => {
                Ok(PathBuf::from_str(s).map_err(|ex| Error::InvalidType {
                    arg: "string".into(),
                    expected: "path".into(),
                    found: format!("string: {}", ex),
                })?)
            }
            Some(ArgsItem::Present) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "path".into(),
                found: "flag".into(),
            }),
            Some(ArgsItem::Many(_)) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "path".into(),
                found: "list".into(),
            }),
            Some(ArgsItem::PresentTimes(_)) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "path".into(),
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
    const TYPE_NAME: &'static str = T::TYPE_NAME;
    const TYPE_DESC: &'static str = T::TYPE_DESC;
    const TYPE_EXTRA: &'static str = "optinal";

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        item.map(|x| T::from_args_item(Some(x))).transpose()
    }
}

impl<T> FromArgsItem for Vec<T>
where
    T: FromArgsItem,
{
    const TYPE_NAME: &'static str = T::TYPE_NAME;
    const TYPE_DESC: &'static str = T::TYPE_DESC;
    const TYPE_EXTRA: &'static str = "list";

    fn from_args_item(item: Option<&ArgsItem>) -> Result<Self, Error> {
        match item {
            Some(ArgsItem::String(s)) => s
                .split(',')
                .map(|x| T::from_args_item(Some(&ArgsItem::String(x.to_string()))))
                .collect::<Result<Vec<_>, _>>(),
            Some(ArgsItem::Present) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "list".into(),
                found: "flag".into(),
            }),
            Some(ArgsItem::Many(list)) => list
                .iter()
                .map(|x| T::from_args_item(Some(&ArgsItem::String(x.to_string()))))
                .collect::<Result<Vec<_>, _>>(),
            Some(ArgsItem::PresentTimes(_)) => Err(Error::InvalidType {
                arg: "string".into(),
                expected: "list".into(),
                found: "flag ".into(),
            }),
            None => Err(Error::NotFound("string".into())),
        }
    }
}

num_from_args!(u8, "<positive number>");
num_from_args!(u16, "<positive number>");
num_from_args!(u32, "<positive number>");
num_from_args!(u64, "<positive number>");
num_from_args!(u128, "<positive number>");
num_from_args!(usize, "<positive number>");

num_from_args!(i8, "<number>");
num_from_args!(i16, "<number>");
num_from_args!(i32, "<number>");
num_from_args!(i64, "<number>");
num_from_args!(i128, "<number>");
num_from_args!(isize, "<number>");

num_from_args!(f64, "<decimal>");
num_from_args!(f32, "<decimal>");
