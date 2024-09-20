mod utils;

use quote::ToTokens;
use visit::Visit;

pub trait PrebuildInCrate {
    fn new() -> Self;

    fn visit(&mut self) -> &mut impl Visit;

    fn into_token_stream(self) -> impl ToTokens;
}

pub use utils::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
