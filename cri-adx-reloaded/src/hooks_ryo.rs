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
