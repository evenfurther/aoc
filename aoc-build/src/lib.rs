#![deny(clippy::pedantic)]
#![allow(
    clippy::case_sensitive_file_extension_comparisons,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

use regex::Regex;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use syn::parse_quote;

fn output(content: &str) -> eyre::Result<()> {
    fs::write(
        format!("{}/register.rs", std::env::var("OUT_DIR")?),
        content,
    )?;
    Ok(())
}

pub fn build() -> eyre::Result<()> {
    let attr_re = Regex::new(r"#\[aoc\(day(\d+),\s*part(\d+),?(.*)\)\]").unwrap();
    let version_re = Regex::new(r"^\w+$").unwrap();
    let mut refs = Vec::new();
    for file in fs::read_dir("src")? {
        let file = file?;
        if !file
            .file_name()
            .into_string()
            .map(|s| s.ends_with(".rs"))
            .unwrap_or(false)
        {
            continue;
        }
        for l in BufReader::new(File::open(file.path())?).lines() {
            if let Some(m) = attr_re.captures(&l?) {
                let day = m[1].parse::<usize>()?;
                let part = m[2].parse::<usize>()?;
                let version = m[3]
                    .split(',')
                    .map(str::trim)
                    .find(|s| version_re.is_match(s));
                let (version, extension): (syn::Expr, &str) = match version {
                    Some(v) => (parse_quote!(Some(String::from(#v))), v),
                    None => (parse_quote!(None), "none"),
                };
                let mod_name: syn::Ident = syn::parse_str(
                    file.file_name()
                        .into_string()
                        .unwrap()
                        .strip_suffix(".rs")
                        .unwrap(),
                )?;
                let runner_name: syn::Ident =
                    syn::parse_str(&format!("runner_{day}_{part}_{extension}"))?;
                let stmt: syn::Stmt = parse_quote! {
                    ::aoc::register_runner(#day, #part, #version, crate::#mod_name::#runner_name);
                };
                refs.push(stmt);
            }
        }
    }
    let register: syn::File = parse_quote! {
        #[allow(clippy::module_name_repetitions)]
        pub fn register_runners() {
            #(#refs)*
        }
    };
    output(&prettyplease::unparse(&register))?;
    Ok(())
}
