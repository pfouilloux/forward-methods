use syn::parse::{Parse, ParseStream};
use syn::{Error, Result, Token};

use crate::model::{Delegate, Method};

impl Parse for Delegate {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![fn]) {
            let meth: Method = input.parse()?;
            let mut methods = vec![meth];

            while input.peek(Token![,]) && input.peek2(Token![fn]) {
                _ = input.parse::<Token![,]>();

                let meth: Method = input.parse()?;
                methods.push(meth)
            }

            Ok(Delegate::MethodList(methods))
        } else {
            Err(Error::new(
                input.span(),
                "delegates must be declared as a list of methods in the form 'fn ident(arg1, arg2, ...) -> Return'"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;
    use test_case::test_case;

    use crate::model::{Delegate, MethodBuilder};

    #[test_case(
        quote!(fn test_a(self), fn test_b(self)),
        Delegate::MethodList(vec![
            MethodBuilder::default().ident("test_a").rcv().build().unwrap(),
            MethodBuilder::default().ident("test_b").rcv().build().unwrap()
        ]);
        "should parse delegate method list"
    )]
    fn should_parse_delegate(input: TokenStream, want: Delegate) {
        let del = syn::parse2::<Delegate>(input).unwrap();

        assert_eq!(del, want)
    }

    #[test_case(
        quote!(invalid),
        "delegates must be declared as a list of methods in the form 'fn ident(arg1, arg2, ...) -> Return'";
        "should require method list format"
    )]
    fn should_fail_to_parse_delegate(input: TokenStream, want: &str) {
        let err = syn::parse2::<Delegate>(input).unwrap_err();

        assert_eq!(err.to_string(), want)
    }
}
