#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::c_void;
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomEx3dSourceConfig;
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomEx3dSourceHn(pub *mut u8);

    pub type criAtomEx3dSource_CalculateWorkSize = fn(*const CriAtomEx3dSourceConfig) -> i32;
    pub type criAtomEx3dSource_ChangeDefaultConeParameter = fn(f32, f32, f32) -> ();
    pub type criAtomEx3dSource_ChangeDefaultMinMaxAttenuationDistance = fn(f32, f32) -> ();
    pub type criAtomEx3dSource_Create = fn(*const CriAtomEx3dSourceConfig, *mut c_void, i32) -> CriAtomEx3dSourceHn;
    pub type criAtomEx3dSource_Destroy = fn(CriAtomEx3dSourceHn) -> ();
    pub type criAtomEx3dSource_GetPosition = fn(CriAtomEx3dSourceHn) -> *const f32;
    pub type criAtomEx3dSource_ResetParameters = fn(CriAtomEx3dSourceHn) -> ();
    pub type criAtomEx3dSource_SetConeParameter = fn(CriAtomEx3dSourceHn, f32, f32, f32) -> ();
    pub type criAtomEx3dSource_SetMinMaxAttenuationDistance = fn(CriAtomEx3dSourceHn, f32, f32) -> ();
    pub type criAtomEx3dSource_SetOrientation = fn(CriAtomEx3dSourceHn, *const f32, *const f32) -> ();
    pub type criAtomEx3dSource_SetPosition = fn(CriAtomEx3dSourceHn, *const f32) -> ();
    pub type criAtomEx3dSource_SetVelocity = fn(CriAtomEx3dSourceHn, *const f32) -> ();
    pub type criAtomEx3dSource_Update = fn(CriAtomEx3dSourceHn) -> ();
    
}
