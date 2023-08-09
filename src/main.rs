use cpp::{cpp, cpp_class};
use std::ffi::CStr;

cpp! {{
#include "cpp/Foo.h"
#include <string>
}}

cpp_class!(pub unsafe struct CXXString as "std::string");

impl CXXString {
    pub fn to_string(&self) -> String {
        unsafe {
            let str = cpp!([self as "std::string*"]->* const std::ffi::c_char as "const char*"{
               return self->c_str();
            });

            let c_str: &CStr =  CStr::from_ptr(str);
            let str_slice: &str = c_str.to_str().unwrap();
            str_slice.to_owned()
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
            return cpp!([self as "Foo*"]->i32 as "int"{
               return self->get_size();
            });
        }
    }
    pub fn get_name(&self) -> CXXString {
        unsafe {
            return cpp!([self as "Foo*"]->CXXString as "std::string"{
               return self->get_name();
            });
        }
    }
}

fn main() {
    loop {
        let f = Foo::new();
        println!("{}", f.get_size());
        let name = f.get_name();
        println!("{}", name.to_string());
        drop(f);
    }
}
