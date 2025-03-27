use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use quickcheck::quickcheck;
use std::mem::{size_of, transmute};
#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_SIG_DFL() {
    assert_eq!(sys::STG_SIG_DFL, super::STG_SIG_DFL.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_SIG_IGN() {
    assert_eq!(sys::STG_SIG_IGN, super::STG_SIG_IGN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_SIG_ERR() {
    assert_eq!(sys::STG_SIG_ERR, super::STG_SIG_ERR.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_SIG_HAN() {
    assert_eq!(sys::STG_SIG_HAN, super::STG_SIG_HAN.into());
}

#[cfg(feature = "sys")]
#[test]
fn test_eq_STG_SIG_RST() {
    assert_eq!(sys::STG_SIG_RST, super::STG_SIG_RST.into());
}
