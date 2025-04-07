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

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomEx3dSourceListHn(pub *mut u8);

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomEx3dListenerHn(pub *mut u8);

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomEx3dListenerListHn(pub *mut u8);

    const CRIATOMEX3DSOURCE_MAX_RANDOM_POSITION_CALCULATION_PARAMETERS: usize = 3;
    
    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExCuePos3dInfoRandomPosition {
        follows_original_source: bool,
        calculation_type: i32,
        calculation_parameters: [f32; CRIATOMEX3DSOURCE_MAX_RANDOM_POSITION_CALCULATION_PARAMETERS]
    } 

    #[repr(C)]
    #[derive(Debug)]
    pub struct CriAtomExCuePos3dInfo {
	    cone_inside_angle: f32,		
	    cone_outside_angle: f32,		
	    min_distance: f32,			
	    max_distance: f32,			
	    source_radius: f32,			
	    interior_distance: f32,		
	    doppler_factor: f32,			
        random_position: CriAtomExCuePos3dInfoRandomPosition,
        distance_aisac_control: u32,					
	    listener_base_angle_aisac_control: u32,		
	    source_base_angle_aisac_control: u32,		
	    listener_base_elevation_aisac_control: u32,	
	    source_base_elevation_aisac_control: u32,	
    }

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

#[repr(C)]
#[derive(Debug)]
pub struct Source3d;
impl Source3d {
    fn into_handle(&self) -> ffi::CriAtomEx3dSourceHn {
        ffi::CriAtomEx3dSourceHn(&raw const *self as *mut u8)
    }

    pub fn set_position(&self, position: glam::Vec3) {
        let ptr = unsafe { &raw const *crate::globals::get_criatomex3dsource_setposition_unchecked() };
        let criAtomEx3dSource_SetPosition = unsafe { std::mem::transmute::<_, ffi::criAtomEx3dSource_SetPosition>(ptr) };
        criAtomEx3dSource_SetPosition(self.into_handle(), &raw const position as *const f32)
    }
}
