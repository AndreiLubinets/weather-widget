use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let key = env::var("API_KEY").expect("API_KEY is not present");
    let package_name = env::var("CARGO_PKG_NAME").expect("Unable to get the package name");

    println!("cargo:rustc-env=API_KEY={}", key);
    println!("cargo:rustc-env=PKG_NAME={}", package_name);
    println!("cargo:rerun-if-changed=build.rs");
}
