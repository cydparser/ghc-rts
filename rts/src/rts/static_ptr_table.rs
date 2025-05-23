use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(test)]
use crate::utils::test::{Arbitrary, Gen, HasReferences};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::slice;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_spt_insert"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_insert(key: *mut StgWord64, spe_closure: *mut c_void) {
    unsafe { sys::hs_spt_insert(key, spe_closure) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_spt_insert_stableptr"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_insert_stableptr(key: *mut StgWord64, entry: *mut StgStablePtr) {
    unsafe { sys::hs_spt_insert_stableptr(key, entry) }
}

#[cfg_attr(feature = "sys", unsafe(export_name = "rust_hs_spt_remove"))]
#[cfg_attr(not(feature = "sys"), unsafe(no_mangle))]
#[cfg_attr(feature = "tracing", instrument)]
pub unsafe extern "C" fn hs_spt_remove(key: *mut StgWord64) {
    unsafe { sys::hs_spt_remove(key) }
}
