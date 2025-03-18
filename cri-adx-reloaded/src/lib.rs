#![allow(non_upper_case_globals)]
pub mod globals;

use riri_mod_tools_proc::riri_hook_static;
use riri_mod_tools_rt::{ logln, sigscan_resolver };
use std::ptr::NonNull;

/*
#[no_mangle]
pub unsafe extern "C" fn set_cri_atom_ex_player_create(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_Create: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_create(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    signature = "48 89 5C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 55 41 54 41 55 41 56 41 57 48 8B EC 48 83 EC 40 45 33 E4",
    resolve_type = set_cri_atom_ex_player_create,
    calling_convention = "microsoft",
))]
riri_static!(CRIATOMEXPLAYER_CREATE, usize);
*/

/*
#[no_mangle]
pub unsafe extern "C" fn set_cri_atom_ex_player_create(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_Create: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_create(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    resolve_type = set_cri_atom_ex_player_create,
    calling_convention = "microsoft",
    shared_scan = "consumer"
))]
riri_static!(criAtomExPlayer_Create, usize);
*/

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
