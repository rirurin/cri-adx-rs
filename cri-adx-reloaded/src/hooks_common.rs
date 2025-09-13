use crate::globals;
use riri_mod_tools_proc::riri_hook_static;
use riri_mod_tools_rt::{ logln, sigscan_resolver };
use std::ptr::NonNull;

// ===============
// criAtomExPlayer_SetPanType
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetPanType(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetPanType: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setpantype(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    signature = "45 31 C0 48 85 C9 75 ??",
    resolve_type = set_criAtomExPlayer_SetPanType,
    calling_convention = "microsoft",
))]
riri_static!(criAtomExPlayer_SetPanType, usize);

// ===============
// criAtomEx3dSource_SetPosition
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomEx3dSource_SetPosition(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomEx3dSource_SetPosition: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomex3dsource_setposition(addr.as_ptr());
    Some(addr)
}

#[no_mangle]
pub unsafe extern "C" fn set_criAtomEx3dSource_SetPosition_P5R(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    let addr2 = match sigscan_resolver::get_indirect_address_short_abs(addr.as_ptr().add(0x37)) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomEx3dSource_SetPosition: 0x{:x}", addr2.as_ptr() as usize);
    globals::set_criatomex3dsource_setposition(addr.as_ptr());
    Some(addr)
}


#[riri_hook_static({
    XRD744_STEAM_104 => dynamic_offset(
        signature = "48 83 EC 38 F3 0F 10 02 48 8D 05 ?? ?? ?? ?? F3 0F 10 4A ?? 48 63 C9 F3 0F 11 44 24 ?? F3 0F 10 42 ?? 48 8D 54 24 ?? F3 0F 11 4C 24 ?? 48 8B 0C ?? F3 0F 11 44 24 ?? E8 ?? ?? ?? ?? 31 C0",
        resolve_type = set_criAtomEx3dSource_SetPosition_P5R,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "48 89 5C 24 ?? 57 48 83 EC 20 48 89 D3 48 89 CF 48 85 C9",
        resolve_type = set_criAtomEx3dSource_SetPosition,
        calling_convention = "microsoft",
    )
})]
riri_static!(criAtomEx3dSource_SetPosition, usize);

