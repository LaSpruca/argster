pub use argster_macros::command;
pub use prettytable;
use std::collections::HashMap;
pub mod term {
    pub use term::{color, stderr, Attr};
}

pub mod from_args;

#[derive(Debug, PartialEq)]
pub enum ArgsItem {
    String(String),
    Many(Vec<String>),
    Present,
    PresentTimes(usize),
}

pub fn parse_args(iter: impl Iterator<Item = String>) -> HashMap<String, ArgsItem> {
    let mut iter = iter.peekable();
    let mut collection = HashMap::new();

    while let Some(item) = iter.next() {
        let (key, value) = if let Some(key) = item.strip_prefix("--") {
            (key.to_string(), {
                let temp = iter
                    .peek()
                    .filter(|item| !item.starts_with('-'))
                    .map(ToString::to_string);
                if temp.is_some() {
                    iter.next();
                }
                temp
            })
        } else if let Some(item) = item.strip_prefix('-') {
            if item.is_empty() {
                continue;
            } else if item.len() == 1 {
                (item.to_string(), {
                    let temp = iter
                        .peek()
                        .filter(|item| !item.starts_with('-'))
                        .map(ToString::to_string);
                    if temp.is_some() {
                        iter.next();
                    }
                    temp
                })
            } else {
                (
                    item.get(0..1).unwrap().to_string(),
                    item.get(1..).map(ToString::to_string),
                )
            }
        } else {
            ("".into(), Some(item))
        };

        collection
            .entry(key.to_string())
            .and_modify(|z| match z {
                ArgsItem::Many(vec) => {
                    vec.push(match value.as_ref() {
                        Some(val) => val.into(),
                        None => "".into(),
                    });
                }
                ArgsItem::String(previous_value) => {
                    *z = ArgsItem::Many(vec![
                        previous_value.clone(),
                        match value.as_ref() {
                            Some(val) => val.into(),
                            None => "".into(),
                        },
                    ])
                }
                ArgsItem::Present => match value.as_ref() {
                    Some(value) => *z = ArgsItem::Many(vec!["".into(), value.clone()]),
                    None => *z = ArgsItem::PresentTimes(2),
                },
                ArgsItem::PresentTimes(n) => match value.as_ref() {
                    Some(value) => {
                        *z = ArgsItem::Many({
                            let mut temp = vec!["".to_string(); *n];
                            temp.push(value.clone());
                            temp
                        })
                    }
                    None => *n += 1,
                },
            })
            .or_insert(match value {
                Some(value) => ArgsItem::String(value),
                None => ArgsItem::Present,
            });
    }

    collection
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_parse_args() {
        let args = vec!["--hello".into()];

        let parsed = parse_args(args.into_iter());

        assert_eq!(
            parsed,
            HashMap::from_iter([("hello".into(), ArgsItem::Present)])
        );
    }

    #[test]
    fn test_parse_args_with_value() {
        let args = vec!["--hello".into(), "world".into()];

        let parsed = parse_args(args.into_iter());

        assert_eq!(
            parsed,
            HashMap::from_iter([("hello".into(), ArgsItem::String("world".into()))])
        );
    }

    #[test]
    fn test_parsed_vales() {
        let args = vec!["hello".into(), "world".into()];

        let parsed = parse_args(args.into_iter());

        assert_eq!(
            parsed,
            HashMap::from_iter([(
                "".into(),
                ArgsItem::Many(vec!["hello".into(), "world".into()])
            )])
        );
    }

    #[test]
    fn short_value() {
        let args = vec![
            "-hworld".into(),
            "yes".into(),
            "-v".into(),
            "-p".into(),
            "other".into(),
        ];

        let parsed = parse_args(args.into_iter());

        assert_eq!(
            parsed,
            HashMap::from_iter([
                ("h".into(), ArgsItem::String("world".into())),
                ("p".into(), ArgsItem::String("other".into())),
                ("v".into(), ArgsItem::Present),
                ("".into(), ArgsItem::String("yes".into()))
            ])
        );
    }
}
