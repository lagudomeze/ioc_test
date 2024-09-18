pub macro_rules! export {
     () => {
         include!(concat!(env!("OUT_DIR"), "/bootstraps/src/bootstrap.rs"));
     };
     ($custom_file:literal) => {
         include!(concat!(env!("OUT_DIR"), "/bootstraps/src/", $custom_file));
     };
 }

use proc_macro::TokenStream as TT;

#[proc_macro]
pub fn export(input: TT) -> TT {

}