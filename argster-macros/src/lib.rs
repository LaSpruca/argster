use help::generate_help;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ImplItem, ImplItemFn, ItemImpl};

use crate::function::{generate_command, Command};

mod doc;
mod function;
mod help;

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut tree = parse_macro_input!(item as ItemImpl);
    let commands = tree
        .items
        .iter()
        .filter_map(|ex| match ex {
            ImplItem::Fn(func) => Some(func),
            _ => None,
        })
        .map(|function| generate_command(function))
        .collect::<Result<Vec<Command>, proc_macro2::TokenStream>>();

    if let Err(err) = commands {
        return err.into();
    }

    let commands = commands.unwrap();

    let main_generator = commands.iter().map(|Command { name, tokens, .. }| {
        quote!(
            #name => {
                #tokens
            }
        )
    });

    let argster_main = quote!(
        fn __argster_main() -> Result<(), (Option<&'static str>, ::argster::from_args::Error)> {
            let mut iter = ::std::env::args().skip(1);
            let command = iter.next().ok_or_else(|| (None, ::argster::from_args::Error::NoCommand))?;
            let args = ::argster::parse_args(iter);

            if (args.get("--help").is_some() || args.get("-h").is_some()) {
                Self::__argster_help(Some(command), None);
                return Ok(());
            }

            match command.as_str() {
                #(#main_generator),*,
                "help" | "--help" | "-h" => Self::__argster_help(::argster::from_args::FromArgsItem::from_args_item(args.get("")).map_err(|x| (Some("help"), x.with_name("input")))?, None),
                _ => panic!("Unknown command {}", command),
            };
            Ok(())
        }
    )
    .into();

    let main = quote!(
        fn main() {
            match Self::__argster_main() {
                Ok(_) => (),
                Err((name, ex)) => Self::__argster_help(name.map(|f| f.to_string()), Some(ex)),
            }
        }
    )
    .into();

    let help = generate_help(&commands);

    // println!("{}", help);
    tree.items
        .push(ImplItem::Fn(parse_macro_input!(argster_main as ImplItemFn)));

    tree.items
        .push(ImplItem::Fn(parse_macro_input!(main as ImplItemFn)));

    tree.items
        .push(ImplItem::Fn(parse_macro_input!(help as ImplItemFn)));

    tree.to_token_stream().into()
}
