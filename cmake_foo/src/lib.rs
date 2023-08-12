use cpp::{cpp, cpp_class};
use std::fmt::{Display, Formatter};
use std::slice;

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

cpp_class!(pub unsafe struct Data as "std::vector<Context>");

cpp_class!(pub unsafe struct ResultInner as "Result");

#[derive(Debug)]
#[repr(C)]
pub struct Context {
    pub index: i32,
    pub ap: i32,
}

pub struct Result {
    inner: ResultInner,
}

impl Result {
    pub fn size(&self) -> usize {
        let inner = &self.inner;
        unsafe {
            cpp!([inner as "Result*"]->usize as "size_t"{
               return inner->data.size();
            })
        }
    }

    pub fn data(&self) -> &[Context] {
        let len = self.size();
        if len == 0 {
            &[]
        } else {
            let inner = &self.inner;
            unsafe {
                let pointer = cpp!([inner as "Result*"] -> *const Context as "Context*" {
                    return &((*inner).data[0]);
                });

                slice::from_raw_parts(pointer, len)
            }
        }
    }

    pub fn into_inner(self) -> Data {
        let inner = self.inner;
        unsafe {
            cpp!([inner as "Result"] -> Data as "std::vector<Context>" {
                  return inner.data;
            })
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

    pub fn get_result(&self) -> Result {
        let inner = unsafe {
            cpp!([self as "CFoo*"]->ResultInner as "Result"{
               return self->GetResult();
            })
        };
        Result { inner }
    }
}
