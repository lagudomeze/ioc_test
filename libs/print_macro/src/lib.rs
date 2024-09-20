use boot_build::{quote, BootBuilder, CompileBootstrap, CrateBuilder, ItemStructExt, SynPath, ToTokens, Visit};

pub struct PrintBootstrap;

pub struct CratePrint(Vec<String>);

impl Visit for CratePrint {
    fn item_struct(&mut self, item: &ItemStructExt<'_>) {
        let path = item.build_path(&item.ident);
        self.0.push(format!("{path:?}"));
    }
}

impl CrateBuilder for CratePrint {
    fn into_token_stream(self) -> impl ToTokens {
        let strings = self.0;

        quote! {
            pub fn push_for_print(t: &mut Vec<String>) {
                #(t.push(#strings);)*
            }
        }
    }
}

pub struct BootPrint(Vec<SynPath>);

impl BootBuilder for BootPrint {
    fn append_crate(&mut self, crate_name: &SynPath) {
        self.0.push(crate_name.clone());
    }

    fn into_token_stream(self) -> impl ToTokens {
        let crates = self.0;

        quote! {
            {
                let mut printer = print::Print(Vec::new());
                #(#crates::push_for_print(&mut print);)*
                printer.run();
            }
        }
    }
}

impl CompileBootstrap for PrintBootstrap {
    type CrateBuilder = CratePrint;
    type BootBuilder = BootPrint;

    fn crate_build() -> Self::CrateBuilder {
        CratePrint(Vec::new())
    }

    fn boot_build() -> Self::BootBuilder {
        BootPrint(Vec::new())
    }
}

