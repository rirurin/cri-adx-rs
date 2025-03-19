/*
use crate::globals;
use riri_mod_tools_proc::riri_hook_static;
use riri_mod_tools_rt::{ logln, sigscan_resolver };
use std::ptr::NonNull;

// ===============
// criAtomExPlayer_Create
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_Create(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    resolve_type = set_criAtomExPlayer_Create,
    calling_convention = "microsoft",
))]
riri_static!(criAtomExPlayer_Create, usize);

// ===============
// criAtomExPlayer_SetCueId
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetCueId(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetCueId: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setcueid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    signature = "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 57 48 83 EC 20 49 63 F8 48 89 D6",
    resolve_type = set_criAtomExPlayer_SetCueId,
    calling_convention = "microsoft",
))]
riri_static!(criAtomExPlayer_SetCueId, usize);
*/
