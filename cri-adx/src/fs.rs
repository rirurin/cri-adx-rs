#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriFsBinderHn(pub *mut u8);
}
