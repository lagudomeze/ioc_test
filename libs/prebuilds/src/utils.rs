use crate::PrebuildInCrate;
use prettyplease::unparse;
use quote::{quote, ToTokens};
use std::{
    fs::write,
    path::Path,
};
use proc_macro2::TokenStream;
use syn::{parse_quote, File};
use visit::{scan, ItemImplExt, ItemStructExt, Visit};

pub fn new_prebuilds() -> Prebuilds<(), ()> {
    Prebuilds((), ())
}

pub struct Prebuilds<T, U>(T, U);

impl<T, U> Prebuilds<T, U> {

    pub fn push<V>(self, rht: V) -> Prebuilds<Self, V> {
        Prebuilds(self, rht)
    }
}

impl<T, U> Visit for Prebuilds<T, U>
where
    T: Visit,
    U: Visit,
{
    fn item_struct(&mut self, i: &ItemStructExt<'_>) {
        self.0.item_struct(i);
        self.1.item_struct(i);
    }

    fn item_impl(&mut self, i: &ItemImplExt<'_>) {
        self.0.item_impl(i);
        self.1.item_impl(i);
    }
}

impl PrebuildInCrate for () {
    fn new() -> Self {
        ()
    }

    fn into_token_stream(self) -> impl ToTokens {
        quote! {}
    }
}

struct Nothing;

impl ToTokens for Nothing {
    fn to_tokens(&self, _: &mut TokenStream) {

    }
}

impl PrebuildInCrate for Prebuilds<(), ()> {
    fn new() -> Self {
        Prebuilds((), ())
    }

    fn into_token_stream(self) -> impl ToTokens {
        Nothing
    }
}

impl<T, U> Prebuilds<T, U>
where
    T: PrebuildInCrate,
    U: PrebuildInCrate,
{
    fn into_file(self) -> File {
        let lft = self.0.into_token_stream();
        let rht = self.1.into_token_stream();

        let file = parse_quote! {
            #lft
            #rht
        };

        file
    }

    pub fn generate(mut self, file: impl AsRef<Path>, target: impl AsRef<Path>) {
        scan(&mut self, file.as_ref());
        let file = self.into_file();
        write(target, unparse(&file))
            .expect("Unable to write file");
    }
}