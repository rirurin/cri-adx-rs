#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::c_void;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExTweenConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExTweenHn(pub *mut u8);

    pub type criAtomExTween_CalculateWorkSize = fn(*const CriAtomExTweenConfig) -> i32;
    pub type criAtomExTween_Create = fn(*const CriAtomExTweenConfig, *const c_void, i32) -> CriAtomExTweenHn;
    pub type criAtomExTween_Destroy = fn(CriAtomExTweenHn) -> ();
    pub type criAtomExTween_GetValue = fn(CriAtomExTweenHn) -> f32;
    pub type criAtomExTween_MoveTo = fn(CriAtomExTweenHn, u16, f32) -> ();
    pub type criAtomExTween_MoveFrom = fn(CriAtomExTweenHn, u16, f32) -> ();
    pub type criAtomExTween_Stop = fn(CriAtomExTweenHn) -> ();
    pub type criAtomExTween_Reset = fn(CriAtomExTweenHn) -> ();
}
