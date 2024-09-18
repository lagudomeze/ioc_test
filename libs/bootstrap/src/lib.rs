pub use bootstraps::Bootstraps;
pub use proc_macro2::TokenStream;
pub use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
pub use utils::NothingToDo;
pub use visit::{
    ItemImplExt, ItemStructExt, SynPath, Visit,
};

pub trait Bootstrap {
    type CompileBootstrap;
    type RuntimeBootstrap;
}

pub trait CompileBootstrap: Visit {
    fn new() -> Self;

    fn into_token_stream(self) -> impl ToTokens;
}

pub trait RuntimeBootstrap {
    fn new() -> Self;

    fn append_crate(&mut self, crate_name: &SynPath);

    fn into_token_stream(self) -> impl ToTokens;
}

mod utils;

mod bootstraps;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
