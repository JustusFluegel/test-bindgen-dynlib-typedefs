example output for:
```bash
cargo expand
```
is
```rs
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub type SSV_InitLIB2 = ::std::option::Option<
    unsafe extern "C" fn(
        pcFichierSesam: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ushort,
>;
pub struct TestLib {
    __library: ::libloading::Library,
    pub SSV_InitLIB2: unsafe extern "C" fn(
        pcFichierSesam: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ushort,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let SSV_InitLIB2 = __library.get(b"SSV_InitLIB2\0").map(|sym| *sym)?;
        Ok(TestLib { __library, SSV_InitLIB2 })
    }
    pub unsafe fn SSV_InitLIB2(
        &self,
        pcFichierSesam: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ushort {
        (self.SSV_InitLIB2)(pcFichierSesam)
    }
}
fn main() {
    let lib = unsafe { TestLib::new("./foo.dll") }.unwrap();
    let _ = unsafe { lib.SSV_InitLIB2(c"foo".as_ptr()) };
    ::core::panicking::panic("not yet implemented");
}
```
