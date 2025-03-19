#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::{ c_char, c_void };

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAcbInfo;

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExAcbCueTypeTag {
	    CRIATOMEXACB_CUE_TYPE_POLYPHONIC = (0),		
	    CRIATOMEXACB_CUE_TYPE_SEQUENTIAL,			
	    CRIATOMEXACB_CUE_TYPE_SHUFFLE,				
	    CRIATOMEXACB_CUE_TYPE_RANDOM,				
	    CRIATOMEXACB_CUE_TYPE_RANDOM_NO_REPEAT,		
	    CRIATOMEXACB_CUE_TYPE_SWITCH_GAME_VARIABLE,	
	    CRIATOMEXACB_CUE_TYPE_COMBO_SEQUENTIAL,		
	    CRIATOMEXACB_CUE_TYPE_SWITCH_SELECTOR,		
	    CRIATOMEXACB_CUE_TYPE_TRACK_TRANSITION_BY_SELECTOR,		
    }

    const CRIATOMEXCATEGORY_MAX_CATEGORIES_PER_PLAYBACK: usize = 16;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExCueInfo {
        id: i32,
        ty: CriAtomExAcbCueTypeTag,
        name: *const i8,
        user_data: *const i8,
        length: i64,
        categories: [u16; CRIATOMEXCATEGORY_MAX_CATEGORIES_PER_PLAYBACK],
        num_limits: i16,
        num_blocks: i16,
        num_tracks: i16,
        num_related_waveforms: i16,
        priority: u8,
        header_visibility: u8,
        ignore_player_parameter: u8,
        probability: u8,
        pan_type: crate::player::ffi::CriAtomExPanType,
        pos3d_info: crate::three::ffi::CriAtomExCuePos3dInfo,
        game_variable_info: crate::player::ffi::CriAtomExGameVariableInfo
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExWaveformInfo;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExAcbHn(pub *mut u8);

    pub type criAtomExAcb_CalculateWorkSizeForLoadAcbData = fn(*mut c_void, i32, crate::fs::ffi::CriFsBinderHn, *const c_char) -> i32;
    pub type criAtomExAcb_CalculateWorkSizeForLoadAcbDataById = fn(*mut c_void, i32, crate::fs::ffi::CriFsBinderHn, u16) -> i32;
    pub type criAtomExAcb_LoadAcbData = fn(*mut c_void, i32, crate::fs::ffi::CriFsBinderHn, *const c_char, *mut c_void, i32) -> CriAtomExAcbHn;
    pub type criAtomExAcb_LoadAcbDataById = fn(*mut c_void, i32, crate::fs::ffi::CriFsBinderHn, u16, *mut c_void, i32) -> CriAtomExAcbHn;
    pub type criAtomExAcb_CalculateWorkSizeForLoadAcbFile = fn(crate::fs::ffi::CriFsBinderHn, *const c_char, crate::fs::ffi::CriFsBinderHn, *const c_char) -> i32;
    pub type criAtomExAcb_CalculateWorkSizeForLoadAcbFileById = fn(crate::fs::ffi::CriFsBinderHn, u16, crate::fs::ffi::CriFsBinderHn, u16) -> i32;
    pub type criAtomExAcb_LoadAcbFile = fn(crate::fs::ffi::CriFsBinderHn, *const c_char, crate::fs::ffi::CriFsBinderHn, *const c_char, *mut c_void, i32) -> CriAtomExAcbHn;
    pub type criAtomExAcb_LoadAcbFileById = fn(crate::fs::ffi::CriFsBinderHn, u16, crate::fs::ffi::CriFsBinderHn, u16, *mut c_void, i32) -> CriAtomExAcbHn;
    pub type criAtomExAcb_Release = fn(CriAtomExAcbHn);
    pub type criAtomExAcb_IsReadyToRelease = fn(CriAtomExAcbHn) -> bool;
    pub type criAtomExAcb_ReleaseAll = fn();
    pub type criAtomExAcb_GetVersion = fn(*mut c_void, i32, *mut c_void) -> u32;
    pub type criAtomExAcb_GetVersionFromFile = fn(crate::fs::ffi::CriFsBinderHn, *const c_char, *mut c_void, i32, *mut bool) -> u32;
    pub type criAtomExAcb_GetSupportedVersion = fn(*mut u32, *mut u32);
    pub type criAtomExAcb_GetNumCues = fn(CriAtomExAcbHn) -> i32;
    pub type criAtomExAcb_ExistsId = fn(CriAtomExAcbHn, i32) -> bool;
    pub type criAtomExAcb_ExistsName = fn(CriAtomExAcbHn, *const c_char) -> bool;
    pub type criAtomExAcb_ExistsIndex = fn(CriAtomExAcbHn, i32) -> bool;
    pub type criAtomExAcb_GetCueIdByIndex = fn(CriAtomExAcbHn, i32) -> i32;
    pub type criAtomExAcb_GetCueIdByName = fn(CriAtomExAcbHn, *const c_char) -> i32;
    pub type criAtomExAcb_GetCueNameByIndex = fn(CriAtomExAcbHn, i32) -> *const c_char;
    pub type criAtomExAcb_GetCueNameById = fn(CriAtomExAcbHn, i32) -> *const c_char;
    pub type criAtomExAcb_GetCueIndexById = fn(CriAtomExAcbHn, i32) -> i32;
    pub type criAtomExAcb_GetCueIndexByName = fn(CriAtomExAcbHn, *const c_char) -> i32;
    pub type criAtomExAcb_GetUserDataById = fn(CriAtomExAcbHn, i32) -> *const c_char;
    pub type criAtomExAcb_GetUserDataByName = fn(CriAtomExAcbHn, *const c_char) -> *const c_char;
    pub type criAtomExAcb_GetLengthById = fn(CriAtomExAcbHn, i32) -> i64;
    pub type criAtomExAcb_GetLengthByName = fn(CriAtomExAcbHn, *const c_char) -> i64;
    pub type criAtomExAcb_GetNumUsableAisacControlsById = fn(CriAtomExAcbHn, i32) -> i32;
    pub type criAtomExAcb_GetNumUsableAisacControlsByName = fn(CriAtomExAcbHn, *const c_char) -> i32;
    pub type criAtomExAcb_GetUsableAisacControlById = fn(CriAtomExAcbHn, i32, u16, *mut crate::acf::ffi::CriAtomExAisacControlInfo) -> bool;
    pub type criAtomExAcb_GetUsableAisacControlByName = fn(CriAtomExAcbHn, *const c_char, u16, *mut crate::acf::ffi::CriAtomExAisacControlInfo) -> bool;
    pub type criAtomExAcb_IsUsingAisacControlById = fn(CriAtomExAcbHn, i32, u32) -> bool;
    pub type criAtomExAcb_IsUsingAisacControlByName = fn(CriAtomExAcbHn, *const c_char, *const c_char) -> bool;
    pub type criAtomExAcb_GetCuePriorityById = fn(CriAtomExAcbHn, i32) -> i32;
    pub type criAtomExAcb_GetCuePriorityByName = fn(CriAtomExAcbHn, *const c_char) -> i32;
    pub type criAtomExAcb_GetWaveformInfoById = fn(CriAtomExAcbHn, i32, *mut CriAtomExWaveformInfo) -> bool;
    pub type criAtomExAcb_GetWaveformInfoByName = fn(CriAtomExAcbHn, *const c_char, *mut CriAtomExWaveformInfo) -> bool;
    pub type criAtomExAcb_GetOnMemoryAwbHandle = fn(CriAtomExAcbHn) -> crate::awb::ffi::CriAtomAwbHn;
    pub type criAtomExAcb_GetStreamingAwbHandle = fn(CriAtomExAcbHn) -> crate::awb::ffi::CriAtomAwbHn;
    pub type criAtomExAcb_GetCueInfoByName = fn(CriAtomExAcbHn, *const c_char, *mut CriAtomExCueInfo) -> bool;
    pub type criAtomExAcb_GetCueInfoById = fn(CriAtomExAcbHn, i32, *mut CriAtomExCueInfo) -> bool;
    pub type criAtomExAcb_GetCueInfoByIndex = fn(CriAtomExAcbHn, i32, *mut CriAtomExCueInfo) -> bool;
    pub type criAtomExAcb_GetNumCuePlayingCountByName = fn(CriAtomExAcbHn, *const c_char) -> i32;
    pub type criAtomExAcb_GetNumCuePlayingCountById = fn(CriAtomExAcbHn, i32) -> i32;
    pub type criAtomExAcb_GetNumCuePlayingCountByIndex = fn(CriAtomExAcbHn, i32) -> i32;
    pub type criAtomExAcb_GetBlockIndexByIndex = fn(CriAtomExAcbHn, i32, *const c_char) -> i32;
    pub type criAtomExAcb_GetBlockIndexById = fn(CriAtomExAcbHn, i32, *const c_char) -> i32;
    pub type criAtomExAcb_GetBlockIndexByName = fn(CriAtomExAcbHn, *const c_char, *const c_char) -> i32;
    pub type criAtomExAcb_GetAcbInfo = fn(CriAtomExAcbHn, *mut CriAtomExAcbInfo) -> bool;
    pub type criAtomExAcb_ResetCueTypeStateByName = fn(CriAtomExAcbHn, *const c_char);
    pub type criAtomExAcb_ResetCueTypeStateById = fn(CriAtomExAcbHn, i32);
    pub type criAtomExAcb_ResetCueTypeStateByIndex = fn(CriAtomExAcbHn, i32);
    pub type criAtomExAcb_AttachAwbFile = fn(CriAtomExAcbHn, crate::fs::ffi::CriFsBinderHn, *const c_char, *const c_char, *mut c_void, i32);
    pub type criAtomExAcb_DetachAwbFile = fn(CriAtomExAcbHn, *const c_char);
    pub type criAtomExAcb_CalculateWorkSizeForAttachAwbFile = fn(crate::fs::ffi::CriFsBinderHn, *const c_char) -> i32;
    pub type criAtomExAcb_GetNumAwbFileSlots = fn(CriAtomExAcbHn) -> i32;
    pub type criAtomExAcb_GetAwbFileSlotName = fn(CriAtomExAcbHn, u16) -> *const c_char;
    pub type criAtomExAcb_IsAttachedAwbFile = fn(CriAtomExAcbHn, *const c_char) -> bool;
}

