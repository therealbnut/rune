//! <img alt="rune logo" src="https://raw.githubusercontent.com/rune-rs/rune/main/assets/icon.png" />
//! <br>
//! <a href="https://github.com/rune-rs/rune"><img alt="github" src="https://img.shields.io/badge/github-rune--rs/rune-8da0cb?style=for-the-badge&logo=github" height="20"></a>
//! <a href="https://crates.io/crates/rune-macros"><img alt="crates.io" src="https://img.shields.io/crates/v/rune-macros.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20"></a>
//! <a href="https://docs.rs/rune-macros"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rune--macros-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20"></a>
//! <a href="https://discord.gg/v5AeNkT"><img alt="chat on discord" src="https://img.shields.io/discord/558644981137670144.svg?logo=discord&style=flat-square" height="20"></a>
//! <br>
//! Minimum support: Rust <b>1.65+</b>.
//! <br>
//! <br>
//! <a href="https://rune-rs.github.io"><b>Visit the site 🌐</b></a>
//! &mdash;
//! <a href="https://rune-rs.github.io/book/"><b>Read the book 📖</b></a>
//! <br>
//! <br>
//!
//! Macros for the Rune Language, an embeddable dynamic programming language for Rust.
//!
//! <br>
//!
//! ## Usage
//!
//! This is part of the [Rune Language](https://rune-rs.github.io).

use ::quote::format_ident;
use syn::{Generics, Path};

extern crate proc_macro;

mod any;
mod context;
mod from_value;
mod function;
mod inst_display;
mod instrument;
mod internals;
mod macro_;
mod opaque;
mod option_spanned;
mod parse;
mod quote;
mod spanned;
mod to_tokens;
mod to_value;

#[proc_macro]
pub fn quote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let parser = crate::quote::Quote::new();

    let output = match parser.parse(input) {
        Ok(output) => output,
        Err(e) => return proc_macro::TokenStream::from(e.to_compile_error()),
    };

    output.into()
}

#[proc_macro_attribute]
pub fn function(
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs = syn::parse_macro_input!(attrs with crate::function::FunctionAttrs::parse);
    let function = syn::parse_macro_input!(item with crate::function::Function::parse);

    let output = match function.expand(attrs) {
        Ok(output) => output,
        Err(e) => return proc_macro::TokenStream::from(e.to_compile_error()),
    };

    output.into()
}

#[proc_macro_attribute]
pub fn macro_(
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs = syn::parse_macro_input!(attrs with crate::macro_::Config::parse);
    let macro_ = syn::parse_macro_input!(item with crate::macro_::Macro::parse);

    let output = match macro_.expand(attrs, format_ident!("function")) {
        Ok(output) => output,
        Err(e) => return proc_macro::TokenStream::from(e.to_compile_error()),
    };

    output.into()
}

#[proc_macro_attribute]
pub fn attribute_macro(
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs = syn::parse_macro_input!(attrs with crate::macro_::Config::parse);
    let macro_ = syn::parse_macro_input!(item with crate::macro_::Macro::parse);

    let output = match macro_.expand(attrs, format_ident!("attribute")) {
        Ok(output) => output,
        Err(e) => return proc_macro::TokenStream::from(e.to_compile_error()),
    };

    output.into()
}

#[proc_macro_derive(ToTokens, attributes(rune))]
#[doc(hidden)]
pub fn to_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as to_tokens::Derive);
    derive.expand().unwrap_or_else(to_compile_errors).into()
}

#[proc_macro_derive(Parse, attributes(rune))]
#[doc(hidden)]
pub fn parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as parse::Derive);
    derive.expand().unwrap_or_else(to_compile_errors).into()
}

#[proc_macro_derive(Spanned, attributes(rune))]
#[doc(hidden)]
pub fn spanned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as spanned::Derive);
    derive.expand().unwrap_or_else(to_compile_errors).into()
}

#[proc_macro_derive(OptionSpanned, attributes(rune))]
#[doc(hidden)]
pub fn option_spanned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as option_spanned::Derive);
    derive.expand().unwrap_or_else(to_compile_errors).into()
}

#[proc_macro_derive(Opaque, attributes(rune))]
#[doc(hidden)]
pub fn opaque(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as opaque::Derive);
    derive.expand().unwrap_or_else(to_compile_errors).into()
}

#[proc_macro_derive(FromValue, attributes(rune))]
pub fn from_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    from_value::expand(&input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

#[proc_macro_derive(ToValue, attributes(rune))]
pub fn to_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    to_value::expand(&input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

#[proc_macro_derive(Any, attributes(rune))]
pub fn any(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as any::Derive);
    derive.expand().unwrap_or_else(to_compile_errors).into()
}

#[proc_macro]
#[doc(hidden)]
pub fn __internal_impl_any(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let internal_call = syn::parse_macro_input!(input as any::InternalCall);
    internal_call
        .expand()
        .unwrap_or_else(to_compile_errors)
        .into()
}

#[proc_macro_attribute]
#[doc(hidden)]
pub fn instrument(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = syn::parse_macro_input!(attr as instrument::Attr);
    let internal_call = syn::parse_macro_input!(item as instrument::Expander);

    internal_call
        .expand(&attr)
        .unwrap_or_else(to_compile_errors)
        .into()
}

#[proc_macro_derive(InstDisplay, attributes(inst_display))]
#[doc(hidden)]
pub fn inst_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input as inst_display::Derive);
    derive.expand().unwrap_or_else(to_compile_errors).into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.into_iter().map(syn::Error::into_compile_error);
    ::quote::quote!(#(#compile_errors)*)
}

/// Adds the `path` as trait bound to each generic
fn add_trait_bounds(generics: &mut Generics, path: &Path) {
    for p in &mut generics.params {
        match p {
            syn::GenericParam::Type(ty) => {
                ty.bounds.push(syn::TypeParamBound::Trait(syn::TraitBound {
                    paren_token: None,
                    modifier: syn::TraitBoundModifier::None,
                    lifetimes: None,
                    path: path.clone(),
                }));
            }
            _ => continue,
        }
    }
}
