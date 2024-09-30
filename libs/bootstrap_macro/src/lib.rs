use cargo_metadata::{CargoOpt, DependencyKind, Metadata, MetadataCommand, Package};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TT};
use quote::{quote, ToTokens};
use std::{
    collections::HashMap,
    env::{
        var,
        var_os,
    },
    path::PathBuf,
};
use syn::{
    parse_quote,
    Ident,
    Path as SynPath,
};

#[proc_macro]
pub fn bootstrap(_: TokenStream) -> TokenStream {
    let mut tt = TT::new();
    for boot in collect_bootstrap_modules() {
        let crate_name = boot.path;
        let items = boot.items;
        eprintln!("{crate_name:?}");
        let part = quote! {
            #crate_name::bootstrap! {
                #(#items,)*
            };
        };
        part.to_tokens(&mut tt);
    }
    tt.into()
}

#[derive(Debug)]
struct Bootstrap {
    path: SynPath,
    items: Vec<SynPath>,
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

fn process_bootstrap_items<'a>(package: &'a Package,
                               dependency_name: &'a str,
                               item_map: &mut HashMap<&'a str, Vec<SynPath>>,
) {
    if let Some(items) = package.metadata.get("bootstrap_items") {
        let package_name = dependency_name;
        let module_name = Ident::new(package_name, Span::call_site());
        eprintln!("bootstrap: find module: {module_name} with bootstrap_items");

        if let Some(map) = items.as_object() {
            for (key, value) in map.iter() {
                let item = value.as_str().expect("invalid bootstrap item value");
                let item = Ident::new(item, Span::call_site());
                eprintln!("bootstrap: find module: {key} {module_name}::{item}");

                let item_path: SynPath = parse_quote!(#module_name::#item);
                item_map.entry(key.as_str()).or_insert_with(Vec::new).push(item_path);
            }
        } else {
            eprintln!("bootstrap: module {package_name} has no bootstrap items");
        }
    }
}

fn process_bootstrap_modules<'a>(package: &'a Package,
                                 dependency_name: &'a str,
                                 bootstrap_map: &mut HashMap<&'a str, &'a str>) {
    if let Some(_) = package.metadata.get("prebuilds") {
        bootstrap_map.insert(&package.name, dependency_name);
    }
}

fn collect_bootstrap_modules() -> Vec<Bootstrap> {
    let manifest = get_manifest_path().expect("CARGO_MANIFEST_DIR not set");
    let pkg_name = var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME not set");

    let metadata = get_metadata(&manifest);

    let package_map = get_dependencies_metadata(&metadata);
    let self_package = package_map.get(pkg_name.as_str()).expect("self package not found");

    let mut item_map = HashMap::new();
    let mut bootstrap_map = HashMap::new();

    for dep in &self_package.dependencies {
        if dep.kind == DependencyKind::Normal {
            let alias_name = dep.rename
                .as_ref()
                .map(String::as_str)
                .unwrap_or_else(|| dep.name.as_str());

            if let Some(package) = package_map.get(dep.name.as_str()) {
                process_bootstrap_items(package, alias_name, &mut item_map);
                process_bootstrap_modules(package, alias_name, &mut bootstrap_map);
            }
        }
    }

    let mut bootstrap_vec = Vec::new();
    for (key, items) in item_map {
        if let Some(name) = bootstrap_map.get(key) {
            let module_name = Ident::new(name, Span::call_site());
            let path: SynPath = parse_quote!(#module_name);
            bootstrap_vec.push(Bootstrap { path, items });
        }
    }
    bootstrap_vec
}
