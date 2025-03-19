#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::c_char;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExCategoryInfo;

    pub type criAtomExCategory_SetVolumeById = fn(u32, f32);
    pub type criAtomExCategory_GetVolumeById = fn(u32) -> f32;
    pub type criAtomExCategory_GetTotalVolumeById = fn(u32) -> f32;
    pub type criAtomExCategory_SetVolumeByName = fn(*const c_char, f32);
    pub type criAtomExCategory_GetVolumeByName = fn(*const c_char) -> f32;
    pub type criAtomExCategory_GetTotalVolumeByName = fn(*const c_char) -> f32;
    pub type criAtomExCategory_MuteById = fn(u32, bool);
    pub type criAtomExCategory_IsMutedById = fn(u32) -> bool;
    pub type criAtomExCategory_MuteByName = fn(*const c_char, bool);
    pub type criAtomExCategory_IsMutedByName = fn(*const c_char) -> bool;
    pub type criAtomExCategory_SoloById = fn(u32, bool, f32);
    pub type criAtomExCategory_IsSoloedById = fn(u32) -> bool;
    pub type criAtomExCategory_SoloByName = fn(*const c_char, bool, f32);
    pub type criAtomExCategory_IsSoloedByName = fn(*const c_char) -> bool;
    pub type criAtomExCategory_PauseById = fn(u32, bool);
    pub type criAtomExCategory_IsPausedById = fn(u32) -> bool;
    pub type criAtomExCategory_PauseByName = fn(*const c_char, bool);
    pub type criAtomExCategory_IsPausedByName = fn(*const c_char) -> bool;
    pub type criAtomExCategory_SetFadeInTimeById = fn(u32, u16);
    pub type criAtomExCategory_SetFadeInTimeByName = fn(*const c_char, u16);
    pub type criAtomExCategory_SetFadeOutTimeById = fn(u32, u16);
    pub type criAtomExCategory_SetFadeOutTimeByName = fn(*const c_char, u16);
    pub type criAtomExCategory_SetAisacControlById = fn(u32, u32, f32);
    pub type criAtomExCategory_SetAisacControlByName = fn(*const c_char, *const c_char, f32);
    pub type criAtomExCategory_ResetAllAisacControlById = fn(u32) -> bool;
    pub type criAtomExCategory_ResetAllAisacControlByName = fn(*const c_char) -> bool;
    pub type criAtomExCategory_AttachAisacById = fn(u32, *const c_char);
    pub type criAtomExCategory_AttachAisacByName = fn(*const c_char, *const c_char);
    pub type criAtomExCategory_DetachAisacById = fn(u32, *const c_char);
    pub type criAtomExCategory_DetachAisacByName = fn(*const c_char, *const c_char);
    pub type criAtomExCategory_DetachAisacAllById = fn(u32);
    pub type criAtomExCategory_DetachAisacAllByName = fn(*const c_char);
    pub type criAtomExCategory_GetNumAttachedAisacsById = fn(u32) -> i32;
    pub type criAtomExCategory_GetNumAttachedAisacsByName = fn(*const c_char) -> i32;
    pub type criAtomExCategory_GetAttachedAisacInfoById = fn(u32, i32, *mut crate::acf::ffi::CriAtomExAisacInfo) -> bool;
    pub type criAtomExCategory_GetAttachedAisacInfoByName = fn(*const c_char, i32, *mut u32) -> bool;
    pub type criAtomExCategory_GetCurrentAisacControlValueById = fn(u32, u32, *mut f32) -> bool;
    pub type criAtomExCategory_GetCurrentAisacControlValueByName = fn(*const c_char, *const c_char, *mut f32) -> bool;
    pub type criAtomExCategory_GetNumCuePlayingCountById = fn(u32) -> i32;
    pub type criAtomExCategory_GetNumCuePlayingCountByName = fn(*const c_char) -> i32;
    pub type criAtomExCategory_StopById = fn(u32);
    pub type criAtomExCategory_StopByName = fn(*const c_char);
    pub type criAtomExCategory_StopWithoutReleaseTimeById = fn(u32);
    pub type criAtomExCategory_StopWithoutReleaseTimeByName = fn(*const c_char);
}

#[repr(C)]
#[derive(Debug)]
pub struct Category;
impl Category {
    pub fn set_volume_by_id(id: u32, volume: f32) {
        let ptr = &raw const *unsafe { crate::globals::get_criatomexcategory_setvolumebyid_unchecked() };
        let criAtomExCategory_SetVolumeById = unsafe { std::mem::transmute::<_, ffi::criAtomExCategory_SetVolumeById>(ptr) };
        criAtomExCategory_SetVolumeById(id, volume)
    }
    pub fn get_volume_by_id(id: u32) -> f32 {
        let ptr = &raw const *unsafe { crate::globals::get_criatomexcategory_getvolumebyid_unchecked() };
        let criAtomExCategory_GetVolumeById = unsafe { std::mem::transmute::<_, ffi::criAtomExCategory_GetVolumeById>(ptr) };
        criAtomExCategory_GetVolumeById(id)
    }
}
