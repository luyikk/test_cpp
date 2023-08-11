fn main() {
    cpp_build::Config::new()
        .opt_level(3)
        .file("cpp/Foo.cpp")
        .build("src/main.rs");

    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=Foolib");
}
