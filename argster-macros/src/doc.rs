use proc_macro::TokenStream;
use quote::quote;
use std::{collections::HashMap, str::FromStr};
use syn::{Attribute, Expr, ExprLit, Lit};

#[derive(Default, Clone, Debug)]
pub struct DocData {
    pub long: String,
    pub short: Option<String>,
    pub docs: String,
}

impl FromStr for DocData {
    type Err = TokenStream;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = s.trim();
        let rest;
        let long;
        if let Some(input) = source.strip_prefix("input") {
            long = "input".to_string();
            rest = input.trim();
            return Ok(DocData {
                long,
                docs: rest.to_string(),
                ..Default::default()
            });
        }

        if let Some(s) = source.strip_prefix("--") {
            if let Some((parsed_long, parsed_rest)) = s.split_once(" ") {
                long = parsed_long.trim().to_string();
                rest = parsed_rest.trim();
            } else {
                return Ok(DocData {
                    long: s.to_string(),
                    ..Default::default()
                });
            }
        } else {
            let err = format!("'{source}' is an invalid Arg str, please make sure that all arg strings are at the end of the doc comment and start with --<long_name>");
            return Err(quote!(compile_error!(#err);).into());
        }

        if let Some(s) = rest.strip_prefix("-") {
            let s = s.trim();
            if s.starts_with("-") && s.len() > 1 {
                let err = format!("{long} can only have one long name, please make sure the second arg is in the format -<short name>, where short name is only one charcter");
                Err(quote!(compile_error!(#err);).into())
            } else if let Some((short, docs)) = s.split_once(" ") {
                Ok(DocData {
                    long,
                    short: Some(short.to_string()),
                    docs: docs.to_string(),
                })
            } else {
                Ok(DocData {
                    long,
                    short: Some(s.to_string()),
                    ..Default::default()
                })
            }
        } else {
            Ok(DocData {
                long,
                docs: rest.to_string(),
                ..Default::default()
            })
        }
    }
}

pub fn parse_docs(attrs: &Vec<Attribute>) -> Result<HashMap<String, DocData>, TokenStream> {
    attrs
        .into_iter()
        .filter_map(|item| {
            item.meta
                .require_name_value()
                .ok()
                .and_then(|x: &syn::MetaNameValue| {
                    Some((
                        x.path.require_ident().ok()?.to_string(),
                        match x.value {
                            Expr::Lit(ExprLit {
                                lit: Lit::Str(ref s),
                                ..
                            }) => s.value().trim().to_string(),
                            _ => return None,
                        },
                    ))
                })
        })
        .skip_while(|(name, value)| !(name == "doc" && value == "# Args"))
        .skip(1)
        .take_while(|(name, _)| name == "doc")
        .map(|(_, value)| value.parse::<DocData>().map(|m| (m.long.clone(), m)))
        .collect()
}
