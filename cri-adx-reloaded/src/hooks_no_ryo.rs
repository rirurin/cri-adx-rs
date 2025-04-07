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
    signature = "40 53 48 83 EC 20 48 89 CB 48 85 C9 75 ?? 44 8D 41 ?? 48 8D 15 ?? ?? ?? ?? 48 83 C4 20 5B E9 ?? ?? ?? ?? E8 ?? ?? ?? ?? 85 C0 74 ?? 83 F8 03 75 ?? 48 8B 8B ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 63 ?? 00 83 A3 ?? ?? ?? ?? 00 C6 83 ?? ?? ?? ?? 00 66 8B 1D ?? ?? ?? ??",
    resolve_type = set_criAtomExPlayer_Stop,
    calling_convention = "microsoft",
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
    signature = "48 89 5C 24 ?? 57 48 83 EC 20 48 89 CF 48 85 C9 75 ?? BA B6 76 D3 DB",
    resolve_type = set_criAtomExPlayer_GetStatus,
    calling_convention = "microsoft",
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
    signature = "40 53 48 83 EC 20 8B D9 33 C9 E8 ?? ?? ?? ?? 85 C0 75 ?? 48 8D 15 ?? ?? ?? ?? 33 C9 E8 ?? ?? ?? ?? F3 0F 10 05",
    resolve_type = set_criAtomExCategory_GetVolumeById,
    calling_convention = "microsoft",
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
    signature = "40 53 48 83 EC 30 8B D9 0F 29 74 24 ?? 33 C9",
    resolve_type = set_criAtomExCategory_SetVolumeById,
    calling_convention = "microsoft",
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
    signature = "48 89 5C 24 ?? 57 48 83 EC 20 48 8B F9 48 85 C9 75 ?? 48 8D 15",
    resolve_type = set_criAtomExPlayer_SetFormat,
    calling_convention = "microsoft",
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
    signature = "48 89 5C 24 ?? 57 48 83 EC 20 8B FA 48 8B D9 48 85 C9 74 ?? 8D 42",
    resolve_type = set_criAtomExPlayer_SetNumChannels,
    calling_convention = "microsoft",
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
    signature = "48 89 5C 24 ?? 57 48 83 EC 20 8B FA 48 8B D9 48 85 C9 74 ?? 85 D2",
    resolve_type = set_criAtomExPlayer_SetSamplingRate,
    calling_convention = "microsoft",
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
    signature = "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 57 48 83 EC 50 48 8B F9 8B F2",
    resolve_type = set_criAtomExPlayer_SetCategoryById,
    calling_convention = "microsoft",
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
    signature = "40 53 48 83 EC 30 0F 29 74 24 ?? 0F 28 F1 48 8B D9 48 85 C9 75 ?? 48 8D 15 ?? ?? ?? ?? 41 B8 FE FF FF FF 33 C9 E8 ?? ?? ?? ?? EB ?? 0F 57 C0 F3 0F 5A C6 E8 ?? ?? ?? ?? 85 C0 74 ?? 48 8D 15 ?? ?? ?? ?? EB ?? F3 0F 11 74 24 ?? 33 D2",
    resolve_type = set_criAtomExPlayer_SetVolume,
    calling_convention = "microsoft",
))]
riri_static!(criAtomExPlayer_SetVolume, usize);

*/
