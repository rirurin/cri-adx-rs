use crate::globals;
use riri_mod_tools_proc::riri_hook_static;
use riri_mod_tools_rt::{ logln, sigscan_resolver };
use std::ptr::NonNull;

// ===============
// criAtomExPlayer_Create
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_Create(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomExPlayer_Create: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_create(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_Create,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_Create, usize);

// ===============
// criAtomExPlayer_Start
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_Start(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomExPlayer_Start: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_start(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_Start,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_Start, usize);

// ===============
// criAtomExPlayer_ResetParameters
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_ResetParameters(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomExPlayer_ResetParameters: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_resetparameters(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_ResetParameters,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_ResetParameters, usize);

// ===============
// criAtomExPlayer_SetFile
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetFile(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomExPlayer_SetFile: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setfile(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetFile,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetFile, usize);

// ===============
// criAtomExPlayer_SetCueId
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetCueId(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomExPlayer_SetCueId: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setcueid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetCueId,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetCueId, usize);

// ===============
// criAtomExPlayer_SetWaveId
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetWaveId(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomExPlayer_SetWaveId: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setwaveid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetWaveId,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetWaveId, usize);

// ===============
// criAtomExAcb_LoadAcbData
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExAcb_LoadAcbData(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomExAcb_LoadAcbData: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexacb_loadacbdata(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExAcb_LoadAcbData,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExAcb_LoadAcbData, usize);

// ===============
// criAtomAwb_LoadToc
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomAwb_LoadToc(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomAwb_LoadToc: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomawb_loadtoc(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomAwb_LoadToc,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomAwb_LoadToc, usize);

// ===============
// criAtomExAcb_GetCueInfoById
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExAcb_GetCueInfoById(ofs: usize) -> Option<NonNull<u8>> {
    let addr = sigscan_resolver::get_address(ofs);
    logln!(Information, "got criAtomExAcb_GetCueInfoById: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexacb_getcueinfobyid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExAcb_GetCueInfoById,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExAcb_GetCueInfoById, usize);

// ===============
// criAtomExPlayer_Stop
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_Stop(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_Stop: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_stop(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_Stop,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_Stop, usize);


// ===============
// criAtomExPlayer_GetStatus
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_GetStatus(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_GetStatus: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_getstatus(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_GetStatus,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_GetStatus, usize);

// ===============
// criAtomExCategory_GetVolumeById
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExCategory_GetVolumeById(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExCategory_GetVolumeById: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexcategory_getvolumebyid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExCategory_GetVolumeById,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExCategory_GetVolumeById, usize);

// ===============
// criAtomExCategory_SetVolumeById
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExCategory_SetVolumeById(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExCategory_SetVolumeById: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexcategory_setvolumebyid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExCategory_SetVolumeById,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExCategory_SetVolumeById, usize);

// ===============
// criAtomExPlayer_SetFormat
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetFormat(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetFormat: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setformat(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetFormat,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetFormat, usize);

// ===============
// criAtomExPlayer_SetNumChannels
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetNumChannels(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetNumChannels: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setnumchannels(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetNumChannels,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetNumChannels, usize);

// ===============
// criAtomExPlayer_SetSamplingRate
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetSamplingRate(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetSamplingRate: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setsamplingrate(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetSamplingRate,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetSamplingRate, usize);

// ===============
// criAtomExPlayer_SetCategoryById
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetCategoryById(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetCategoryById: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setcategorybyid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetCategoryById,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetCategoryById, usize);

// ===============
// criAtomExPlayer_SetVolume
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetVolume(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetVolume: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setvolume(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetVolume,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetVolume, usize);

// ===============
// criAtomExPlayer_SetAisacControlByName
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetAisacControlByName(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetAisacControlByName: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setaisaccontrolbyid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_criAtomExPlayer_SetAisacControlByName,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_SetAisacControlByName, usize);