use proc_macro2::TokenStream;
use quote::quote;
use syn::{Pat, PatType, Receiver};

use crate::model::{Delegate, FwdDecl};

impl FwdDecl {
    pub fn implement(&self) -> TokenStream {
        let impls = self.implement_delegates();

        quote!(#(#impls) *)
    }

    pub fn implement_pub(&self) -> TokenStream {
        let impls = self.implement_delegates();

        quote!(#(pub #impls) *)
    }

    fn implement_delegates(&self) -> Vec<TokenStream> {
        match &self.delegate {
            Delegate::MethodList(meths) => meths.iter().map(|meth| {
                let member = &self.target;
                let name = &meth.ident;
                let args = quote_args(&meth.rcv, &meth.args);
                let arg_names = quote_arg_names(&meth.args);
                let ret = &meth.ret;

                let clone = if meth.rcv.reference.is_some() {
                    quote!(.clone())
                } else {
                    quote!()
                };

                quote!(fn #name(#args) #ret { self.#member.#name(#arg_names)#clone })
            }),
        }
        .collect()
    }
}

fn quote_args(rcv: &Receiver, args: &Vec<PatType>) -> TokenStream {
    if args.is_empty() {
        quote!(#rcv)
    } else {
        quote!(#rcv, #(#args),*)
    }
}

fn quote_arg_names(args: &Vec<PatType>) -> TokenStream {
    let pats: Vec<Box<Pat>> = args.iter().map(|x| x.pat.clone()).collect();
    quote!(#(#pats),*)
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::quote;
    use test_case::test_case;

    use crate::model::{FwdDeclBuilder, MethodBuilder};

    #[test_case(
        FwdDeclBuilder::default().named_target("tester").with_method(MethodBuilder::default().ident("test").rcv()),
        quote!(fn test(self) { self.tester.test() });
        "should implement method forwarding to named member"
    )]
    #[test_case(
        FwdDeclBuilder::default().unnamed_target(42).with_method(MethodBuilder::default().ident("test").rcv()),
        quote!(fn test(self) { self.42.test() });
        "should implement method forwarding to unnamed member"
    )]
    #[test_case(
        FwdDeclBuilder::default().unnamed_target(42).with_method(
            MethodBuilder::default().ident("test").with_arg("arg1: u8").with_arg("arg2: &str").rcv()
        ),
        quote!(fn test(self, arg1: u8, arg2: &str) { self.42.test(arg1, arg2) });
        "should implement method with arguments forwarding to unnamed member"
    )]
    #[test_case(
        FwdDeclBuilder::default().unnamed_target(42).with_method(
            MethodBuilder::default().ident("test").rcv().ret("-> String")
        ),
        quote!(fn test(self) -> String { self.42.test() });
        "should implement method with return value forwarding to unnamed member"
    )]
    #[test_case(
        FwdDeclBuilder::default().unnamed_target(42).with_method(
            MethodBuilder::default().ident("test").ref_rcv().ret("-> String")
        ),
        quote!(fn test(&self) -> String { self.42.test().clone() });
        "should implement method with return value forwarding to unnamed member reference"
    )]
    fn should_write_forwarding_impl(input: &FwdDeclBuilder, want: TokenStream) {
        let decl = input.build().unwrap();

        assert_eq!(decl.implement().to_string(), want.to_string())
    }

    #[test]
    fn should_write_public_forwarding_impl() {
        let decls = FwdDeclBuilder::default()
            .named_target("tester")
            .with_method(MethodBuilder::default().ident("test1").rcv())
            .with_method(MethodBuilder::default().ident("test2").rcv())
            .build()
            .unwrap();

        let want = quote!(
            pub fn test1(self) {
                self.tester.test1()
            }
            pub fn test2(self) {
                self.tester.test2()
            }
        );

        assert_eq!(decls.implement_pub().to_string(), want.to_string())
    }
}
