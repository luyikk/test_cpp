use cpp::{cpp, cpp_class};
use std::fmt::{Display, Formatter};

cpp! {{
#include "cpp/Foo.h"
#include <string>
}}

cpp_class!(pub unsafe struct CXXString as "std::string");

impl Display for CXXString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl CXXString {
    pub fn as_str(&self) -> &str {
        use std::ffi::CStr;
        unsafe {
            let str = cpp!([self as "std::string*"]->* const std::ffi::c_char as "const char*"{
               return self->c_str();
            });

            let c_str: &CStr = CStr::from_ptr(str);
            c_str.to_str().unwrap()
        }
    }
}

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
    }
    // output

    // 1002
    // foo
    // drop
}
