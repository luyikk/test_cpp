fn main() {
    if cfg!(target_os = "windows") {
        cpp_build::Config::new().build("src/main.rs");
        println!("cargo:rustc-link-search=native=./build_foo/lib");
        println!("cargo:rustc-link-lib=static=Foolib");
    } else {
        cpp_build::Config::new()
            .file("cpp/Foo.cpp")
            .build("src/main.rs");
    }
}
