use cpp::{cpp, cpp_class};
use std::fmt::{Display, Formatter};

cpp! {{
    #include "library.h"
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

cpp_class!(pub unsafe struct CFoo as "CFoo");

impl CFoo {
    pub fn new() -> Self {
        unsafe { cpp!([] -> CFoo as "CFoo" { return CFoo(); }) }
    }
    pub fn get_size(&self) -> i32 {
        unsafe {
            cpp!([self as "CFoo*"]->i32 as "int"{
               return self->get_size();
            })
        }
    }
    pub fn get_name(&self) -> CXXString {
        unsafe {
            cpp!([self as "CFoo*"]->CXXString as "std::string"{
               return self->get_name();
            })
        }
    }
}
