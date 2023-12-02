use crate::doc::{parse_docs, DocData};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, ImplItemFn, Lit, Type};

pub struct Command {
    pub name: String,
    pub help: String,
    pub doc_data: Vec<(DocData, Box<Type>)>,
    pub tokens: proc_macro2::TokenStream,
}

pub fn generate_command(func: &ImplItemFn) -> Result<Command, TokenStream> {
    let doc_data = parse_docs(&func.attrs)?;
    let name = &func.sig.ident;
    let name_string = name.to_string();

    let (args, doc_data): (Vec<TokenStream>, Vec<(DocData, Box<Type>)>) = func
        .sig
        .inputs
        .iter()
        .filter_map(|item| match item {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(arg) => match arg.pat.as_ref() {
                syn::Pat::Ident(ident) => Some((ident, arg.ty.to_owned())),
                _ => None,
            },
        })
        .map(|(item, ty)| {
            let item_name = item.ident.to_string();
            let doc_data = doc_data.get(item_name.as_str()).cloned().unwrap_or_else(|| DocData { long: item_name.to_string(), short: None, docs: "".into() });
            let short = doc_data.short.as_ref();

            if item_name == "input" {
                (quote!(::argster::from_args::FromArgsItem::from_args_item(args.get("")).map_err(|x| (Some(#name_string), x.with_name("input")))?), (doc_data, ty))
            } else if let Some(short) = short {
                (quote!(::argster::from_args::FromArgsItem::from_args_item(args.get(#item_name).or_else(|| args.get(#short))).map_err(|x| (Some(#name_string), x.with_name(#item_name)))?), (doc_data, ty))
            } else {
                (quote!(::argster::from_args::FromArgsItem::from_args_item(args.get(#item_name)).map_err(|x| (Some(#name_string), x.with_name(#item_name)))?), (doc_data, ty))
            }
        }).unzip();

    let help = func
        .attrs
        .iter()
        .filter_map(|item| {
            let item = item.meta.require_name_value().ok()?;
            if !item.path.is_ident("doc") {
                return None;
            }

            match item.value {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(ref s),
                    ..
                }) => Some(s.value()),
                _ => None,
            }
        })
        .take_while(|item| item != " # Args")
        .collect::<Vec<_>>()
        .join("\n");

    Ok(Command {
        name: name_string,
        help,
        doc_data,
        tokens: quote!(Self::#name(#(#args),*)),
    })
}
