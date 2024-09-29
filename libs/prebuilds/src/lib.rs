mod utils;

pub trait PrebuildInCrate {
    fn new() -> Self;

    fn visit(&mut self) -> &mut impl Visit;

    fn into_token_stream(self) -> impl ToTokens;
}

pub use utils::*;

pub use prebuilds_macro::*;

pub use visit::{Visit, scan};

pub use quote::{ToTokens, TokenStreamExt};
pub use proc_macro2::TokenStream as TT;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
