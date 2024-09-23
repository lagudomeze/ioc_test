use crate::PrebuildInCrate;
use visit::{ItemImplExt, ItemStructExt, Visit};

pub struct Prebuilds<'a> {
    visits: Vec<&'a mut dyn Visit>,
}

impl<'a> Prebuilds<'a> {
    pub fn new() -> Self {
        Self { visits: vec![] }
    }

    pub fn push(&mut self, prebuild_in_crate: &'a mut impl PrebuildInCrate) -> &mut Self {
        self.visits.push(prebuild_in_crate.visit());
        self
    }
}

impl<'a> Visit for Prebuilds<'a> {
    fn item_struct(&mut self, i: &ItemStructExt<'_>) {
        for visit in self.visits.iter_mut() {
            visit.item_struct(i)
        }
    }

    fn item_impl(&mut self, i: &ItemImplExt<'_>) {
        for visit in self.visits.iter_mut() {
            visit.item_impl(i)
        }
    }
}