use crate::{
    quote, Bootstrap, CompileBootstrap, RuntimeBootstrap, SynPath, ToTokens, Visit,
};

pub struct NothingToDo;

impl Visit for NothingToDo {}

impl CompileBootstrap for NothingToDo {
    fn new() -> Self {
        Self
    }

    fn into_token_stream(self) -> impl ToTokens {
        quote! {}
    }
}

impl RuntimeBootstrap for NothingToDo {
    fn new() -> Self {
        Self
    }

    fn append_crate(&mut self, _crate_name: SynPath) {}

    fn into_token_stream(self) -> impl ToTokens {
        quote! {}
    }
}
