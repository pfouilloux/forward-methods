use proc_macro2::TokenStream;
use quote::quote;
use syn::{Pat, PatType, Path, Receiver, ReturnType, Type};

use crate::model::{Delegate, FwdDecl, Method};

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

                let clone = if is_rcv_ref(meth) && is_ret_val_not_option(meth) {
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

fn is_rcv_ref(meth: &Method) -> bool {
    meth.rcv.reference.is_some()
}

fn is_ret_val_not_option(meth: &Method) -> bool {
    if let ReturnType::Type(_, typ) = meth.ret.clone() {
        !is_option_type(typ.as_ref())
    } else {
        true
    }
}

fn is_option_type(typ: &Type) -> bool {
    match typ {
        Type::Path(x) => has_option_token(&x.path),
        Type::Reference(x) => is_option_type(x.elem.as_ref()),
        _ => false,
    }
}

fn has_option_token(path: &Path) -> bool {
    path.segments.iter().any(|x| x.ident == "Option")
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
    #[test_case(
        FwdDeclBuilder::default().unnamed_target(42).with_method(
            MethodBuilder::default().ident("test").ref_rcv().ret("-> Option<String>")
        ),
        quote!(fn test(&self) -> Option<String> { self.42.test() });
        "should implement method with option return value forwarding to unnamed member reference"
    )]
    #[test_case(
        FwdDeclBuilder::default().unnamed_target(42).with_method(
            MethodBuilder::default().ident("test").ref_rcv().ret("-> &Option<String>")
        ),
        quote!(fn test(&self) -> &Option<String> { self.42.test() });
        "should implement method with option reference return value forwarding to unnamed member reference"
    )]
    #[test_case(
        FwdDeclBuilder::default().unnamed_target(42).with_method(
            MethodBuilder::default().ident("test").ref_rcv().ret("-> &mut Option<String>")
        ),
        quote!(fn test(&self) -> &mut Option<String> { self.42.test() });
        "should implement method with mutable option reference return value forwarding to unnamed member reference"
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
