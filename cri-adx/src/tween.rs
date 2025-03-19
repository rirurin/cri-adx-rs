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

#[repr(C)]
#[derive(Debug)]
pub struct Tween;
impl Tween {
    fn into_handle(&self) -> ffi::CriAtomExTweenHn {
        ffi::CriAtomExTweenHn(&raw const *self as *mut u8)
    }
    pub fn reset(&mut self) {
        let ptr = &raw const *unsafe { crate::globals::get_criatomextween_reset_unchecked() };
        let criAtomExTween_Reset = unsafe { std::mem::transmute::<_, ffi::criAtomExTween_Reset>(ptr) };
        criAtomExTween_Reset(self.into_handle())
    }
    pub fn stop(&mut self) {
        let ptr = &raw const *unsafe { crate::globals::get_criatomextween_stop_unchecked() };
        let criAtomExTween_Stop = unsafe { std::mem::transmute::<_, ffi::criAtomExTween_Stop>(ptr) };
        criAtomExTween_Stop(self.into_handle())
    }
}
