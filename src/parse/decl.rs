use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Member, Result, Token};

use crate::parse::Delegate;

pub struct FwdDecl {
    delegate: Delegate,
    target: Member,
}

impl Parse for FwdDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let delegate: Delegate = input.parse()?;

        let ident: Ident = input.parse()?;
        if ident != Ident::new("to", input.cursor().span()) {
            return Err(Error::new(
                input.span(),
                "malformed delegation: missing 'to' between delegate and target",
            ));
        }

        _ = input.parse::<Token![self]>()?;
        _ = input.parse::<Token![.]>()?;

        let target: Member = input.parse()?;

        Ok(FwdDecl { delegate, target })
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Formatter};

    use proc_macro2::{Span, TokenStream};
    use quote::{quote, ToTokens};
    use syn::{Index, Member};
    use test_case::test_case;

    use crate::{
        parse::tests::new_ident,
        parse::{Delegate, FwdDecl, Method},
    };

    #[test_case(
        quote!(fn test(self) to self.tester),
        FwdDecl::new(Delegate::MethodList(vec![Method::new("test", vec!["self"], None)]), Member::Named(new_ident("tester")));
        "should parse forwarding with name"
    )]
    #[test_case(
        quote!(fn test(self) to self.42),
        FwdDecl::new(Delegate::MethodList(vec![Method::new("test", vec!["self"], None)]), Member::Unnamed(new_index(42)));
        "should parse forwarding with numeric id"
    )]
    fn should_parse_fwd_decl(input: TokenStream, want: FwdDecl) {
        let decl = syn::parse2::<FwdDecl>(input).unwrap();

        assert_eq!(decl, want)
    }

    #[test_case(
    quote!(fn test(self) ot self.tester),
    "malformed delegation: missing 'to' between delegate and target";
    "should require 'to' between delegate and target"
    )]
    fn should_fail_to_parse_fwd_decl(input: TokenStream, want: &str) {
        let err = syn::parse2::<FwdDecl>(input).unwrap_err();

        assert_eq!(err.to_string(), want)
    }

    impl FwdDecl {
        fn new(delegate: Delegate, target: Member) -> Self {
            FwdDecl { delegate, target }
        }
    }

    impl PartialEq for FwdDecl {
        fn eq(&self, other: &Self) -> bool {
            self.delegate == other.delegate && self.target == other.target
        }
    }

    impl Debug for FwdDecl {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{:?} => {}",
                self.delegate,
                self.target.to_token_stream()
            )
        }
    }

    pub fn new_index(idx: u32) -> Index {
        Index {
            index: idx,
            span: Span::call_site(),
        }
    }
}
