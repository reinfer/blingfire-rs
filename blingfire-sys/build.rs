use cmake::Config;

fn main() {
    let is_macos = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos");

    let mut config = Config::new("BlingFire");
    if is_macos {
        // Modern CMake dropped support for BlingFire's pre-3.5 cmake_minimum_required.
        config.define("CMAKE_POLICY_VERSION_MINIMUM", "3.5");
    }
    let destination = config
        .always_configure(true)
        .define("BLING_FIRE_VERSION_MAJOR", "1")
        .define("BLING_FIRE_VERSION_MINOR", "0")
        .build_target("blingfiretokdll_static")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build",
        destination.display()
    );
    println!("cargo:rustc-link-lib=static=blingfiretokdll_static");
    println!("cargo:rustc-link-lib=static=fsaClient");
    // macOS ships libc++, not libstdc++; Linux keeps stdc++ unchanged.
    let cxx = if is_macos { "c++" } else { "stdc++" };
    println!("cargo:rustc-link-lib={cxx}");
}
