fn main() {
    cpp_build::Config::new()
        .file("cpp/Foo.cpp")
        .build("src/main.rs")
}