use std::{
    ffi::c_void,
    mem::MaybeUninit
};

#[repr(C)]
#[derive(Debug)]
pub struct Acb;
impl Acb {
    pub(crate) fn into_handle(&self) -> ffi::CriAtomExAcbHn {
        ffi::CriAtomExAcbHn(&raw const *self as *mut u8)
    }

    pub fn new(
        acb_data: &[u8], 
        awb_binder: &crate::fs::Binder, 
        awb_path: &str, 
        work: &[u8]
    ) -> &'static Self {
        let ptr = &raw const *unsafe { crate::globals::get_criatomexplayer_setcueid_unchecked() };
        let criAtomExAcb_LoadAcbData = unsafe { std::mem::transmute::<_, ffi::criAtomExAcb_LoadAcbData>(ptr) };
        let handle = criAtomExAcb_LoadAcbData(
            acb_data.as_ptr() as *mut c_void, 
            acb_data.len() as i32,
            awb_binder.into_handle(),
            awb_path.as_ptr() as *const i8,
            work.as_ptr() as *mut c_void,
            work.len() as i32
        );
        unsafe { &mut *(handle.0 as *mut Self) }
    }

    pub fn get_cue_info_by_id(&self, id: i32) -> Option<ffi::CriAtomExCueInfo> {
        let ptr = &raw const *unsafe { crate::globals::get_criatomexacb_getcueinfobyid_unchecked() };
        let criAtomExAcb_GetCueInfoById = unsafe { std::mem::transmute::<_, ffi::criAtomExAcb_GetCueInfoById>(ptr) };
        let mut result: MaybeUninit<ffi::CriAtomExCueInfo> = MaybeUninit::uninit();
        match criAtomExAcb_GetCueInfoById(self.into_handle(), id, result.as_mut_ptr()) {
            true => Some(unsafe { result.assume_init() }),
            false => None
        }
    }
}
