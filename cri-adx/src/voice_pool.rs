#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::c_void;
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExStandardVoicePoolConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAdxVoicePoolConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExHcaVoicePoolConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExHcaMxVoicePoolConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExWaveVoicePoolConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAiffVoicePoolConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExRawPcmVoicePoolConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExVoicePoolHn(pub *mut u8);

    pub type criAtomExVoicePool_CalculateWorkSizeForStandardVoicePool = fn(*const CriAtomExStandardVoicePoolConfig) -> i32;
    pub type criAtomExVoicePool_AllocateStandardVoicePool = fn(*const CriAtomExStandardVoicePoolConfig, *mut c_void, i32) -> CriAtomExVoicePoolHn;
    pub type criAtomExVoicePool_CalculateWorkSizeForAdxVoicePool = fn(*const CriAtomExAdxVoicePoolConfig) -> i32;
    pub type criAtomExVoicePool_AllocateAdxVoicePool = fn(*const CriAtomExAdxVoicePoolConfig, *mut c_void, i32) -> CriAtomExVoicePoolHn;
    pub type criAtomExVoicePool_CalculateWorkSizeForHcaVoicePool = fn(*const CriAtomExHcaVoicePoolConfig) -> i32;
    pub type criAtomExVoicePool_AllocateHcaVoicePool = fn(*const CriAtomExHcaVoicePoolConfig, *mut c_void, i32) -> CriAtomExVoicePoolHn;
    pub type criAtomExVoicePool_CalculateWorkSizeForHcaMxVoicePool = fn(*const CriAtomExHcaMxVoicePoolConfig) -> i32;
    pub type criAtomExVoicePool_AllocateHcaMxVoicePool = fn(*const CriAtomExHcaMxVoicePoolConfig, *mut c_void, i32) -> CriAtomExVoicePoolHn;
    pub type criAtomExVoicePool_CalculateWorkSizeForWaveVoicePool = fn(*const CriAtomExWaveVoicePoolConfig) -> i32;
    pub type criAtomExVoicePool_AllocateWaveVoicePool = fn(*const CriAtomExWaveVoicePoolConfig, *mut c_void, i32) -> CriAtomExVoicePoolHn;
    pub type criAtomExVoicePool_CalculateWorkSizeForAiffVoicePool = fn(*const CriAtomExAiffVoicePoolConfig) -> i32;
    pub type criAtomExVoicePool_AllocateAiffVoicePool = fn(*const CriAtomExAiffVoicePoolConfig, *mut c_void, i32) -> CriAtomExVoicePoolHn;
    pub type criAtomExVoicePool_CalculateWorkSizeForRawPcmVoicePool = fn(*const CriAtomExRawPcmVoicePoolConfig) -> i32;
    pub type criAtomExVoicePool_AllocateRawPcmVoicePool = fn(*const CriAtomExRawPcmVoicePoolConfig, *mut c_void, i32) -> CriAtomExVoicePoolHn;
    pub type criAtomExVoicePool_Free = fn(CriAtomExVoicePoolHn);
    pub type criAtomExVoicePool_FreeAll = fn();
    pub type criAtomExVoicePool_GetNumUsedVoices = fn(CriAtomExVoicePoolHn, *mut i32, *mut i32);
    pub type criAtomExVoicePool_GetPlayerHandle = fn(CriAtomExVoicePoolHn, i32) -> crate::player::ffi::CriAtomPlayerHn;
}
