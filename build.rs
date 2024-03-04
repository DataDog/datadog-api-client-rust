use genemichaels::{format_str, FormatConfig};
use rustc_version::version;
use std::fs::{read_to_string, write};

fn main() {
    println!(
        "cargo:rustc-env=DD_RUSTC_VERSION={}",
        match version() {
            Ok(ver) => ver.to_string(),
            Err(_) => "0.0.0".to_owned(),
        }
    );
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=examples/");
    // Format every .rs file in the examples directory recursively
    let mut format_config = FormatConfig::default();
    format_config.comment_width = Some(120);
    let examples = glob::glob("examples/**/*.rs").unwrap();
    for example in examples {
        if let Ok(path) = example {
            let contents = read_to_string(path.clone()).unwrap();
            if let Ok(formatted) = format_str(contents.as_str(), &format_config) {
                write(path, formatted.rendered).unwrap();
            } else {
                println!(
                    "cargo:warning={}",
                    format!("Failed to format example file: {:?}", path)
                );
            }
        }
    }
}
