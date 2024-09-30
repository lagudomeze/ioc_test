mod utils;

pub trait PrebuildInCrate: Visit {
    fn new() -> Self;

    fn into_token_stream(self) -> impl ToTokens;
}

pub use utils::*;

pub use prebuilds_macro::*;

pub use visit::{scan, Visit, *};

pub use proc_macro2::TokenStream as TT;
pub use quote::{ToTokens, TokenStreamExt, *};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
