fn main() {
    let dst = cmake::Config::new("libfoo")
        .profile("Release")
        .build_target("ALL_BUILD")
        .build();

    let dst = dst.join("build/Release");
    println!("cargo:rustc-link-search=native={}", dst.display());

    cpp_build::Config::new()
        .include("libfoo")
        .build("src/lib.rs");

    println!("cargo:rustc-link-lib=static=libfoo");
}
