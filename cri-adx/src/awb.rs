#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::{ c_char, c_void };
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomAwbHn(pub *mut u8);

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomAwbType {
	    CRIATOMAWB_TYPE_TOC = 0,	
	    CRIATOMAWB_TYPE_ONMEMORY,	
	    CRIATOMAWB_TYPE_ERROR,		
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomAwbStatus {
	    CRIATOMAWB_STATUS_STOP = 0,		
	    CRIATOMAWB_STATUS_LOADING,		
	    CRIATOMAWB_STATUS_COMPLETE,		
	    CRIATOMAWB_STATUS_ERROR,		
    }

    pub type criAtomAwb_CalculateWorkSizeForLoadToc = fn(i32) -> i32;
    pub type criAtomAwb_LoadToc = fn(crate::fs::ffi::CriFsBinderHn, *const c_char, *mut c_void, i32) -> CriAtomAwbHn;
    pub type criAtomAwb_LoadTocById = fn(crate::fs::ffi::CriFsBinderHn, u16, *mut c_void, i32) -> CriAtomAwbHn;
    pub type criAtomAwb_LoadTocAsync = fn(crate::fs::ffi::CriFsBinderHn, *const c_char, *mut c_void, i32) -> CriAtomAwbHn;
    pub type criAtomAwb_LoadTocAsyncById = fn(crate::fs::ffi::CriFsBinderHn, u16, *mut c_void, i32) -> CriAtomAwbHn;
    pub type criAtomAwb_LoadFromMemory = fn(*mut c_void, i32, *mut c_void, i32) -> CriAtomAwbHn;
    pub type criAtomAwb_GetType = fn(CriAtomAwbHn) -> crate::awb::ffi::CriAtomAwbType;
    pub type criAtomAwb_GetWaveFileInfo = fn(CriAtomAwbHn, i32, *mut i64, *mut u32) -> bool;
    pub type criAtomAwb_GetWaveDataInfo = fn(CriAtomAwbHn, i32, *mut *mut c_void, *mut u32);
    pub type criAtomAwb_GetNumContents = fn(CriAtomAwbHn) -> u16;
    pub type criAtomAwb_Release = fn(CriAtomAwbHn);
    pub type criAtomAwb_IsReadyToRelease = fn(CriAtomAwbHn) -> bool;
    pub type criAtomAwb_GetStatus = fn(CriAtomAwbHn) -> CriAtomAwbStatus;
}
use std::ffi::{ c_char, c_void };

#[repr(C)]
#[derive(Debug)]
pub struct Awb;
impl Awb {
    pub(crate) fn into_handle(&self) -> ffi::CriAtomAwbHn {
        ffi::CriAtomAwbHn(&raw const *self as *mut u8)
    }

    pub fn new(
        binder: &crate::fs::Binder,
        path: &str,
        work: &[u8]
    ) -> &'static Self {
        let ptr = unsafe { &raw const *crate::globals::get_criatomawb_loadtoc_unchecked() };
        let criAtomAwb_LoadToc = unsafe { std::mem::transmute::<_, ffi::criAtomAwb_LoadToc>(ptr) };
        let handle = criAtomAwb_LoadToc(binder.into_handle(), path.as_ptr() as *const c_char, work.as_ptr() as *mut c_void, work.len() as i32);
        unsafe { &mut *(handle.0 as *mut Self) }
    }
}
