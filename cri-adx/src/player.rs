#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    use bitflags::bitflags;
    use std::ffi::{ c_char, c_void };
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExPlayerConfig;

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExPlayerHn(pub *mut u8);

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomPlayerHn(pub *mut u8);
    
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CriAtomExPlaybackId(u32);
    const CRIATOMEX_INVALID_PLAYBACK_ID: u32 = u32::MAX;
    impl CriAtomExPlaybackId {
        pub fn get_id(&self) -> u32 { self.0 }
        pub fn is_invalid(&self) -> bool { self.0 == CRIATOMEX_INVALID_PLAYBACK_ID }
        pub fn set_invalid(&mut self) { self.0 = CRIATOMEX_INVALID_PLAYBACK_ID }
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
	    CRIATOMEXPLAYER_STATUS_STOP = 0,		
	    CRIATOMEXPLAYER_STATUS_PREP,			
	    CRIATOMEXPLAYER_STATUS_PLAYING,			
	    CRIATOMEXPLAYER_STATUS_PLAYEND,			
	    CRIATOMEXPLAYER_STATUS_ERROR,			
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExResumeMode {
	    CRIATOMEX_RESUME_ALL_PLAYBACK = 0,			
	    CRIATOMEX_RESUME_PAUSED_PLAYBACK = 1,		
	    CRIATOMEX_RESUME_PREPARED_PLAYBACK = 2,		
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExParameterId {
	    CRIATOMEX_PARAMETER_ID_VOLUME					=  0,	
	    CRIATOMEX_PARAMETER_ID_PITCH					=  1,	
	    CRIATOMEX_PARAMETER_ID_PAN3D_ANGLE				=  2,	
	    CRIATOMEX_PARAMETER_ID_PAN3D_DISTANCE			=  3,	
	    CRIATOMEX_PARAMETER_ID_PAN3D_VOLUME				=  4,	
	    CRIATOMEX_PARAMETER_ID_PAN_TYPE					=  5,	
	    CRIATOMEX_PARAMETER_ID_PAN_SPEAKER_TYPE			=  6,	
	    CRIATOMEX_PARAMETER_ID_PAN_CH0					=  7,	
	    CRIATOMEX_PARAMETER_ID_PAN_CH1					=  8,	
	    CRIATOMEX_PARAMETER_ID_BUS_SEND_LEVEL_0			=  9,	
	    CRIATOMEX_PARAMETER_ID_BUS_SEND_LEVEL_1			= 10,	
	    CRIATOMEX_PARAMETER_ID_BUS_SEND_LEVEL_2			= 11,	
	    CRIATOMEX_PARAMETER_ID_BUS_SEND_LEVEL_3			= 12,	
	    CRIATOMEX_PARAMETER_ID_BUS_SEND_LEVEL_4			= 13,	
	    CRIATOMEX_PARAMETER_ID_BUS_SEND_LEVEL_5			= 14,	
	    CRIATOMEX_PARAMETER_ID_BUS_SEND_LEVEL_6			= 15,	
	    CRIATOMEX_PARAMETER_ID_BUS_SEND_LEVEL_7			= 16,	
	    CRIATOMEX_PARAMETER_ID_BANDPASS_FILTER_COF_LOW	= 17,	
	    CRIATOMEX_PARAMETER_ID_BANDPASS_FILTER_COF_HIGH	= 18,	
	    CRIATOMEX_PARAMETER_ID_BIQUAD_FILTER_TYPE		= 19,	
	    CRIATOMEX_PARAMETER_ID_BIQUAD_FILTER_FREQ		= 20,	
	    CRIATOMEX_PARAMETER_ID_BIQUAD_FILTER_Q			= 21,	
	    CRIATOMEX_PARAMETER_ID_BIQUAD_FILTER_GAIN		= 22,	
	    CRIATOMEX_PARAMETER_ID_ENVELOPE_ATTACK_TIME		= 23,	
	    CRIATOMEX_PARAMETER_ID_ENVELOPE_HOLD_TIME		= 24,	
	    CRIATOMEX_PARAMETER_ID_ENVELOPE_DECAY_TIME		= 25,	
	    CRIATOMEX_PARAMETER_ID_ENVELOPE_RELEASE_TIME	= 26,	
	    CRIATOMEX_PARAMETER_ID_ENVELOPE_SUSTAIN_LEVEL	= 27,	
	    CRIATOMEX_PARAMETER_ID_START_TIME				= 28,	
	    CRIATOMEX_PARAMETER_ID_PRIORITY					= 31,	
	    CRIATOMEX_PARAMETER_ID_SILENT_MODE				= 32,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_0			= 33,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_1			= 34,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_2			= 35,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_3			= 36,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_4			= 37,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_5			= 38,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_6			= 39,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_7			= 40,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_8			= 41,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_9			= 42,	
	    CRIATOMEX_PARAMETER_ID_DSP_PARAMETER_10			= 43,	
	    CRIATOMEX_PARAMETER_ID_DSP_BYPASS_FLAG			= 44,	
    }

    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct CriAtomSoundRendererType: u32 {
            const CRIATOM_SOUND_RENDERER_ANY = 0;			
	        const CRIATOM_SOUND_RENDERER_NATIVE = 1;		
	        const CRIATOM_SOUND_RENDERER_ASR = 2;			
	        const CRIATOM_SOUND_RENDERER_EXTENDED = 3;
	        const CRIATOM_SOUND_RENDERER_HW1 = (0 << 2) | 1;		
	        const CRIATOM_SOUND_RENDERER_HW2 = (1 << 2) | 1;		
	        const CRIATOM_SOUND_RENDERER_HW3 = (2 << 2) | 1;		
	        const CRIATOM_SOUND_RENDERER_HW4 = (3 << 2) | 1;		
	        const CRIATOM_SOUND_RENDERER_HAPTIC = (0 << 2) | 3;	
	        const CRIATOM_SOUND_RENDERER_PSEUDO = (1 << 2) | 3;	
	        const CRIATOM_SOUND_RENDERER_OBJECT = (2 << 2) | 3;	
        }
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExVoiceControlMethod {
	    CRIATOMEX_PREFER_LAST = 0,				
	    CRIATOMEX_PREFER_FIRST = 1,				
	    CRIATOMEX_PREFER_DATA = 2,				
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExPanSpeakerType {
	    CRIATOMEX_PAN_SPEAKER_TYPE_4CH = 0,
	    CRIATOMEX_PAN_SPEAKER_TYPE_5CH = 1,
	    CRIATOMEX_PAN_SPEAKER_TYPE_6CH = 2,
	    CRIATOMEX_PAN_SPEAKER_TYPE_7CH = 3,
        CRIATOMEX_PAN_SPEAKER_TYPE_5_0_2CH = 4,
	    CRIATOMEX_PAN_SPEAKER_TYPE_7_0_4CH = 5,
	    CRIATOMEX_PAN_SPEAKER_TYPE_AUTO = 10,
	    CRIATOMEX_PAN_SPEAKER_TYPE_AUTO_WITH_CENTER = 11,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExPanAngleType {
	    CRIATOMEX_PAN_ANGLE_TYPE_OFFSET = 0,
	    CRIATOMEX_PAN_ANGLE_TYPE_FIX = 1,
	    CRIATOMEX_PAN_ANGLE_TYPE_AMBIENCE_MIX = 4,
	    CRIATOMEX_PAN_ANGLE_TYPE_AMBIENCE_STRAIGHT = 5,
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExSpeakerId {
	    CRIATOMEX_SPEAKER_FRONT_LEFT = 0,			
	    CRIATOMEX_SPEAKER_FRONT_RIGHT = 1,			
	    CRIATOMEX_SPEAKER_FRONT_CENTER = 2,			
	    CRIATOMEX_SPEAKER_LOW_FREQUENCY = 3,		
	    CRIATOMEX_SPEAKER_SURROUND_LEFT = 4,		
	    CRIATOMEX_SPEAKER_SURROUND_RIGHT = 5,		
	    CRIATOMEX_SPEAKER_SURROUND_BACK_LEFT = 6,	
	    CRIATOMEX_SPEAKER_SURROUND_BACK_RIGHT = 7,	
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExBiquadFilterType {
	    CRIATOMEX_BIQUAD_FILTER_TYPE_OFF = 0,			
	    CRIATOMEX_BIQUAD_FILTER_TYPE_LOWPASS = 1,		
	    CRIATOMEX_BIQUAD_FILTER_TYPE_HIGHPASS = 2,		
	    CRIATOMEX_BIQUAD_FILTER_TYPE_NOTCH = 3,			
	    CRIATOMEX_BIQUAD_FILTER_TYPE_LOWSHELF = 4,		
	    CRIATOMEX_BIQUAD_FILTER_TYPE_HIGHSHELF = 5,		
	    CRIATOMEX_BIQUAD_FILTER_TYPE_PEAKING = 6,		
    }

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomExSilentMode {
	    CRIATOMEX_SILENT_MODE_NORMAL = 0,
	    CRIATOMEX_SILENT_MODE_STOP = 1,
	    CRIATOMEX_SILENT_MODE_VIRTUAL = 2,
	    CRIATOMEX_SILENT_MODE_VIRTUAL_RETRIGGER = 3,
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExGameVariableInfo {
        name: *const i8,
        id: u32,
        value: f32
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExSoundObjectHn(pub *mut u8);

    pub type criAtomExPlayer_CalculateWorkSize = fn(*const CriAtomExPlayerConfig) -> i32;
    pub type criAtomExPlayer_Create = fn(*const CriAtomExPlayerConfig, *mut c_void, i32) -> CriAtomExPlayerHn;
    pub type criAtomExPlayer_Destroy = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_SetCueId = fn(CriAtomExPlayerHn, crate::acb::ffi::CriAtomExAcbHn, i32);
    pub type criAtomExPlayer_SetCueName = fn(CriAtomExPlayerHn, crate::acb::ffi::CriAtomExAcbHn, *const c_char);
    pub type criAtomExPlayer_SetCueIndex = fn(CriAtomExPlayerHn, crate::acb::ffi::CriAtomExAcbHn, i32);
    pub type criAtomExPlayer_SetData = fn(CriAtomExPlayerHn, *mut c_void, i32);
    pub type criAtomExPlayer_SetFile = fn(CriAtomExPlayerHn, crate::fs::ffi::CriFsBinderHn, *const c_char);
    pub type criAtomExPlayer_SetContentId = fn(CriAtomExPlayerHn, crate::fs::ffi::CriFsBinderHn, i32);
    pub type criAtomExPlayer_SetWaveId = fn(CriAtomExPlayerHn, crate::awb::ffi::CriAtomAwbHn, i32);
    pub type criAtomExPlayer_Start = fn(CriAtomExPlayerHn) -> CriAtomExPlaybackId;
    pub type criAtomExPlayer_Prepare = fn(CriAtomExPlayerHn) -> CriAtomExPlaybackId;
    pub type criAtomExPlayer_Stop = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_StopWithoutReleaseTime = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_StopAllPlayers = fn();
    pub type criAtomExPlayer_StopAllPlayersWithoutReleaseTime = fn();
    pub type criAtomExPlayer_Pause = fn(CriAtomExPlayerHn, bool);
    pub type criAtomExPlayer_Resume = fn(CriAtomExPlayerHn, CriAtomExResumeMode);
    pub type criAtomExPlayer_IsPaused = fn(CriAtomExPlayerHn) -> bool;
    pub type criAtomExPlayer_GetStatus = fn(CriAtomExPlayerHn) -> CriAtomExPlayerStatus;
    pub type criAtomExPlayer_GetNumPlaybacks = fn(CriAtomExPlayerHn) -> i32;
    pub type criAtomExPlayer_GetLastPlaybackId = fn(CriAtomExPlayerHn) -> CriAtomExPlaybackId;
    pub type criAtomExPlayer_GetTime = fn(CriAtomExPlayerHn) -> i64;
    pub type criAtomExPlayer_SetFormat = fn(CriAtomExPlayerHn, crate::atom::ffi::CriAtomFormat);
    pub type criAtomExPlayer_SetNumChannels = fn(CriAtomExPlayerHn, i32);
    pub type criAtomExPlayer_SetSamplingRate = fn(CriAtomExPlayerHn, i32);
    pub type criAtomExPlayer_SetSoundRendererType = fn(CriAtomExPlayerHn, CriAtomSoundRendererType);
    pub type criAtomExPlayer_SetGroupNumber = fn(CriAtomExPlayerHn, i32);
    pub type criAtomExPlayer_SetVoiceControlMethod = fn(CriAtomExPlayerHn, CriAtomExVoiceControlMethod);
    pub type criAtomExPlayer_SetVoicePoolIdentifier = fn(CriAtomExPlayerHn, u32);
    pub type criAtomExPlayer_SetHcaMxMixerId = fn(CriAtomExPlayerHn, i32);
    pub type criAtomExPlayer_SetAsrRackId = fn(CriAtomExPlayerHn, i32);
    pub type criAtomExPlayer_SetAsrRackIdArray = fn(CriAtomExPlayerHn, *const i32, i32);
    pub type criAtomExPlayer_SetStartTime = fn(CriAtomExPlayerHn, i64);
    pub type criAtomExPlayer_SetSequencePrepareTime = fn(CriAtomExPlayerHn, u32);
    pub type criAtomExPlayer_SetSyncPlaybackId = fn(CriAtomExPlayerHn, CriAtomExPlaybackId);
    pub type criAtomExPlayer_SetPlaybackRatio = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_LimitLoopCount = fn(CriAtomExPlayerHn, i32);
    pub type criAtomExPlayer_UpdateAll = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_Update = fn(CriAtomExPlayerHn, CriAtomExPlaybackId);
    pub type criAtomExPlayer_ResetParameters = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_GetParameterFloat32 = fn(CriAtomExPlayerHn, CriAtomExParameterId) -> f32;
    pub type criAtomExPlayer_GetParameterUint32 = fn(CriAtomExPlayerHn, CriAtomExParameterId) -> u32;
    pub type criAtomExPlayer_GetParameterSint32 = fn(CriAtomExPlayerHn, CriAtomExParameterId) -> i32;
    pub type criAtomExPlayer_SetVolume = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetPitch = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetMaxPitch = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetPan3dAngle = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetPan3dInteriorDistance = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetPan3dVolume = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetPanType = fn(CriAtomExPlayerHn, CriAtomExPanType);
    pub type criAtomExPlayer_SetPanSpeakerType = fn(CriAtomExPlayerHn, CriAtomExPanSpeakerType);
    pub type criAtomExPlayer_AddMixDownCenterVolumeOffset = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_AddMixDownLfeVolumeOffset = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_ChangeDefaultPanSpeakerType = fn(CriAtomExPanSpeakerType);
    pub type criAtomExPlayer_SetPanAngleType = fn(CriAtomExPlayerHn, CriAtomExPanAngleType);
    pub type criAtomExPlayer_SetPanSpread = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetSendLevel = fn(CriAtomExPlayerHn, i32, CriAtomExSpeakerId, f32);
    pub type criAtomExPlayer_SetBusSendLevelByName = fn(CriAtomExPlayerHn, *const c_char, f32);
    pub type criAtomExPlayer_SetBusSendLevelOffsetByName = fn(CriAtomExPlayerHn, *const c_char, f32);
    pub type criAtomExPlayer_SetPanAdx1Compatible = fn(CriAtomExPlayerHn, i32, f32);
    pub type criAtomExPlayer_SetBandpassFilterParameters = fn(CriAtomExPlayerHn, f32, f32);
    pub type criAtomExPlayer_SetBiquadFilterParameters = fn(CriAtomExPlayerHn, CriAtomExBiquadFilterType, f32, f32, f32);
    pub type criAtomExPlayer_SetVoicePriority = fn(CriAtomExPlayerHn, i32);
    pub type criAtomExPlayer_SetAisacControlById = fn(CriAtomExPlayerHn, u32, f32);
    pub type criAtomExPlayer_SetAisacControlByName = fn(CriAtomExPlayerHn, *const c_char, f32);
    pub type criAtomExPlayer_ClearAisacControls = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_Set3dSourceHn = fn(CriAtomExPlayerHn, crate::three::ffi::CriAtomEx3dSourceHn);
    pub type criAtomExPlayer_Set3dSourceListHn = fn(CriAtomExPlayerHn, crate::three::ffi::CriAtomEx3dSourceListHn);
    pub type criAtomExPlayer_Set3dListenerHn = fn(CriAtomExPlayerHn, crate::three::ffi::CriAtomEx3dListenerHn);
    pub type criAtomExPlayer_GetAisacControlById = fn(CriAtomExPlayerHn, u32) -> f32;
    pub type criAtomExPlayer_GetAisacControlByName = fn(CriAtomExPlayerHn, *const c_char) -> f32;
    pub type criAtomExPlayer_SetCategoryById = fn(CriAtomExPlayerHn, u32);
    pub type criAtomExPlayer_SetCategoryByName = fn(CriAtomExPlayerHn, *const c_char);
    pub type criAtomExPlayer_UnsetCategory = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_GetNumCategories = fn(CriAtomExPlayerHn) -> i32;
    pub type criAtomExPlayer_GetCategoryInfo = fn(CriAtomExPlayerHn, u16, *mut crate::category::ffi::CriAtomExCategoryInfo) -> bool;
    pub type criAtomExPlayer_SetTrackInfo = fn(CriAtomExPlayerHn, i32, *const i32);
    pub type criAtomExPlayer_SetTrackVolume = fn(CriAtomExPlayerHn, i32, f32);
    pub type criAtomExPlayer_SetSilentMode = fn(CriAtomExPlayerHn, CriAtomExSilentMode);
    pub type criAtomExPlayer_SetCuePriority = fn(CriAtomExPlayerHn, i32);
    pub type criAtomExPlayer_SetPreDelayTime = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetEnvelopeAttackTime = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetEnvelopeHoldTime = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetEnvelopeDecayTime = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetEnvelopeReleaseTime = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetEnvelopeSustainLevel = fn(CriAtomExPlayerHn, f32);
    pub type criAtomExPlayer_SetRandomSeed = fn(CriAtomExPlayerHn, u32);
    pub type criAtomExPlayer_SetDspParameter = fn(CriAtomExPlayerHn, i32, f32);
    pub type criAtomExPlayer_AttachAisac = fn(CriAtomExPlayerHn, *const c_char);
    pub type criAtomExPlayer_AttachAisacByIndex = fn(CriAtomExPlayerHn, u16);
    pub type criAtomExPlayer_DetachAisac = fn(CriAtomExPlayerHn, *const c_char);
    pub type criAtomExPlayer_DetachAisacByIndex = fn(CriAtomExPlayerHn, u16);
    pub type criAtomExPlayer_DetachAisacAll = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_GetNumAttachedAisacs = fn(CriAtomExPlayerHn) -> i32;
    pub type criAtomExPlayer_GetAttachedAisacInfo = fn(CriAtomExPlayerHn, i32, *mut crate::acf::ffi::CriAtomExAisacInfo) -> bool;
    pub type criAtomExPlayer_SetStreamingCacheId = fn(CriAtomExPlayerHn, *mut u32);
    pub type criAtomExPlayer_AttachTween = fn(CriAtomExPlayerHn, crate::tween::ffi::CriAtomExTweenHn);
    pub type criAtomExPlayer_DetachTween = fn(CriAtomExPlayerHn, crate::tween::ffi::CriAtomExTweenHn);
    pub type criAtomExPlayer_DetachTweenAll = fn(CriAtomExPlayerHn);
    pub type criAtomExPlayer_SetFirstBlockIndex = fn(CriAtomExPlayerHn, u32);
    pub type criAtomExPlayer_GetSoundObject = fn(CriAtomExPlayerHn) -> CriAtomExSoundObjectHn;
    pub type criAtomExPlayer_SetDrySendLevel = fn(CriAtomExPlayerHn, CriAtomExSpeakerId, f32, f32);
    pub type criAtomExPlayer_SetSelectorLabel = fn(CriAtomExPlayerHn, *const c_char, *const c_char);
    pub type criAtomExPlayer_ClearSelectorLabels = fn(CriAtomExPlayerHn);
    
}
use std::ffi::{ c_char, c_void };

#[repr(C)]
#[derive(Debug)]
pub struct AtomExPlayer;
impl AtomExPlayer {
    // criAtomExPlayer_Create
    pub fn new(
        config: *mut u8,
        work: &[u8],
    ) -> &'static Self {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_create_unchecked() };
        let criAtomExPlayer_Create = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_Create>(ptr) };
        let handle = criAtomExPlayer_Create(
            config as *const ffi::CriAtomExPlayerConfig,
            work.as_ptr() as *mut c_void,
            work.len() as i32
        );
        unsafe { &mut *(handle.0 as *mut Self) }
    }

    fn into_handle(&self) -> ffi::CriAtomExPlayerHn {
        ffi::CriAtomExPlayerHn(&raw const *self as *mut u8)
    }

    pub fn start(&mut self) -> ffi::CriAtomExPlaybackId {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_start_unchecked() };
        let criAtomExPlayer_Start = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_Start>(ptr) };
        criAtomExPlayer_Start(self.into_handle())
    }
    pub fn set_format(&mut self, format: crate::atom::ffi::CriAtomFormat) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setformat_unchecked() };
        let criAtomExPlayer_SetFormat = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetFormat>(ptr) };
        criAtomExPlayer_SetFormat(self.into_handle(), format);
    }
    pub fn set_num_channels(&mut self, channels: i32) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setformat_unchecked() };
        let criAtomExPlayer_SetNumChannels = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetNumChannels>(ptr) };
        criAtomExPlayer_SetNumChannels(self.into_handle(), channels);
    }
    pub fn set_sample_rate(&mut self, sample_rate: i32) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setsamplingrate_unchecked() };
        let criAtomExPlayer_SetSamplingRate = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetSamplingRate>(ptr) };
        criAtomExPlayer_SetSamplingRate(self.into_handle(), sample_rate);
    }
    pub fn set_category_by_id(&mut self, category: u32) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setcategorybyid_unchecked() };
        let criAtomExPlayer_SetCategoryById = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetCategoryById>(ptr) };
        criAtomExPlayer_SetCategoryById(self.into_handle(), category);
    }
    pub fn set_volume(&mut self, volume: f32) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setvolume_unchecked() };
        let criAtomExPlayer_SetVolume = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetVolume>(ptr) };
        criAtomExPlayer_SetVolume(self.into_handle(), volume);
    }
    pub fn is_paused(&self) -> bool {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_ispaused_unchecked() };
        let criAtomExPlayer_IsPaused = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_IsPaused>(ptr) };
        criAtomExPlayer_IsPaused(self.into_handle())
    }
    pub fn pause(&mut self, sw: bool) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_pause_unchecked() };
        let criAtomExPlayer_Pause = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_Pause>(ptr) };
        criAtomExPlayer_Pause(self.into_handle(), sw)
    }
    pub fn stop(&mut self) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_stop_unchecked() };
        let criAtomExPlayer_Stop = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_Stop>(ptr) };
        criAtomExPlayer_Stop(self.into_handle())
    }
    pub fn stop_without_release_time(&mut self) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_stopwithoutreleasetime_unchecked() };
        let criAtomExPlayer_StopWithoutReleaseTime = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_StopWithoutReleaseTime>(ptr) };
        criAtomExPlayer_StopWithoutReleaseTime(self.into_handle())
    }
    pub fn reset_parameters(&mut self) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_resetparameters_unchecked() };
        let criAtomExPlayer_ResetParameters = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_ResetParameters>(ptr) };
        criAtomExPlayer_ResetParameters(self.into_handle())
    }
    pub fn set_file(&mut self, binder: &crate::fs::Binder, path: &str) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setfile_unchecked() };
        let criAtomExPlayer_SetFile = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetFile>(ptr) };
        criAtomExPlayer_SetFile(self.into_handle(), binder.into_handle(), path.as_ptr() as *const i8)
    }
    pub fn set_cue_id(&mut self, acb: &crate::acb::Acb, cue_id: i32) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setcueid_unchecked() };
        let criAtomExPlayer_SetCueId = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetCueId>(ptr) };
        criAtomExPlayer_SetCueId(self.into_handle(), acb.into_handle(), cue_id)
    }
    pub fn set_wave_id(&mut self, awb: &crate::awb::Awb, wave_id: i32) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setcueid_unchecked() };
        let criAtomExPlayer_SetWaveId = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetWaveId>(ptr) };
        criAtomExPlayer_SetWaveId(self.into_handle(), awb.into_handle(), wave_id)
    }
    pub fn set_pan_type(&mut self, pan_type: ffi::CriAtomExPanType) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setpantype_unchecked() };
        let criAtomExPlayer_SetPanType = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetPanType>(ptr) };
        criAtomExPlayer_SetPanType(self.into_handle(), pan_type)
    }
    pub fn set_aisac_control_by_id(&mut self, control_id: u32, control_value: f32) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setaisaccontrolbyid_unchecked() };
        let criAtomExPlayer_SetAisacControlById = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetAisacControlById>(ptr) };
        criAtomExPlayer_SetAisacControlById(self.into_handle(), control_id, control_value)
    }
    pub fn get_status(&self) -> ffi::CriAtomExPlayerStatus {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_getstatus_unchecked() };
        let criAtomExPlayer_GetStatus = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_GetStatus>(ptr) };
        criAtomExPlayer_GetStatus(self.into_handle()) 
    }
    pub fn is_playing(&self) -> bool {
        self.get_status() == ffi::CriAtomExPlayerStatus::CRIATOMEXPLAYER_STATUS_PLAYING
        || self.get_status() == ffi::CriAtomExPlayerStatus::CRIATOMEXPLAYER_STATUS_PREP
    }
    pub fn set_aisac_control_by_name(&mut self, control_name: &str, control_value: f32) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setaisaccontrolbyname_unchecked() };
        let criAtomExPlayer_SetAisacControlByName = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetAisacControlByName>(ptr) };
        criAtomExPlayer_SetAisacControlByName(self.into_handle(), control_name.as_ptr() as *const c_char, control_value)
    }
    // pub fn limit_loop_count(&mut self, limit: u32) {
        // let ptr = unsafe { &raw const *crate::globals::get_criatomexplayer_setaisaccontrolbyname_unchecked() };
        // let criAtomExPlayer_SetAisacControlByName = unsafe { std::mem::transmute::<_, ffi::criAtomExPlayer_SetAisacControlByName>(ptr) };
        // criAtomExPlayer_SetAisacControlByName(self.into_handle(), control_name.as_ptr() as *const c_char, control_value)
    // }
}