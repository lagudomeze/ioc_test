use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse,
    parse::ParseStream,
    parse_macro_input,
    punctuated::Punctuated,
    Path as SynPath,
    Result as SynResult,
    Token,
};

struct InputBootstrapItems {
    paths: Punctuated<SynPath, Token![,]>
}

impl Parse for InputBootstrapItems {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(Self {
            paths: Punctuated::<SynPath, Token![,]>::parse_terminated(input)?
        })
    }
}

#[proc_macro]
pub fn bootstrap(tokens: TokenStream) -> TokenStream {
    let items = parse_macro_input!(tokens as InputBootstrapItems);
    let paths = items.paths.iter();

    // here all items is fn without param and no return, only print some str!;

    let tt = quote! {
        #(#paths();)*
    };

    tt.into()
}