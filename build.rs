use rustc_version::version;

fn main() {
    println!(
        "cargo:rustc-env=DD_RUSTC_VERSION={}",
        match version() {
            Ok(ver) => ver.to_string(),
            Err(_) => "0.0.0".to_owned(),
        }
    );
    println!("cargo:rerun-if-changed=src/");
}
