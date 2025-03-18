#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use std::ffi::{ c_char, c_void };
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExPlayerConfig;
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExPlayerHn(pub *mut u8);
    
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CriAtomExPlaybackId(u32);
    impl CriAtomExPlaybackId {
        pub fn get_id(&self) -> u32 { self.0 }
        // CRIATOMEX_INVALID_PLAYBACK_ID
        pub fn is_invalid(&self) -> bool { self.0 == u32::MAX }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExPanType {
        CRIATOMEX_PAN_TYPE_PAN3D 			=0x00000000,
        CRIATOMEX_PAN_TYPE_3D_POS  			=0x00000001,
        CRIATOMEX_PAN_TYPE_AUTO  			=0x00000002,
        CRIATOMEX_PAN_TYPE_UNKNOWN 			= u32::MAX,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExPlayerStatus {
	    CRIATOMEXPLAYER_STATUS_STOP = 0,		/*JP< 停止中		*/
	    CRIATOMEXPLAYER_STATUS_PREP,			/*JP< 再生準備中	*/
	    CRIATOMEXPLAYER_STATUS_PLAYING,			/*JP< 再生中		*/
	    CRIATOMEXPLAYER_STATUS_PLAYEND,			/*JP< 再生完了		*/
	    CRIATOMEXPLAYER_STATUS_ERROR,			/*JP< エラーが発生	*/
    }

    pub type criAtomExPlayer_Create = fn(*const CriAtomExPlayerConfig, *mut c_void, i32) -> CriAtomExPlayerHn;
    pub type criAtomExPlayer_GetStatus = fn(CriAtomExPlayerHn) -> CriAtomExPlayerStatus;
    pub type criAtomExPlayer_Pause = fn(CriAtomExPlayerHn, bool) -> ();
    pub type criAtomExPlayer_Prepare = fn(CriAtomExPlayerHn) -> CriAtomExPlaybackId;
    pub type criAtomExPlayer_ResetParameters = fn(CriAtomExPlayerHn) -> ();
    pub type criAtomExPlayer_SetAisacControlById = fn(CriAtomExPlayerHn, u32, f32) -> ();
    pub type criAtomExPlayer_SetCueId = fn(CriAtomExPlayerHn, crate::acb::ffi::CriAtomExAcbHn, i32) -> ();
    pub type criAtomExPlayer_SetCategoryById = fn(CriAtomExPlayerHn, u32) -> ();
    pub type criAtomExPlayer_SetFile = fn(CriAtomExPlayerHn, crate::fs::ffi::CriFsBinderHn, *const c_char) -> ();
    pub type criAtomExPlayer_SetFormat = fn(CriAtomExPlayerHn, crate::atom::ffi::CriAtomFormat) -> ();
    pub type criAtomExPlayer_SetNumChannels = fn(CriAtomExPlayerHn, i32) -> ();
    pub type criAtomExPlayer_SetPanType = fn(CriAtomExPlayerHn, CriAtomExPanType) -> ();
    pub type criAtomExPlayer_SetSamplingRate = fn(CriAtomExPlayerHn, i32) -> ();
    pub type criAtomExPlayer_SetVolume = fn(CriAtomExPlayerHn, f32) -> ();
    pub type criAtomExPlayer_SetWaveId = fn(CriAtomExPlayerHn, crate::awb::ffi::CriAtomExAwbHn, i32) -> ();
    pub type criAtomExPlayer_Start = fn(CriAtomExPlayerHn) -> CriAtomExPlaybackId;
    pub type criAtomExPlayer_Update = fn(CriAtomExPlayerHn, u32) -> ();
    pub type criAtomExPlayer_UpdateAll = fn(CriAtomExPlayerHn) -> ();
    // pub type criAtomExAcb_LoadAcbData = fn(CriAtomExPlayerHn) -> u32;
    // pub type criAtomAwb_LoadToc = fn() -> u32;
    // pub type criAtomExCategory_GetVolumeById = fn() -> u32;
    // pub type criAtomExCategory_SetVolumeById = fn() -> u32;
    // pub type criAtomExAcb_GetCueInfoById = fn() -> u32;
    
}
use std::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct AtomExPlayer;
impl AtomExPlayer {
    pub fn new(
        config: *mut u8,
        work: &[u8],
    ) -> &'static Self {
        let ptr = &raw const *unsafe { crate::globals::get_criatomexplayer_create_unchecked() };
        let criAtomExPlayer_Create = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_Create>(ptr) };
        let handle = criAtomExPlayer_Create(
            config as *const ffi::CriAtomExPlayerConfig,
            work.as_ptr() as *mut c_void,
            work.len() as i32
        );
        unsafe { &mut *(handle.0 as *mut Self) }
    }
}
