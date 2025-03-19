#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomFormat {
        CRIATOM_FORMAT_NONE			=0x00000000,		
        CRIATOM_FORMAT_ADX			=0x00000001,		
        CRIATOM_FORMAT_HCA			=0x00000003,		
        CRIATOM_FORMAT_HCA_MX		=0x00000004,		
        CRIATOM_FORMAT_WAVE			=0x00000005,		
        CRIATOM_FORMAT_RAW_PCM		=0x00000006,		
        CRIATOM_FORMAT_AIFF			=0x00000007,		
        CRIATOM_FORMAT_VIBRATION	=0x00000008,		
        CRIATOM_FORMAT_AUDIO_BUFFER	=0x00000009,		
        CRIATOM_FORMAT_INSTRUMENT	=0x0000000C,		
        CRIATOM_FORMAT_INPUT_PORT	=0x00000100,		
        CRIATOM_FORMAT_MIC_INPUT	=0x00000101,		
        CRIATOM_FORMAT_AUX_INPUT	=0x00000102,		
        CRIATOM_FORMAT_HW1			=0x00010001,		
        CRIATOM_FORMAT_HW2			=0x00010002,		
    }
}
