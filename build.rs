fn main() {
    let target = std::env::var("TARGET").unwrap();

    if !matches!(target.as_str(), "raspi3") {
        panic!("unsupported target {}", target);
    }

    println!(r#"cargo:rustc-cfg=target="{}""#, target);

    println!("cargo:rerun-if-changed=build.rs");
}
