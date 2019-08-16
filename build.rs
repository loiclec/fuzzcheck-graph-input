use std::env;

fn main() {
    let lib = env::var("FUZZCHECK_LIB").unwrap();
    println!("cargo:rustc-link-search=all={}", lib);
    println!("cargo:rerun-if-changed={}", lib);
}
