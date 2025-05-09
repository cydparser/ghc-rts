use super::*;
use crate::stg::types::StgWord64;
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck_macros::quickcheck;
use std::ptr::null_mut;

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_CHAR_MIN() {
    assert_eq!(sys::HS_CHAR_MIN, HS_CHAR_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_CHAR_MAX() {
    assert_eq!(sys::HS_CHAR_MAX, HS_CHAR_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_BOOL_FALSE() {
    assert_eq!(sys::HS_BOOL_FALSE, HS_BOOL_FALSE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_BOOL_TRUE() {
    assert_eq!(sys::HS_BOOL_TRUE, HS_BOOL_TRUE);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_BOOL_MIN() {
    assert_eq!(sys::HS_BOOL_MIN, HS_BOOL_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_BOOL_MAX() {
    assert_eq!(sys::HS_BOOL_MAX, HS_BOOL_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT8_MIN() {
    assert_eq!(sys::HS_INT8_MIN, HS_INT8_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT8_MAX() {
    assert_eq!(sys::HS_INT8_MAX, HS_INT8_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT16_MIN() {
    assert_eq!(sys::HS_INT16_MIN, HS_INT16_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT16_MAX() {
    assert_eq!(sys::HS_INT16_MAX, HS_INT16_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT32_MIN() {
    assert_eq!(sys::HS_INT32_MIN, HS_INT32_MIN);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_INT32_MAX() {
    assert_eq!(sys::HS_INT32_MAX, HS_INT32_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_WORD8_MAX() {
    assert_eq!(sys::HS_WORD8_MAX, HS_WORD8_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_WORD16_MAX() {
    assert_eq!(sys::HS_WORD16_MAX, HS_WORD16_MAX);
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_HS_WORD32_MAX() {
    assert_eq!(sys::HS_WORD32_MAX, HS_WORD32_MAX);
}

#[test]
#[ignore]
fn test_hs_init() {
    // let mut argc = null_mut();
    // let mut argv = null_mut();
    // unsafe { hs_init(&mut argc, &mut &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_exit() {
    unsafe { hs_exit() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_exit_nowait() {
    unsafe { hs_exit_nowait() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_set_argv() {
    // let argc = Default::default();
    // let mut argv = null_mut();
    // unsafe { hs_set_argv(argc, &mut &mut argv) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_thread_done() {
    unsafe { hs_thread_done() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_perform_gc() {
    unsafe { hs_perform_gc() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_lock_stable_ptr_table() {
    unsafe { hs_lock_stable_ptr_table() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_lock_stable_tables() {
    unsafe { hs_lock_stable_tables() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_unlock_stable_ptr_table() {
    unsafe { hs_unlock_stable_ptr_table() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_unlock_stable_tables() {
    unsafe { hs_unlock_stable_tables() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_free_stable_ptr_unsafe() {
    let sp = null_mut();
    unsafe { hs_free_stable_ptr_unsafe(sp) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_free_stable_ptr() {
    let sp = null_mut();
    unsafe { hs_free_stable_ptr(sp) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_free_fun_ptr() {
    let fp = Default::default();
    unsafe { hs_free_fun_ptr(fp) };
    todo!("assert")
}

#[cfg(feature = "sys")]
#[quickcheck]
#[ignore]
fn equivalent_hs_spt_lookup(key: StgWord64) -> bool {
    let mut key = key;
    let expected = unsafe { sys::hs_spt_lookup(&mut key) };
    let actual = unsafe { hs_spt_lookup(&mut key) };
    actual == expected
}

#[test]
#[ignore]
fn test_hs_spt_lookup() {
    let mut key = 0;
    unsafe { hs_spt_lookup(&raw mut key) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_spt_keys() {
    let mut keys = null_mut();
    let szKeys = Default::default();
    unsafe { hs_spt_keys(&mut keys, szKeys) };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_spt_key_count() {
    unsafe { hs_spt_key_count() };
    todo!("assert")
}

#[test]
#[ignore]
fn test_hs_try_putmvar() {
    let capability = Default::default();
    let sp = null_mut();
    unsafe { hs_try_putmvar(capability, sp) };
    todo!("assert")
}
