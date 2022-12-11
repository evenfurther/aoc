use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, FnArg, Ident, ItemFn, LitChar, LitStr, PatType, ReturnType, Token,
};

#[derive(Default)]
struct AocEntry {
    day: usize,
    part: usize,
    version: Option<String>,
    separator: Option<String>,
}

impl Parse for AocEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let day = syn::Ident::parse(input)?;
        let day = match day.to_string().strip_prefix("day") {
            Some(d) => match d.parse::<usize>() {
                Ok(day @ 1..=25) => day,
                _ => {
                    return Err(syn::Error::new(
                        day.span(),
                        format!("cannot parse day (between 1 and 25) {d}"),
                    ))
                }
            },
            None => {
                return Err(syn::Error::new(
                    day.span(),
                    "day must start with `day` such as `day12`",
                ))
            }
        };
        <Token![,]>::parse(input)?;
        let part = syn::Ident::parse(input)?;
        let part = match part.to_string().strip_prefix("part") {
            Some(d) => match d.parse::<usize>() {
                Ok(part @ (1 | 2)) => part,
                _ => {
                    return Err(syn::Error::new(
                        part.span(),
                        format!("cannot parse part (1 or 2) {d}"),
                    ))
                }
            },
            None => {
                return Err(syn::Error::new(
                    part.span(),
                    "part must start with `part` such as `part12`",
                ))
            }
        };
        let mut entry = AocEntry {
            day,
            part,
            ..Default::default()
        };
        while !input.is_empty() {
            <Token![,]>::parse(input)?;
            match input.parse::<Ident>()?.to_string().as_str() {
                "separator" => {
                    <Token![=]>::parse(input)?;
                    let lookahead = input.lookahead1();
                    if lookahead.peek(LitChar) {
                        entry.separator =
                            Some(input.parse::<LitChar>().unwrap().value().to_string());
                    } else if lookahead.peek(LitStr) {
                        entry.separator = Some(input.parse::<LitStr>().unwrap().value());
                    } else {
                        lookahead.error();
                    }
                }
                i => entry.version = Some(i.to_owned()),
            }
        }
        Ok(entry)
    }
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn aoc(attr: TokenStream, input: TokenStream) -> TokenStream {
    let aoc_entry = parse_macro_input!(attr as AocEntry);
    let day = aoc_entry.day;
    let part = aoc_entry.part;
    let version = aoc_entry.version;
    let func = parse_macro_input!(input as ItemFn);
    let func_name = func.sig.ident.clone();
    let runner_func_name = Ident::new(
        &format!(
            "runner_{}_{}_{}",
            day,
            part,
            version.clone().unwrap_or_else(|| String::from("none"))
        ),
        func.sig.ident.span(),
    );
    let sep = match aoc_entry.separator {
        Some(sep) => quote!(Some(#sep)),
        None => quote!(None),
    };
    let inputs = match func.sig.inputs.first() {
        Some(FnArg::Typed(PatType { ty, .. }))
            if quote!(#ty).to_string().contains("Vec < & [u8] >") =>
        {
            quote!((::aoc::input::parse_input_bytes(&::aoc::input::input_bytes(#day)?, #sep.map(|c: char| c as u8))?))
        }
        Some(FnArg::Typed(PatType { ty, .. }))
            if quote!(#ty).to_string().contains("& [& [u8]]") =>
        {
            quote!((&::aoc::input::parse_input_bytes(&::aoc::input::input_bytes(#day)?, #sep.map(|c: char| c as u8))?))
        }
        Some(FnArg::Typed(PatType { ty, .. }))
            if quote!(#ty).to_string().contains("Vec < & str >") =>
        {
            quote!((::aoc::input::input_string(#day)?.lines().collect()))
        }
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& [& str]") => {
            quote!((&::aoc::input::input_string(#day)?.lines().collect::<Vec<_>>()))
        }
        Some(FnArg::Typed(PatType { ty, .. }))
            if quote!(#ty).to_string().contains("& mut [& str]") =>
        {
            quote!((&mut ::aoc::input::input_string(#day)?.lines().collect::<Vec<_>>()))
        }
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& str") => {
            quote!((&::aoc::input::input_string(#day)?))
        }
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& [u8]") => {
            quote!((&::aoc::input::input_bytes(#day)?))
        }
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("Vec <") => {
            quote!((::aoc::input::parse_input(&::aoc::input::input_string(#day)?, #sep)?))
        }
        Some(FnArg::Typed(PatType { ty, .. })) if quote!(#ty).to_string().contains("& mut [") => {
            quote!((&mut ::aoc::input::parse_input(&::aoc::input::input_string(#day)?, #sep)?))
        }
        Some(_) => {
            quote!((&::aoc::input::parse_input(&::aoc::input::input_string(#day)?, #sep)?))
        }
        None => quote!(()),
    };
    let (call, ty) = match func.sig.output {
        ReturnType::Type(_, ref t) if t.to_token_stream().to_string().contains("Result < ") => {
            (quote!(#func_name #inputs), quote!(#t))
        }
        ReturnType::Type(_, ref t) if t.to_token_stream().to_string().contains("Option <") => {
            let no_option = t
                .to_token_stream()
                .into_iter()
                .skip(1)
                .collect::<proc_macro2::TokenStream>();
            let version = version.map_or(quote!(None), |v| quote!(Some(#v)));
            (
                quote!(Ok(#func_name #inputs .ok_or(::aoc::error::Error::NoOutput(#day, #part, #version))?)),
                quote!(::eyre::Result #no_option),
            )
        }
        ReturnType::Type(_, ref t) => (quote!(Ok(#func_name #inputs)), quote!(::eyre::Result<#t>)),
        _ => abort!(func.sig, "AOC part cannot return ()"),
    };
    quote! {
        #func

        pub fn #runner_func_name() -> #ty {
            #call
        }
    }
    .into()
}
