use bootstrap::{quote, Bootstrap, CompileBootstrap, ItemStructExt, RuntimeBootstrap, SynPath, ToTokens, TokenStream, TokenStreamExt, Visit};

pub struct Print;

impl Bootstrap for Print {
    type CompileBootstrap = CompilePrintCollector;
    type RuntimeBootstrap = RuntimePrintExecutor;
}

pub struct CompilePrintCollector(Vec<SynPath>);

impl Visit for CompilePrintCollector {
    fn item_struct(&mut self, item: &ItemStructExt<'_>) {
        self.0.push(item.build_path(&item.ident));
    }
}

impl CompileBootstrap for CompilePrintCollector {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn into_token_stream(self) -> impl ToTokens {
        let types = self.0;

        quote! {
            pub fn print() {
                use std::any::type_name;
                #(println!("{}", type_name::<#types>());)
            }
        }
    }
}

pub struct RuntimePrintExecutor(Vec<TokenStream>);

impl RuntimeBootstrap for RuntimePrintExecutor {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn append_crate(&mut self, crate_name: &SynPath) {
        self.0.push(quote! {
            #crate_name::print();
        });
    }

    fn into_token_stream(self) -> impl ToTokens {
        let tts = self.0;
        quote! {
            #(#tts)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
