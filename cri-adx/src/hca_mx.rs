#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::{ c_char, c_void };
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExHcaMxConfig;

    pub type criAtomExHcaMx_CalculateWorkSize = fn(*const CriAtomExHcaMxConfig) -> i32;
    pub type criAtomExHcaMx_SetConfigForWorkSizeCalculation = fn(*const CriAtomExHcaMxConfig);
    pub type criAtomExHcaMx_Initialize = fn(*const CriAtomExHcaMxConfig, *mut c_void, i32);
    pub type criAtomExHcaMx_Finalize = fn();
    pub type criAtomExHcaMx_SetBusSendLevelByName = fn(i32, *const c_char, f32);
    pub type criAtomExHcaMx_SetFrequencyRatio = fn(i32, f32);
    pub type criAtomExHcaMx_SetAsrRackId = fn(i32, i32);
}
