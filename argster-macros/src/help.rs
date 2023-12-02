use crate::{doc::DocData, function::Command};
use proc_macro::TokenStream;
use quote::quote;
use syn::Type;

pub fn generate_help(commands: &[Command]) -> TokenStream {
    let help_table = commands.iter().map(|Command { name, help, .. }| {
        quote!(Row::new(vec![
            Cell::new(&#name.to_string()).style_spec("bFG"),
            Cell::new(&#help.to_string()),
        ]))
    });

    let commands_help_table = commands.iter().map(|command| {
        let help_table = generate_command_help(&command.doc_data);
        let name = &command.name;
        let help = &command.help;
        quote!(Some(s) if s == #name => {
            _ = stderr.attr(Attr::Bold);
            _ = stderr.fg(color::BRIGHT_GREEN);
            _ = write!(stderr, #name);
            _ = stderr.reset();
            _ = writeln!(stderr, #help);

            table = Table::init(#help_table);
        })
    });

    quote!(
        fn __argster_help(command: Option<String>, error: Option<::argster::from_args::Error>) {
            use ::argster::{
                prettytable::{
                    format::{LinePosition, LineSeparator, TableFormat, consts::FORMAT_CLEAN},
                    Cell, Row, Table,
                },
                term::*,
            };

            let mut stderr = stderr().unwrap();
            _ = stderr.attr(Attr::Bold);
            _ = write!(stderr, "{}", env!("CARGO_PKG_NAME"));
            _ = stderr.reset();
            _ = stderr.fg(color::BRIGHT_BLACK);
            _ = writeln!(stderr, " {}", env!("CARGO_PKG_VERSION"));
            _ = stderr.reset();

            if env!("CARGO_PKG_DESCRIPTION").len() > 0 {
                _ = writeln!(stderr, "{}", env!("CARGO_PKG_DESCRIPTION"));
            }

            if let Some(error) = error {
                _ = stderr.attr(Attr::Bold);
                _ = stderr.fg(color::BRIGHT_RED);
                _ = write!(stderr, "Error");
                _ = stderr.reset();
                _ = writeln!(stderr, ": {}", error);
            }

            _ = writeln!(stderr, "");

            let mut table;
            let content = match command.as_ref() {
                None => {
                    _ = stderr.attr(Attr::Bold);
                    _ = writeln!(stderr, "Commands:");
                    _ = stderr.reset();
                    table = Table::init(vec![#(#help_table),*]);
                },
                #(#commands_help_table),*,
                _ => todo!(),
            };

            table.set_format(*FORMAT_CLEAN);
            _ = table.print_term(stderr.as_mut());
        }
    )
    .into()
}

pub fn generate_command_help(doc_data: &[(DocData, Box<Type>)]) -> proc_macro2::TokenStream {
    let help = doc_data
        .iter()
        .map(|(DocData { docs, short, long }, typ)| {
            let long = if long == "input" {
                "input".to_string()
            } else {
                format!("--{}", long) 
            };
            let short = match short.as_ref() {
                Some(f) => format!("--{f}"),
                None => "".to_string(),
            };
            quote!(Row::new(vec![
                Cell::new(&#long.to_string()).style_spec("bFG"),
                Cell::new(&#short.to_string()).style_spec("bFG"),
                Cell::new(&<#typ as ::argster::from_args::FromArgsItem>::TYPE_NAME.to_string()).style_spec("FD"),
                Cell::new(&<#typ as ::argster::from_args::FromArgsItem>::TYPE_DESC.to_string()).style_spec("FD"),
                Cell::new(&&<#typ as ::argster::from_args::FromArgsItem>::TYPE_EXTRA.to_string().to_string()).style_spec("FD"),
                Cell::new(&#docs.to_string()),
            ]))
        });

    quote!(vec![#(#help),*])
}
