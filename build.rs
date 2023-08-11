fn main() {
    cpp_build::Config::new()
        .file("cpp/Foo.cpp")
        .build("src/main.rs");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=native=lib");
        println!("cargo:rustc-link-lib=static=Foolib");
    }
}
