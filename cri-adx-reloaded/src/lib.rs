#![allow(non_upper_case_globals)]
pub mod globals;
pub mod hooks_common;
#[cfg(feature = "use_ryo")]
pub mod hooks_ryo;
#[cfg(not(feature = "use_ryo"))]
pub mod hooks_no_ryo;
