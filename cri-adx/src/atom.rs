#![allow(non_snake_case)]
pub mod ffi {
    #![allow(non_camel_case_types)]
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CriAtomFormat {
        CRIATOM_FORMAT_NONE			=0x00000000,		/*JP< なし				*/
        CRIATOM_FORMAT_ADX			=0x00000001,		/*JP< ADX				*/
        CRIATOM_FORMAT_HCA			=0x00000003,		/*JP< HCA				*/
        CRIATOM_FORMAT_HCA_MX		=0x00000004,		/*JP< HCA-MX			*/
        CRIATOM_FORMAT_WAVE			=0x00000005,		/*JP< Wave				*/
        CRIATOM_FORMAT_RAW_PCM		=0x00000006,		/*JP< Raw PCM			*/
        CRIATOM_FORMAT_AIFF			=0x00000007,		/*JP< AIFF				*/
        CRIATOM_FORMAT_VIBRATION	=0x00000008,		/*JP< 振動				*/
        CRIATOM_FORMAT_AUDIO_BUFFER	=0x00000009,		/*JP< AudioBuffer		*/
        CRIATOM_FORMAT_INSTRUMENT	=0x0000000C,		/*JP< インストゥルメント*/
        CRIATOM_FORMAT_INPUT_PORT	=0x00000100,		/*JP< 入力ポート		*/
        CRIATOM_FORMAT_MIC_INPUT	=0x00000101,		/*JP< マイク入力		*/
        CRIATOM_FORMAT_AUX_INPUT	=0x00000102,		/*JP< AUX入力			*/
        CRIATOM_FORMAT_HW1			=0x00010001,		/*JP< ハードウェア固有	*/
        CRIATOM_FORMAT_HW2			=0x00010002,		/*JP< ハードウェア固有	*/
    }
}
