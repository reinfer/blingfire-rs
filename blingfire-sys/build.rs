use cmake::Config;

fn main() {
    let destination = Config::new("BlingFire")
        .always_configure(true)
        .define("BLING_FIRE_VERSION_MAJOR", "1")
        .define("BLING_FIRE_VERSION_MINOR", "0")
        .build_target("")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build",
        destination.display()
    );
    println!("cargo:rustc-link-lib=static=blingfiretokdll_static");
    println!("cargo:rustc-link-lib=static=fsaClient");
    println!("cargo:rustc-link-lib=stdc++");
}
