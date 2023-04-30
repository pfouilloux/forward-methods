use proc_macro2::{Ident, Span};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Error, FnArg, PatType, Receiver, Result, Token,
};

use crate::model::Method;

impl Parse for Method {
    fn parse(input: ParseStream) -> Result<Self> {
        _ = input.parse::<Token![fn]>()?;

        let ident: Ident = input.parse()?;
        let (rcv, args) = parse_fn_args(input)?;
        let ret = input.parse()?;

        Ok(Method {
            ident,
            rcv,
            args,
            ret,
        })
    }
}

fn parse_fn_args(input: ParseStream) -> Result<(Receiver, Vec<PatType>)> {
    let args_buf;
    _ = parenthesized!(args_buf in input);

    let punct_args = args_buf.parse_terminated(FnArg::parse, Token![, ])?;
    let mut args = punct_args.iter();

    Ok((
        get_receiver(args.next())?,
        args.flat_map(select_pat_type).collect(),
    ))
}

fn get_receiver(arg: Option<&FnArg>) -> Result<Receiver> {
    if let Some(FnArg::Receiver(rcv)) = arg {
        Ok(rcv.clone())
    } else {
        Err(Error::new(
            Span::call_site(),
            "method must have a receiver to be forwarded",
        ))
    }
}

fn select_pat_type(arg: &FnArg) -> Option<PatType> {
    if let FnArg::Typed(pt) = arg.clone() {
        Some(pt)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use proc_macro2::TokenStream;
    use quote::quote;
    use test_case::test_case;

    use crate::model::{Method, MethodBuilder};

    #[test_case(
        quote!(fn test(self)),
        MethodBuilder::default().ident("test").rcv();
        "should parse method with move receiver"
    )]
    #[test_case(
        quote!(fn test(mut self)),
        MethodBuilder::default().ident("test").mut_rcv();
        "should parse method with mut receiver"
    )]
    #[test_case(
        quote!(fn test(&self)),
        MethodBuilder::default().ident("test").ref_rcv();
        "should parse method with ref receiver"
    )]
    #[test_case(
        quote!(fn test(&mut self)),
        MethodBuilder::default().ident("test").ref_mut_rcv();
        "should parse method with mut ref receiver"
    )]
    #[test_case(
        quote!(fn test(self, val: String)),
        MethodBuilder::default().ident("test").rcv().with_arg("val: String");
        "should parse method with multiple params"
    )]
    #[test_case(
        quote!(fn test(self) -> Result<String, Error>),
        MethodBuilder::default().ident("test").rcv().ret("-> Result<String, Error>");
        "should parse method with return value"
    )]
    #[test_case(
        quote!(fn test(self) -> (String, uint)),
        MethodBuilder::default().ident("test").rcv().ret("-> (String, uint)");
        "should parse method with tuple return value"
    )]
    fn should_parse_method(input: TokenStream, want: &mut MethodBuilder) {
        let meth = syn::parse2::<Method>(input).unwrap();

        assert_eq!(meth, want.build().unwrap())
    }

    #[test_case(quote!(fn test()), "method must have a receiver to be forwarded"; "should require a receiver")]
    fn should_fail_to_parse_method(input: TokenStream, want: &str) {
        let err = syn::parse2::<Method>(input).unwrap_err();

        assert_eq!(err.to_string(), want)
    }
}
