use prebuilds::{quote, ItemStructExt, PrebuildInCrate, ToTokens, Visit};

pub use print_macro::bootstrap;

pub struct PrintXxxx(Vec<String>);

impl Visit for PrintXxxx {
    fn item_struct(&mut self, i: &ItemStructExt<'_>) {
        let path = i.build_path(&i.ident);
        self.0.push(format!("{path:#?}"));
    }
}

pub struct PrintFn(Vec<String>);

impl ToTokens for PrintFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let paths = &self.0;

        let tt = quote! {
            pub fn print() {
                #(println!("{}", #paths);)*
            }
        };
        tt.to_tokens(tokens);
    }
}

impl PrebuildInCrate for PrintXxxx {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn into_token_stream(self) -> impl ToTokens {
        PrintFn(self.0)
    }
}