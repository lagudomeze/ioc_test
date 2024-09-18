use crate::{
    quote,
    Bootstrap,
    CompileBootstrap,
    ItemImplExt,
    ItemStructExt,
    RuntimeBootstrap,
    SynPath,
    ToTokens,
    Visit,
};

pub struct Bootstraps<T, U> {
    lft: T,
    rht: U,
}

impl<T, U> Bootstraps<T, U> {
    pub fn new() -> Self {
        Self {
            lft,
            rht,
        }
    }

    pub fn append<V>(self, rht: V) -> Self {
        Self {
            lft: self,
            rht,
        }
    }
}

impl<T, U> Visit for Bootstraps<T, U>
where
    T: CompileBootstrap,
    U: CompileBootstrap,
{
    fn item_struct(&mut self, i: &ItemStructExt<'_>) {
        self.lft.item_struct(i);
        self.rht.item_struct(i);
    }

    fn item_impl(&mut self, i: &ItemImplExt<'_>) {
        self.lft.item_impl(i);
        self.rht.item_impl(i);
    }
}

impl<T, U> CompileBootstrap for Bootstraps<T, U>
where
    T: CompileBootstrap,
    U: CompileBootstrap,
{
    fn new() -> Self {
        Self {
            lft: T::new(),
            rht: U::new(),
        }
    }

    fn into_token_stream(self) -> impl ToTokens {
        let lft = self.lft;
        let rht = self.rht;
        quote! {
            #lft
            #rht
        }
    }
}

impl<T, U> RuntimeBootstrap for Bootstraps<T, U>
where
    T: RuntimeBootstrap,
    U: RuntimeBootstrap,
{
    fn new() -> Self {
        Self {
            lft: T::new(),
            rht: U::new(),
        }
    }

    fn append_crate(&mut self, crate_name: &SynPath) {
        self.lft.append_crate(crate_name);
        self.rht.append_crate(crate_name);
    }

    fn into_token_stream(self) -> impl ToTokens {
        let lft = self.lft;
        let rht = self.rht;
        quote! {
            #lft
            #rht
        }
    }
}

impl<T, U> Bootstrap for Bootstraps<T, U>
where
    T: Bootstrap,
    U: Bootstrap,
{
    type CompileBootstrap = Bootstraps<T::CompileBootstrap, U::CompileBootstrap>;
    type RuntimeBootstrap = Bootstraps<T::RuntimeBootstrap, U::RuntimeBootstrap>;
}
