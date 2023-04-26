use proc_macro::TokenStream;

use quote::quote;

mod parse;

#[proc_macro]
pub fn fwd(input: TokenStream) -> TokenStream {
    // let decl = parse_macro_input!(input as parse::ForwardDecl);

    let tokens = quote! {
        decl
    };

    tokens.into()
}
