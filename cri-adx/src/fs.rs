#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriFsBinderHn(pub *mut u8);
}

#[repr(C)]
#[derive(Debug)]
pub struct Binder;
impl Binder {
    pub(crate) fn into_handle(&self) -> ffi::CriFsBinderHn {
        ffi::CriFsBinderHn(&raw const *self as *mut u8)
    }
}
