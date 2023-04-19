use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Composite)]
pub fn derive_composite(_input: TokenStream) -> TokenStream {
    let tokens = quote! {
        impl CompositeStruct {
            pub fn get_message(&self) -> String { self.0.get_message() }
            pub fn println(&self, msg: String) { println!("{}", msg) }
        }
    };

    tokens.into()
}

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn composite(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        impl CompositeStruct {
            pub fn get_message(&self) -> String { self.msg.get_message() }
            pub fn println(&self, msg: impl Into<String>) { println!("{}", msg.into()) }
        }
    };

    tokens.into()
}
