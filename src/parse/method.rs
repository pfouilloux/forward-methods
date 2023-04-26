use proc_macro2::{Ident, Span};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Error, FnArg, Result, ReturnType, Token,
};

pub struct Method {
    ident: Ident,
    args: Vec<FnArg>,
    ret: ReturnType,
}

impl Parse for Method {
    fn parse(input: ParseStream) -> Result<Self> {
        _ = input.parse::<Token![fn]>()?;

        let ident: Ident = input.parse()?;
        let args = parse_fn_args(input)?;
        let ret = input.parse()?;

        if let Some(FnArg::Receiver(_)) = args.first() {
            Ok(Method { ident, args, ret })
        } else {
            Err(Error::new(
                Span::call_site(),
                "method must have a receiver to be forwarded",
            ))
        }
    }
}

fn parse_fn_args(input: ParseStream) -> Result<Vec<FnArg>> {
    let args_buf;
    _ = parenthesized!(args_buf in input);

    Ok(args_buf
        .parse_terminated(FnArg::parse, Token![,])?
        .iter()
        .map(|x| x.to_owned())
        .collect())
}

#[cfg(test)]
mod test {
    use std::fmt::{Debug, Formatter};

    use proc_macro2::{Ident, TokenStream};
    use quote::{quote, ToTokens};
    use syn::{FnArg, PatType, Receiver, ReturnType, ReturnType::Default};
    use test_case::test_case;

    use crate::{
        parse::tests::{eq_idents, new_ident},
        parse::Method,
    };

    #[test_case(quote!(fn test(self)), Method::new("test", vec!["self"], None); "should parse method with move receiver")]
    #[test_case(quote!(fn test(mut self)), Method::new("test", vec!["mut self"], None); "should parse method with mut move receiver")]
    #[test_case(quote!(fn test(&self)), Method::new("test", vec!["&self"], None); "should parse method with ref receiver")]
    #[test_case(quote!(fn test(&mut self)), Method::new("test", vec!["&mut self"], None); "should parse method with mut ref receiver")]
    #[test_case(quote!(fn test(self, val: String)), Method::new("test", vec!["self", "val: String"], None); "should parse method with multiple params")]
    #[test_case(quote!(fn test(self) -> Result<String, Error>), Method::new("test", vec!["self"], Some("Result<String, Error>")); "should parse method with return value")]
    #[test_case(quote!(fn test(self) -> (String, uint)), Method::new("test", vec!["self"], Some("(String, uint)")); "should parse method with tuple return value")]
    fn should_parse_method(input: TokenStream, want: Method) {
        let meth = syn::parse2::<Method>(input).unwrap();

        assert_eq!(meth, want)
    }

    #[test_case(quote!(fn test()), "method must have a receiver to be forwarded"; "should require a receiver")]
    fn should_fail_to_parse_method(input: TokenStream, want: &str) {
        let err = syn::parse2::<Method>(input).unwrap_err();

        assert_eq!(err.to_string(), want)
    }

    impl Method {
        pub fn new(ident: &str, strings: Vec<&str>, ret: Option<&str>) -> Self {
            Method {
                ident: new_ident(ident),
                args: new_args(strings),
                ret: new_ret_val(ret),
            }
        }
    }

    impl PartialEq for Method {
        fn eq(&self, other: &Self) -> bool {
            eq_idents(&self.ident, &other.ident)
                && vec_fn_args_eq(self.args.clone(), other.args.clone())
                && eq_ret(&self.ret, &other.ret)
        }
    }

    impl Debug for Method {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut args = Vec::<String>::new();
            for arg in &self.args {
                args.push(arg.to_token_stream().to_string())
            }

            write!(f, "fn {}({})", self.ident, args.join(", "))?;
            if let ReturnType::Type(_, ty) = &self.ret {
                write!(f, " -> {}", ty.to_token_stream())?
            }

            Ok(())
        }
    }

    fn new_args(input: Vec<&str>) -> Vec<FnArg> {
        let mut args = Vec::new();
        for str in input {
            args.push(syn::parse_str::<FnArg>(str).unwrap());
        }

        args
    }

    fn new_ret_val(ret: Option<&str>) -> ReturnType {
        if let Some(str) = ret {
            syn::parse_str(format!("-> {}", str).as_str()).unwrap()
        } else {
            Default
        }
    }

    fn vec_fn_args_eq(a: Vec<FnArg>, b: Vec<FnArg>) -> bool {
        a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| fn_args_eq(a, b))
    }

    pub fn eq_idents(a: &Ident, b: &Ident) -> bool {
        a == b
    }

    fn eq_ret(a: &ReturnType, b: &ReturnType) -> bool {
        a.to_token_stream().to_string() == b.to_token_stream().to_string()
    }

    fn fn_args_eq(a: &FnArg, b: &FnArg) -> bool {
        match (a, b) {
            (FnArg::Receiver(rcv_a), FnArg::Receiver(rcv_b)) => rcv_eq(rcv_a, rcv_b),
            (FnArg::Typed(pat_a), FnArg::Typed(pat_b)) => pat_eq(pat_a, pat_b),
            _ => false,
        }
    }

    fn rcv_eq(a: &Receiver, b: &Receiver) -> bool {
        a.reference.is_some() == b.reference.is_some()
            && a.mutability.is_some() == b.mutability.is_some()
    }

    fn pat_eq(a: &PatType, b: &PatType) -> bool {
        a.pat.to_token_stream().to_string() == b.pat.to_token_stream().to_string()
    }
}
