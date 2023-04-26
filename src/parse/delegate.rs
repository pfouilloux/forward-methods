use syn::parse::{Parse, ParseStream};
use syn::{Error, Result, Token};

use crate::parse::Method;

pub enum Delegate {
    MethodList(Vec<Method>),
}

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
    use std::fmt::{Debug, Formatter};
    use test_case::test_case;

    use crate::parse::Delegate;
    use crate::parse::Method;

    #[test_case(
        quote!(fn test_a(self), fn test_b(self)),
        Delegate::MethodList(vec![Method::new("test_a", vec!["self"], None), Method::new("test_b", vec!["self"], None)]);
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

    impl PartialEq for Delegate {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Delegate::MethodList(a), Delegate::MethodList(b)) => a == b,
            }
        }
    }

    impl Debug for Delegate {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Delegate::MethodList(methods) => {
                    let meths = methods
                        .iter()
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<String>>()
                        .join(", ");
                    write!(f, "[{}]", meths)
                }
            }
        }
    }
}
