use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Member, Result, Token};

use crate::model::{Delegate, FwdDecl};

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
    use proc_macro2::TokenStream;
    use quote::quote;
    use test_case::test_case;

    use crate::model::{FwdDecl, FwdDeclBuilder, MethodBuilder};

    #[test_case(
        quote!(fn test(self) to self.tester), FwdDeclBuilder::default().named_target("tester").with_method(
            MethodBuilder::default().ident("test").rcv()
        ); "should parse forwarding with name"
    )]
    #[test_case(
        quote!(fn test(self) to self.42), FwdDeclBuilder::default().unnamed_target(42).with_method(
            MethodBuilder::default().ident("test").rcv()
        ); "should parse forwarding with numeric id"
    )]
    fn should_parse_fwd_decl(input: TokenStream, want: &FwdDeclBuilder) {
        let decl = syn::parse2::<FwdDecl>(input).unwrap();

        assert_eq!(decl, want.build().unwrap())
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
}