// ===============
// criAtomExPlayer_SetAisacControlById
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_SetAisacControlById(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_SetAisacControlById: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_setaisaccontrolbyid(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static({
    XRD744_STEAM_104 => dynamic_offset(
        signature = "48 89 5C 24 ?? 57 48 83 EC 30 48 8B F9 0F 29 74 24 ?? 33 C9 0F 28 F2 8B DA E8 ?? ?? ?? ?? 85 C0 75 ?? 48 8D 15 ?? ?? ?? ?? 33 C9 E8 ?? ?? ?? ?? EB ?? 48 85 FF",
        resolve_type = set_criAtomExPlayer_SetAisacControlById,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "48 89 5C 24 ?? 57 48 83 EC 30 48 89 CF 0F 29 74 24 ??",
        resolve_type = set_criAtomExPlayer_SetAisacControlById,
        calling_convention = "microsoft",
    )
})]
riri_static!(criAtomExPlayer_SetAisacControlById, usize);

// ===============
// criAtomExTween_Reset
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExTween_Reset(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExTween_Reset: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomextween_reset(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static({
    XRD759_STEAM_102 => dynamic_offset(
        signature = "40 53 48 83 EC 30 48 89 CB 48 85 C9 75 ?? BA FD 32 76 FB",
        resolve_type = set_criAtomExTween_Reset,
        calling_convention = "microsoft",
    ),
    XRD759_STEAM_1013 => dynamic_offset(
        signature = "40 53 48 83 EC 30 48 89 CB 48 85 C9 75 ?? BA DA BE A4 B2",
        resolve_type = set_criAtomExTween_Reset,
        calling_convention = "microsoft",
    ),
    XRD744_STEAM_104 => dynamic_offset(
        signature = "40 53 48 83 EC 30 48 8B D9 48 85 C9 75 ?? 44 8D 41 ?? 48 8D 15 ?? ?? ?? ?? 48 83 C4 30 5B E9 ?? ?? ?? ?? E8 ?? ?? ?? ?? 0F B7 4B ??",
        resolve_type = set_criAtomExTween_Reset,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "40 53 48 83 EC 30 48 89 CB 48 85 C9 75 ?? 44 8D 41 ?? 48 8D 15 ?? ?? ?? ?? 48 83 C4 30 5B E9 ?? ?? ?? ?? E8 ?? ?? ?? ?? 8B 4B ??",
        resolve_type = set_criAtomExTween_Reset,
        calling_convention = "microsoft",
    ),
})]
riri_static!(criAtomExTween_Reset, usize);

// ===============
// criAtomExPlayer_StopWithoutReleaseTime
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_StopWithoutReleaseTime(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_StopWithoutReleaseTime: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_stopwithoutreleasetime(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static({
    XRD744_STEAM_104 => dynamic_offset(
        signature = "40 53 48 83 EC 20 48 8B D9 48 85 C9 75 ?? 44 8D 41 ?? 48 8D 15 ?? ?? ?? ?? 48 83 C4 20 5B E9 ?? ?? ?? ?? E8 ?? ?? ?? ?? 85 C0 74 ?? 83 F8 03 75 ?? 48 8B 8B ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 63 ?? 00 83 A3 ?? ?? ?? ?? 00 C6 83 ?? ?? ?? ?? 00 48 83 C4 20 5B C3 E8 ?? ?? ?? ?? BA 01 00 00 00",
        resolve_type = set_criAtomExPlayer_StopWithoutReleaseTime,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "40 53 48 83 EC 20 48 89 CB 48 85 C9 75 ?? 44 8D 41 ?? 48 8D 15 ?? ?? ?? ?? 48 83 C4 20 5B E9 ?? ?? ?? ?? E8 ?? ?? ?? ?? 85 C0 74 ?? 83 F8 03 75 ?? 48 8B 8B ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 63 ?? 00 83 A3 ?? ?? ?? ?? 00 C6 83 ?? ?? ?? ?? 00 48 83 C4 20",
        resolve_type = set_criAtomExPlayer_StopWithoutReleaseTime,
        calling_convention = "microsoft",
    )
})]
riri_static!(criAtomExPlayer_StopWithoutReleaseTime, usize);

// ===============
// criAtomExPlayer_IsPaused
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_IsPaused(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_IsPaused: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_ispaused(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static({
    XRD744_STEAM_104 => dynamic_offset(
        signature = "48 89 5C 24 ?? 57 48 83 EC 20 33 DB 48 8B F9 48 85 C9 75 ?? 44 8D 41 ?? 48 8D 15 ?? ?? ?? ?? E8 ?? ?? ?? ?? 33 C0 EB ??",
        resolve_type = set_criAtomExPlayer_IsPaused,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "48 89 5C 24 ?? 57 48 83 EC 20 31 DB 48 89 CF 48 85 C9 75 ??",
        resolve_type = set_criAtomExPlayer_IsPaused,
        calling_convention = "microsoft",
    )
})]
riri_static!(criAtomExPlayer_IsPaused, usize);

// ===============
// criAtomExPlayer_Pause
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExPlayer_Pause(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExPlayer_Pause: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomexplayer_pause(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static({
    XRD744_STEAM_104 => dynamic_offset(
        signature = "48 89 5C 24 ?? 57 48 83 EC 20 8B FA 48 8B D9 48 85 C9 75 ?? 44 8D 41 ?? 48 8D 15 ?? ?? ?? ?? E8 ?? ?? ?? ?? EB ?? E8 ?? ?? ?? ?? 85 FF",
        resolve_type = set_criAtomExPlayer_Pause,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "48 89 5C 24 ?? 48 89 74 24 ?? 57 48 83 EC 20 89 D6 48 89 CF 48 85 C9",
        resolve_type = set_criAtomExPlayer_Pause,
        calling_convention = "microsoft",
    )
})]
riri_static!(criAtomExPlayer_Pause, usize);

// ===============
// criAtomExTween_Stop
// ===============

#[no_mangle]
pub unsafe extern "C" fn set_criAtomExTween_Stop(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got criAtomExTween_Stop: 0x{:x}", addr.as_ptr() as usize);
    globals::set_criatomextween_stop(addr.as_ptr());
    Some(addr)
}

#[riri_hook_static({
    XRD744_STEAM_104 => dynamic_offset(
        signature = "40 53 48 83 EC 30 48 8B D9 48 85 C9 75 ?? 44 8D 41 ?? 48 8D 15 ?? ?? ?? ?? 48 83 C4 30 5B E9 ?? ?? ?? ?? E8 ?? ?? ?? ?? 0F B7 53 ??",
        resolve_type = set_criAtomExTween_Stop,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "40 53 48 83 EC 30 48 89 CB 48 85 C9 75 ?? 44 8D 41 ??",
        resolve_type = set_criAtomExTween_Stop,
        calling_convention = "microsoft",
    )
})]
riri_static!(criAtomExTween_Stop, usize);
