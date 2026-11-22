#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use std::fs;
use std::path::Path;
use syn::{
    Attribute, Ident, Item, Meta, Token, parse_quote, punctuated::Punctuated, spanned::Spanned,
};

/// A runner registration found in the sources.
struct Entry {
    day: usize,
    part: usize,
    version: Option<String>,
    module: Vec<Ident>,
}

fn output(content: &str) -> eyre::Result<()> {
    fs::write(
        format!("{}/register.rs", std::env::var("OUT_DIR")?),
        content,
    )?;
    Ok(())
}

/// Parse `dayN`/`partN` style identifiers.
fn numbered(meta: Option<&Meta>, prefix: &str) -> syn::Result<usize> {
    let path = match meta {
        Some(Meta::Path(path)) => path,
        Some(m) => return Err(syn::Error::new(m.span(), format!("expected `{prefix}N`"))),
        None => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("missing `{prefix}N`"),
            ));
        }
    };
    path.get_ident()
        .and_then(|i| i.to_string().strip_prefix(prefix)?.parse().ok())
        .ok_or_else(|| syn::Error::new(path.span(), format!("expected `{prefix}N`")))
}

/// Extract day, part and version from an `#[aoc(…)]` attribute, following
/// the same rules as the `aoc` procedural macro: the first two arguments are
/// the day and the part, `name = value` arguments are options, and a bare
/// identifier names a variant.
fn parse_attr(attr: &Attribute) -> syn::Result<Option<(usize, usize, Option<String>)>> {
    if !attr.path().is_ident("aoc") {
        return Ok(None);
    }
    let args = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
    let mut args = args.iter();
    let day = numbered(args.next(), "day")?;
    let part = numbered(args.next(), "part")?;
    let version = args
        .filter_map(|m| match m {
            Meta::Path(p) => p.get_ident().map(ToString::to_string),
            _ => None,
        })
        .next_back();
    Ok(Some((day, part, version)))
}

fn is_cfg_test(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|a| a.path().is_ident("cfg") && a.parse_args::<Ident>().is_ok_and(|i| i == "test"))
}

fn collect_items(items: &[Item], module: &[Ident], entries: &mut Vec<Entry>) -> syn::Result<()> {
    for item in items {
        match item {
            Item::Fn(f) => {
                for attr in &f.attrs {
                    if let Some((day, part, version)) = parse_attr(attr)? {
                        entries.push(Entry {
                            day,
                            part,
                            version,
                            module: module.to_vec(),
                        });
                    }
                }
            }
            Item::Mod(m) if !is_cfg_test(&m.attrs) => {
                if let Some((_, items)) = &m.content {
                    let mut module = module.to_vec();
                    module.push(m.ident.clone());
                    collect_items(items, &module, entries)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Compute the module path of a source file relative to `src`:
/// `lib.rs` is the crate root, `foo.rs` and `foo/mod.rs` are `foo`,
/// `foo/bar.rs` is `foo::bar`. `main.rs` and `bin/` belong to other crates.
fn module_path(relative: &Path) -> eyre::Result<Option<Vec<Ident>>> {
    let mut components = relative
        .with_extension("")
        .iter()
        .map(|c| c.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    match components.as_slice() {
        [single] if single == "main" => return Ok(None),
        [first, ..] if first == "bin" => return Ok(None),
        [single] if single == "lib" => components.clear(),
        [.., last] if last == "mod" => {
            components.pop();
        }
        _ => {}
    }
    Ok(Some(
        components
            .iter()
            .map(|c| syn::parse_str::<Ident>(c))
            .collect::<Result<_, _>>()?,
    ))
}

fn scan_dir(root: &Path, dir: &Path, entries: &mut Vec<Entry>) -> eyre::Result<()> {
    let mut paths = fs::read_dir(dir)?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    for path in paths {
        if path.is_dir() {
            scan_dir(root, &path, entries)?;
            continue;
        }
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let Some(module) = module_path(path.strip_prefix(root)?)? else {
            continue;
        };
        let content = fs::read_to_string(&path)?;
        // Leave syntax errors to the compiler, which reports them much better.
        let Ok(file) = syn::parse_file(&content) else {
            println!("cargo::warning=cannot parse {}, skipping", path.display());
            continue;
        };
        collect_items(&file.items, &module, entries)
            .map_err(|e| eyre::eyre!("{}: {e}", path.display()))?;
    }
    Ok(())
}

pub fn build() -> eyre::Result<()> {
    let src = Path::new("src");
    println!("cargo::rerun-if-changed=src");
    let mut entries = Vec::new();
    scan_dir(src, src, &mut entries)?;
    let refs = entries.iter().map(|entry| -> syn::Stmt {
        let Entry {
            day,
            part,
            version,
            module,
        } = entry;
        let (version, extension): (syn::Expr, &str) = match version {
            Some(v) => (parse_quote!(Some(String::from(#v))), v),
            None => (parse_quote!(None), "none"),
        };
        let runner_name = Ident::new(
            &format!("runner_{day}_{part}_{extension}"),
            proc_macro2::Span::call_site(),
        );
        parse_quote! {
            ::aoc::register_runner(#day, #part, #version, crate #(::#module)* ::#runner_name);
        }
    });
    let register: syn::File = parse_quote! {
        #[allow(clippy::module_name_repetitions)]
        pub fn register_runners() {
            #(#refs)*
        }
    };
    output(&prettyplease::unparse(&register))?;
    Ok(())
}
