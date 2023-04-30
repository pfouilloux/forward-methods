use proc_macro::TokenStream;

use syn::parse_macro_input;

mod implement;
mod model;
mod parse;

#[proc_macro]
pub fn fwd(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as model::FwdDecl)
        .implement()
        .into()
}

#[proc_macro]
pub fn fwd_pub(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as model::FwdDecl)
        .implement_pub()
        .into()
}
