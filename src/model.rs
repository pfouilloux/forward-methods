use std::fmt::{Debug, Formatter};

use derive_builder::Builder;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{Member, PatType, Receiver, ReturnType};

#[derive(Builder, Clone, PartialEq)]
pub struct FwdDecl {
    #[builder(setter(custom))]
    pub delegate: Delegate,
    #[builder(setter(custom))]
    pub target: Member,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Delegate {
    MethodList(Vec<Method>),
}

#[derive(Builder, Clone)]
pub struct Method {
    #[builder(setter(custom))]
    pub ident: Ident,
    #[builder(setter(custom))]
    pub rcv: Receiver,
    #[builder(setter(custom), default = "Vec::new()")]
    pub args: Vec<PatType>,
    #[builder(setter(custom), default = "ReturnType::Default")]
    pub ret: ReturnType,
}

impl Debug for FwdDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tokens = self.target.to_token_stream();

        write!(f, "{:?} to {}", self.delegate, tokens)
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
            && eq_rcv(&self.rcv, &other.rcv)
            && eq_args(self.args.clone(), other.args.clone())
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

fn eq_args(a: Vec<PatType>, b: Vec<PatType>) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| eq_pat(a, b))
}

fn eq_ret(a: &ReturnType, b: &ReturnType) -> bool {
    a.to_token_stream().to_string() == b.to_token_stream().to_string()
}

fn eq_rcv(a: &Receiver, b: &Receiver) -> bool {
    a.reference.is_some() == b.reference.is_some()
        && a.mutability.is_some() == b.mutability.is_some()
}

fn eq_pat(a: &PatType, b: &PatType) -> bool {
    a.pat.to_token_stream().to_string() == b.pat.to_token_stream().to_string()
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use quote::{format_ident, quote, IdentFragment};
    use syn::{FnArg, Index, Member};

    use crate::model::{Delegate, FwdDeclBuilder, MethodBuilder};

    impl FwdDeclBuilder {
        pub fn named_target(&mut self, ident: &str) -> &mut Self {
            self.target = Some(Member::Named(format_ident!("{}", ident)));
            self
        }

        pub fn unnamed_target(&mut self, idx: u32) -> &mut Self {
            self.target = Some(Member::Unnamed(Index {
                index: idx,
                span: Span::call_site(),
            }));
            self
        }

        pub fn with_method(&mut self, meth: &MethodBuilder) -> &mut Self {
            if let Some(Delegate::MethodList(methods)) = &mut self.delegate {
                methods.push(meth.build().unwrap())
            } else {
                self.delegate = Some(Delegate::MethodList(vec![meth.build().unwrap()]))
            }
            self
        }
    }

    #[cfg(test)]
    impl MethodBuilder {
        pub fn ident(&mut self, name: impl IdentFragment) -> &mut Self {
            self.ident = Some(format_ident!("{}", name));
            self
        }

        pub fn rcv(&mut self) -> &mut Self {
            self.rcv = Some(syn::parse2(quote!(self)).unwrap());
            self
        }

        pub fn mut_rcv(&mut self) -> &mut Self {
            self.rcv = Some(syn::parse2(quote!(mut self)).unwrap());
            self
        }

        pub fn ref_rcv(&mut self) -> &mut Self {
            self.rcv = Some(syn::parse2(quote!(&self)).unwrap());
            self
        }

        pub fn ref_mut_rcv(&mut self) -> &mut Self {
            self.rcv = Some(syn::parse2(quote!(&mut self)).unwrap());
            self
        }

        pub fn with_arg(&mut self, arg: &str) -> &mut Self {
            if let FnArg::Typed(pt) = syn::parse_str::<FnArg>(arg).unwrap() {
                match &mut self.args {
                    None => self.args = Some(vec![pt]),
                    Some(args) => args.push(pt),
                };
            }
            self
        }

        pub fn ret(&mut self, ret: &str) -> &mut Self {
            self.ret = Some(syn::parse_str(ret).unwrap());
            self
        }
    }
}
