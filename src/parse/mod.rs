pub use decl::FwdDecl;
pub use delegate::Delegate;
pub use method::Method;

mod decl;
mod delegate;
mod method;

#[cfg(test)]
mod tests {
    use proc_macro2::{Ident, Span};

    pub fn new_ident(str: &str) -> Ident {
        Ident::new(str, Span::call_site())
    }
}
