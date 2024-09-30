use cargo_metadata::{CargoOpt, DependencyKind, Metadata, MetadataCommand, Package};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TT};
use quote::{quote, ToTokens};
use serde_json::Value;
use std::{
    collections::HashMap,
    env::{
        var,
        var_os,
    },
    path::PathBuf,
};
use syn::{parse_quote, Ident, LitStr, Path as SynPath};

#[proc_macro]
pub fn prebuilds(_: TokenStream) -> TokenStream {

    let modules = collect_prebuilds_modules();

    let mut prebuilds_init = TT::new();

    let mut prebuilds_meta = TT::new();

    for PrebuildItem { item: path, meta } in modules.iter() {
        let tt = quote! {
            let mut __prebuilds__ = __prebuilds__.push(#path::new());
        };
        tt.to_tokens(&mut prebuilds_init);

        let tt = quote! {
            println!("cargo::metadata=bootstrap_items={}", #meta);
        };
        tt.to_tokens(&mut prebuilds_meta);
    }

    let tt = quote! {
        {
            use std::{path::{PathBuf, Path}, fs, env::var_os};
            use prebuilds::*;

            let mut __prebuilds__ = new_prebuilds();

            #prebuilds_init;

            let mut target : PathBuf = std::env::var_os("OUT_DIR")
                .expect("No OUT_DIR env")
                .into();

            target.push("bootstrap.rs");

            __prebuilds__.generate("./src/lib.rs", &target);

            #prebuilds_meta
        }
    };
    tt.into()
}

#[derive(Debug)]
struct PrebuildItem {
    item: SynPath,
    meta: LitStr,
}

fn get_manifest_path() -> Option<PathBuf> {
    let mut manifest: PathBuf = var_os("CARGO_MANIFEST_DIR")?.into();
    manifest.push("Cargo.toml");
    Some(manifest)
}

fn get_metadata(manifest: &PathBuf) -> Metadata {
    MetadataCommand::new()
        .manifest_path(&manifest)
        .features(CargoOpt::AllFeatures)
        .exec()
        .expect("cargo metadata failed")
}

fn get_dependencies_metadata(metadata: &Metadata) -> HashMap<&str, &Package> {
    let mut package_map = HashMap::new();
    for package in &metadata.packages {
        package_map.insert(package.name.as_str(), package);
    }
    package_map
}

fn process_prebuilds_items<'a>(package: &'a Package, dependency_name: &'a str, items: &mut Vec<PrebuildItem>) {
    if let Some(prebuilds) = package.metadata.get("prebuilds") {
        let package_name = dependency_name;
        let module_name = Ident::new(package_name, Span::call_site());

        if let Some(map) = prebuilds.as_object() {
            // todo make it an struct
            if let Some(name) = map.get("name")
                .map(Value::as_str)
                .flatten() {
                let meta = LitStr::new(name, Span::call_site());
                let name = Ident::new(name, Span::call_site());
                let item: SynPath = parse_quote!(#module_name::#name);

                eprintln!("prebuilds: find {module_name}::{name}");
                items.push(PrebuildItem {
                    item,
                    meta,
                });
            } else {
                // todo maybe panic here!
                eprintln!("module: {package_name} has no prebuilds items");
            }
        } else {
            eprintln!("module: {package_name} has no prebuilds items");
        }
    }
}

fn collect_prebuilds_modules() -> Vec<PrebuildItem> {
    let manifest = get_manifest_path().expect("CARGO_MANIFEST_DIR not set");
    let pkg_name = var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME not set");

    let metadata = get_metadata(&manifest);

    let package_map = get_dependencies_metadata(&metadata);
    let self_package = package_map.get(pkg_name.as_str()).expect("self package not found");

    let mut prebuilds = Vec::new();

    for dep in &self_package.dependencies {
        if dep.kind == DependencyKind::Build {
            let alias_name = dep.rename
                .as_ref()
                .map(String::as_str)
                .unwrap_or_else(|| dep.name.as_str());

            if let Some(package) = package_map.get(dep.name.as_str()) {
                process_prebuilds_items(package, alias_name, &mut prebuilds);
            }
        }
    }

    prebuilds
}
