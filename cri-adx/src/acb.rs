#![allow(non_snake_case)]
pub mod ffi {
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAcbHn(pub *mut u8);
}
