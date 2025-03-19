#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::{ c_char, c_void };

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAisacControlInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAcfDspSettingInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAcfDspSettingSnapshotInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAcfDspBusInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAcfDspBusLinkInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExGlobalAisacInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAisacGraphType;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAisacGraphInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAcfInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAisacInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExSelectorInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExSelectorLabelInfo;

    pub type criAtomExAcf_GetNumAisacControls = fn() -> i32;
    pub type criAtomExAcf_GetAisacControlInfo = fn(u16, *mut CriAtomExAisacControlInfo) -> bool;
    pub type criAtomExAcf_GetAisacControlIdByName = fn(*const c_char) -> u32;
    pub type criAtomExAcf_GetAisacControlNameById = fn(u32) -> *const c_char;
    pub type criAtomExAcf_GetNumDspSettings = fn() -> i32;
    pub type criAtomExAcf_GetNumDspSettingsFromAcfData = fn(*mut c_void, i32) -> i32;
    pub type criAtomExAcf_GetDspSettingNameByIndex = fn(u16) -> *const c_char;
    pub type criAtomExAcf_GetDspSettingNameByIndexFromAcfData = fn(*mut c_void, i32, u16) -> *const c_char;
    pub type criAtomExAcf_GetDspSettingInformation = fn(*const c_char, *mut CriAtomExAcfDspSettingInfo) -> bool;
    pub type criAtomExAcf_GetDspSettingSnapshotInformation = fn(u16, *mut CriAtomExAcfDspSettingSnapshotInfo) -> bool;
    pub type criAtomExAcf_GetDspBusInformation = fn(u16, *mut CriAtomExAcfDspBusInfo) -> bool;
    pub type criAtomExAcf_GetDspFxType = fn(u16) -> u32;
    pub type criAtomExAcf_GetDspFxName = fn(u16) -> *const c_char;
    pub type criAtomExAcf_GetDspFxParameters = fn(u16, *mut c_void, i32) -> bool;
    pub type criAtomExAcf_GetDspBusLinkInformation = fn(u16, *mut CriAtomExAcfDspBusLinkInfo) -> bool;
    pub type criAtomExAcf_GetNumCategoriesFromAcfData = fn(*mut c_void, i32) -> i32;
    pub type criAtomExAcf_GetNumCategories = fn() -> i32;
    pub type criAtomExAcf_GetNumCategoriesPerPlaybackFromAcfData = fn(*mut c_void, i32) -> i32;
    pub type criAtomExAcf_GetNumCategoriesPerPlayback = fn() -> i32;
    pub type criAtomExAcf_GetCategoryInfo = fn(u16, *mut crate::category::ffi::CriAtomExCategoryInfo) -> bool;
    pub type criAtomExAcf_GetCategoryInfoByName = fn(*const c_char, *mut crate::category::ffi::CriAtomExCategoryInfo) -> bool;
    pub type criAtomExAcf_GetCategoryInfoById = fn(u32, *mut crate::category::ffi::CriAtomExCategoryInfo) -> bool;
    pub type criAtomExAcf_GetNumGlobalAisacs = fn() -> i32;
    pub type criAtomExAcf_GetGlobalAisacInfo = fn(u16, *mut CriAtomExGlobalAisacInfo) -> bool;
    pub type criAtomExAcf_GetGlobalAisacInfoByName = fn(*const c_char, *mut CriAtomExGlobalAisacInfo) -> bool;
    pub type criAtomExAcf_GetGlobalAisacGraphInfo = fn(*const CriAtomExGlobalAisacInfo, u16, *mut CriAtomExAisacGraphInfo) -> bool;
    pub type criAtomExAcf_GetGlobalAisacValue = fn(*const CriAtomExGlobalAisacInfo, f32, CriAtomExAisacGraphType, *mut f32) -> bool;
    pub type criAtomExAcf_GetAcfInfo = fn(*mut CriAtomExAcfInfo) -> bool;
    pub type criAtomExAcf_GetAcfInfoFromAcfData = fn(*mut c_void, i32, *mut CriAtomExAcfInfo) -> bool;
    pub type criAtomExAcf_GetNumSelectors = fn() -> i32;
    pub type criAtomExAcf_GetSelectorInfoByIndex = fn(u16, *mut CriAtomExSelectorInfo) -> bool;
    pub type criAtomExAcf_GetSelectorInfoByName = fn(*const c_char, *mut CriAtomExSelectorInfo) -> bool;
    pub type criAtomExAcf_GetSelectorLabelInfo = fn(*const CriAtomExSelectorInfo, u16, *mut CriAtomExSelectorLabelInfo) -> bool;
    pub type criAtomExAcf_SetGlobalLabelToSelectorByName = fn(*const c_char, *const c_char);
    pub type criAtomExAcf_SetGlobalLabelToSelectorByIndex = fn(u16, u16);
    pub type criAtomExAcf_GetNumBusesFromAcfData = fn(*mut c_void, i32) -> i32;
    pub type criAtomExAcf_GetNumBuses = fn() -> i32;
    pub type criAtomExAcf_GetMaxBusesOfDspBusSettingsFromAcfData = fn(*mut c_void, i32) -> i32;
    pub type criAtomExAcf_GetMaxBusesOfDspBusSettings = fn() -> i32;
}
