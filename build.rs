use std::process::Command;

fn main() {
    if std::env::var_os("CARGO_FEATURE_BLIS").is_some() {
        let output = Command::new("brew")
            .args(["--prefix", "blis"])
            .output()
            .expect("failed to run `brew --prefix blis`");

        let prefix = String::from_utf8(output.stdout)
            .expect("brew prefix was not valid UTF-8");

        let prefix = prefix.trim();

        println!("cargo:rustc-link-search=native={prefix}/lib");
        println!("cargo:rustc-link-lib=dylib=blis");
    }
}
