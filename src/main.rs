use cmake_foo::{CFoo, CXXString};
use cpp::{cpp, cpp_class};

cpp! {{
#include "cpp/Foo.h"
}}

cpp_class!(pub unsafe struct Foo as "Foo");

impl Foo {
    pub fn new() -> Self {
        unsafe { cpp!([] -> Foo as "Foo" { return Foo(); }) }
    }
    pub fn get_size(&self) -> i32 {
        unsafe {
            cpp!([self as "Foo*"]->i32 as "int"{
               return self->get_size();
            })
        }
    }
    pub fn get_name(&self) -> CXXString {
        unsafe {
            cpp!([self as "Foo*"]->CXXString as "std::string"{
               return self->get_name();
            })
        }
    }
}

fn main() {
    loop {
        let f = Foo::new();
        println!("{}", f.get_size());
        let name = f.get_name();
        println!("{}", name);
        drop(f);

        let f = CFoo::new();
        println!("{}", f.get_size());
        let name = f.get_name();
        println!("{}", name);
        drop(f);
    }
    // output

    // 1002
    // foo
    // drop
}
