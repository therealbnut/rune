prelude!();

use std::sync::Arc;

use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use macros::quote;
use parse::Parser;

#[test]
fn test_parse_in_macro() -> Result<()> {
    let mut m = Module::default();

    let string = "1 + 2 + 13 * 3";

    m.macro_(["string_as_code"], move |ctx, _| {
        let id = ctx.insert_source("string_as_code", string);
        let expr = ctx.parse_source::<ast::Expr>(id)?;

        Ok(quote!(#expr).into_token_stream(ctx))
    })?;

    m.macro_(["string_as_code_from_arg"], |ctx, stream| {
        let mut p = Parser::from_token_stream(stream, ctx.input_span());
        let s = p.parse_all::<ast::LitStr>()?;
        let s = ctx.resolve(s)?.into_owned();
        let id = ctx.insert_source("string_as_code_from_arg", &s);
        let expr = ctx.parse_source::<ast::Expr>(id)?;

        Ok(quote!(#expr).into_token_stream(ctx))
    })?;

    let mut context = Context::with_default_modules()?;
    context.install(m)?;

    let mut sources = sources! {
        entry => {
            pub fn main() {
                let a = string_as_code!();
                let b = string_as_code_from_arg!("1 + 2 + 13 * 3");
                (a, b)
            }
        }
    };

    let unit = prepare(&mut sources).with_context(&context).build()?;

    let mut vm = Vm::new(Arc::new(context.runtime()), Arc::new(unit));
    let output = vm.call(["main"], ())?;
    let output: (u32, u32) = from_value(output)?;

    assert_eq!(output, (42, 42));
    Ok(())
}

#[test]
fn conflicting_attribute_function() -> Result<()> {
    let mut m = Module::default();

    m.macro_(["conflicting"], move |ctx, _| {
        Ok(quote!(21).into_token_stream(ctx))
    })?;

    m.attribute_macro(["conflicting"], |ctx, _, _| {
        Ok(quote!(
            fn hello() {
                21
            }
        )
        .into_token_stream(ctx))
    })?;

    let mut context = Context::with_default_modules()?;
    context.install(m)?;

    let mut sources = sources! {
        entry => {
            pub fn main() {
                hello() + conflicting!()
            }

            #[conflicting]
            fn hi() {}
        }
    };

    let unit = prepare(&mut sources).with_context(&context).build()?;

    let mut vm = Vm::new(Arc::new(context.runtime()), Arc::new(unit));
    let output = vm.call(["main"], ())?;
    let output: u32 = from_value(output)?;

    assert_eq!(output, 42);
    Ok(())
}

#[test]
fn attribute_imports_builtin() -> Result<()> {
    let mut m = Module::with_crate("abc");

    m.attribute_macro(["before_use"], |ctx, _, _| {
        Ok(quote!(
            fn before() {
                21
            }
        )
        .into_token_stream(ctx))
    })?;

    m.attribute_macro(["after_use"], |ctx, _, _| {
        Ok(quote!(
            fn after() {
                21
            }
        )
        .into_token_stream(ctx))
    })?;

    let mut context = Context::with_default_modules()?;
    context.install(m)?;

    let mut sources = sources! {
        entry => {
            #[doc = "Doc comment"]
            #[test]
            pub fn main() {
                before() + after()
            }

            #[before_use]
            fn hi() {}

            use ::abc::{ before_use, after_use };

            #[after_use]
            fn ho() {}

        }
    };

    let diagnostics = &mut Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(diagnostics)
        .build();

    if !diagnostics.is_empty() {
        diagnostics.emit(&mut StandardStream::stdout(ColorChoice::Auto), &sources)?;
    }

    let unit = result?;

    let mut vm = Vm::new(Arc::new(context.runtime()), Arc::new(unit));
    let output = vm.call(["main"], ())?;
    let output: u32 = from_value(output)?;

    assert_eq!(output, 42);
    Ok(())
}
