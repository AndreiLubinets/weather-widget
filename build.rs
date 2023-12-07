use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let key = env::var("API_KEY").expect("API_KEY is not present");
    println!("cargo:rustc-env=API_KEY={}", key);
    println!("cargo:rerun-if-changed=build.rs");
}
