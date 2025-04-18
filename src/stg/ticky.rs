use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
#[cfg(feature = "sys")]
use ghc_rts_sys as sys;
use libc::{clockid_t, pid_t, pthread_cond_t, pthread_key_t, pthread_mutex_t, pthread_t};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use std::mem::transmute;
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(test)]
mod tests;

pub const TICKY_BIN_COUNT: u32 = 9;

static mut ENT_VIA_NODE_ctr: StgInt = unsafe { sys::ENT_VIA_NODE_ctr };

#[unsafe(no_mangle)]
pub static mut ENT_STATIC_THK_SINGLE_ctr: StgInt = unsafe { sys::ENT_STATIC_THK_SINGLE_ctr };

#[unsafe(no_mangle)]
pub static mut ENT_DYN_THK_SINGLE_ctr: StgInt = unsafe { sys::ENT_DYN_THK_SINGLE_ctr };

#[unsafe(no_mangle)]
pub static mut ENT_STATIC_THK_MANY_ctr: StgInt = unsafe { sys::ENT_STATIC_THK_MANY_ctr };

#[unsafe(no_mangle)]
pub static mut ENT_DYN_THK_MANY_ctr: StgInt = unsafe { sys::ENT_DYN_THK_MANY_ctr };

#[unsafe(no_mangle)]
pub static mut ENT_STATIC_FUN_DIRECT_ctr: StgInt = unsafe { sys::ENT_STATIC_FUN_DIRECT_ctr };

static mut ENT_DYN_FUN_DIRECT_ctr: StgInt = unsafe { sys::ENT_DYN_FUN_DIRECT_ctr };

static mut ENT_STATIC_CON_ctr: StgInt = unsafe { sys::ENT_STATIC_CON_ctr };

#[unsafe(no_mangle)]
pub static mut ENT_DYN_CON_ctr: StgInt = unsafe { sys::ENT_DYN_CON_ctr };

static mut ENT_STATIC_IND_ctr: StgInt = unsafe { sys::ENT_STATIC_IND_ctr };

static mut ENT_DYN_IND_ctr: StgInt = unsafe { sys::ENT_DYN_IND_ctr };

static mut ENT_PERM_IND_ctr: StgInt = unsafe { sys::ENT_PERM_IND_ctr };

static mut ENT_PAP_ctr: StgInt = unsafe { sys::ENT_PAP_ctr };

static mut ENT_CONTINUATION_ctr: StgInt = unsafe { sys::ENT_CONTINUATION_ctr };

static mut ENT_AP_ctr: StgInt = unsafe { sys::ENT_AP_ctr };

static mut ENT_AP_STACK_ctr: StgInt = unsafe { sys::ENT_AP_STACK_ctr };

static mut ENT_BH_ctr: StgInt = unsafe { sys::ENT_BH_ctr };

#[unsafe(no_mangle)]
pub static mut ENT_LNE_ctr: StgInt = unsafe { sys::ENT_LNE_ctr };

#[unsafe(no_mangle)]
pub static mut UNKNOWN_CALL_ctr: StgInt = unsafe { sys::UNKNOWN_CALL_ctr };

static mut SLOW_CALL_fast_v16_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_v16_ctr };

static mut SLOW_CALL_fast_v_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_v_ctr };

static mut SLOW_CALL_fast_f_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_f_ctr };

static mut SLOW_CALL_fast_d_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_d_ctr };

static mut SLOW_CALL_fast_l_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_l_ctr };

static mut SLOW_CALL_fast_n_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_n_ctr };

static mut SLOW_CALL_fast_p_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_p_ctr };

static mut SLOW_CALL_fast_pv_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_pv_ctr };

static mut SLOW_CALL_fast_pp_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_pp_ctr };

static mut SLOW_CALL_fast_ppv_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_ppv_ctr };

static mut SLOW_CALL_fast_ppp_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_ppp_ctr };

static mut SLOW_CALL_fast_pppv_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_pppv_ctr };

static mut SLOW_CALL_fast_pppp_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_pppp_ctr };

static mut SLOW_CALL_fast_ppppp_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_ppppp_ctr };

static mut SLOW_CALL_fast_pppppp_ctr: StgInt = unsafe { sys::SLOW_CALL_fast_pppppp_ctr };

#[unsafe(no_mangle)]
pub static mut VERY_SLOW_CALL_ctr: StgInt = unsafe { sys::VERY_SLOW_CALL_ctr };

static mut ticky_slow_call_unevald: StgInt = unsafe { sys::ticky_slow_call_unevald };

static mut SLOW_CALL_ctr: StgInt = unsafe { sys::SLOW_CALL_ctr };

static mut MULTI_CHUNK_SLOW_CALL_ctr: StgInt = unsafe { sys::MULTI_CHUNK_SLOW_CALL_ctr };

static mut MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr: StgInt =
    unsafe { sys::MULTI_CHUNK_SLOW_CALL_CHUNKS_ctr };

#[unsafe(no_mangle)]
pub static mut KNOWN_CALL_ctr: StgInt = unsafe { sys::KNOWN_CALL_ctr };

#[unsafe(no_mangle)]
pub static mut KNOWN_CALL_TOO_FEW_ARGS_ctr: StgInt = unsafe { sys::KNOWN_CALL_TOO_FEW_ARGS_ctr };

#[unsafe(no_mangle)]
pub static mut KNOWN_CALL_EXTRA_ARGS_ctr: StgInt = unsafe { sys::KNOWN_CALL_EXTRA_ARGS_ctr };

static mut SLOW_CALL_FUN_TOO_FEW_ctr: StgInt = unsafe { sys::SLOW_CALL_FUN_TOO_FEW_ctr };

static mut SLOW_CALL_FUN_CORRECT_ctr: StgInt = unsafe { sys::SLOW_CALL_FUN_CORRECT_ctr };

static mut SLOW_CALL_FUN_TOO_MANY_ctr: StgInt = unsafe { sys::SLOW_CALL_FUN_TOO_MANY_ctr };

static mut SLOW_CALL_PAP_TOO_FEW_ctr: StgInt = unsafe { sys::SLOW_CALL_PAP_TOO_FEW_ctr };

static mut SLOW_CALL_PAP_CORRECT_ctr: StgInt = unsafe { sys::SLOW_CALL_PAP_CORRECT_ctr };

static mut SLOW_CALL_PAP_TOO_MANY_ctr: StgInt = unsafe { sys::SLOW_CALL_PAP_TOO_MANY_ctr };

static mut SLOW_CALL_UNEVALD_ctr: StgInt = unsafe { sys::SLOW_CALL_UNEVALD_ctr };

#[unsafe(no_mangle)]
pub static mut UPDF_OMITTED_ctr: StgInt = unsafe { sys::UPDF_OMITTED_ctr };

#[unsafe(no_mangle)]
pub static mut UPDF_PUSHED_ctr: StgInt = unsafe { sys::UPDF_PUSHED_ctr };

static mut CATCHF_PUSHED_ctr: StgInt = unsafe { sys::CATCHF_PUSHED_ctr };

static mut UPDF_RCC_PUSHED_ctr: StgInt = unsafe { sys::UPDF_RCC_PUSHED_ctr };

static mut UPDF_RCC_OMITTED_ctr: StgInt = unsafe { sys::UPDF_RCC_OMITTED_ctr };

static mut UPD_SQUEEZED_ctr: StgInt = unsafe { sys::UPD_SQUEEZED_ctr };

static mut UPD_CON_IN_NEW_ctr: StgInt = unsafe { sys::UPD_CON_IN_NEW_ctr };

static mut UPD_CON_IN_PLACE_ctr: StgInt = unsafe { sys::UPD_CON_IN_PLACE_ctr };

static mut UPD_PAP_IN_NEW_ctr: StgInt = unsafe { sys::UPD_PAP_IN_NEW_ctr };

static mut UPD_PAP_IN_PLACE_ctr: StgInt = unsafe { sys::UPD_PAP_IN_PLACE_ctr };

#[unsafe(no_mangle)]
pub static mut ALLOC_HEAP_ctr: StgInt = unsafe { sys::ALLOC_HEAP_ctr };

#[unsafe(no_mangle)]
pub static mut ALLOC_HEAP_tot: StgInt = unsafe { sys::ALLOC_HEAP_tot };

#[unsafe(no_mangle)]
pub static mut HEAP_CHK_ctr: StgInt = unsafe { sys::HEAP_CHK_ctr };

#[unsafe(no_mangle)]
pub static mut STK_CHK_ctr: StgInt = unsafe { sys::STK_CHK_ctr };

#[unsafe(no_mangle)]
pub static mut ALLOC_RTS_ctr: StgInt = unsafe { sys::ALLOC_RTS_ctr };

#[unsafe(no_mangle)]
pub static mut ALLOC_RTS_tot: StgInt = unsafe { sys::ALLOC_RTS_tot };

#[unsafe(no_mangle)]
pub static mut ALLOC_FUN_ctr: StgInt = unsafe { sys::ALLOC_FUN_ctr };

static mut ALLOC_FUN_adm: StgInt = unsafe { sys::ALLOC_FUN_adm };

#[unsafe(no_mangle)]
pub static mut ALLOC_FUN_gds: StgInt = unsafe { sys::ALLOC_FUN_gds };

static mut ALLOC_FUN_slp: StgInt = unsafe { sys::ALLOC_FUN_slp };

static mut UPD_NEW_IND_ctr: StgInt = unsafe { sys::UPD_NEW_IND_ctr };

static mut UPD_NEW_PERM_IND_ctr: StgInt = unsafe { sys::UPD_NEW_PERM_IND_ctr };

static mut UPD_OLD_IND_ctr: StgInt = unsafe { sys::UPD_OLD_IND_ctr };

static mut UPD_OLD_PERM_IND_ctr: StgInt = unsafe { sys::UPD_OLD_PERM_IND_ctr };

static mut UPD_BH_UPDATABLE_ctr: StgInt = unsafe { sys::UPD_BH_UPDATABLE_ctr };

#[unsafe(no_mangle)]
pub static mut UPD_CAF_BH_UPDATABLE_ctr: StgInt = unsafe { sys::UPD_CAF_BH_UPDATABLE_ctr };

#[unsafe(no_mangle)]
pub static mut UPD_CAF_BH_SINGLE_ENTRY_ctr: StgInt = unsafe { sys::UPD_CAF_BH_SINGLE_ENTRY_ctr };

static mut GC_SEL_ABANDONED_ctr: StgInt = unsafe { sys::GC_SEL_ABANDONED_ctr };

static mut GC_SEL_MINOR_ctr: StgInt = unsafe { sys::GC_SEL_MINOR_ctr };

static mut GC_SEL_MAJOR_ctr: StgInt = unsafe { sys::GC_SEL_MAJOR_ctr };

static mut GC_FAILED_PROMOTION_ctr: StgInt = unsafe { sys::GC_FAILED_PROMOTION_ctr };

#[unsafe(no_mangle)]
pub static mut ALLOC_UP_THK_ctr: StgInt = unsafe { sys::ALLOC_UP_THK_ctr };

#[unsafe(no_mangle)]
pub static mut ALLOC_SE_THK_ctr: StgInt = unsafe { sys::ALLOC_SE_THK_ctr };

static mut ALLOC_THK_adm: StgInt = unsafe { sys::ALLOC_THK_adm };

#[unsafe(no_mangle)]
pub static mut ALLOC_THK_gds: StgInt = unsafe { sys::ALLOC_THK_gds };

#[unsafe(no_mangle)]
pub static mut ALLOC_THK_slp: StgInt = unsafe { sys::ALLOC_THK_slp };

#[unsafe(no_mangle)]
pub static mut ALLOC_CON_ctr: StgInt = unsafe { sys::ALLOC_CON_ctr };

static mut ALLOC_CON_adm: StgInt = unsafe { sys::ALLOC_CON_adm };

#[unsafe(no_mangle)]
pub static mut ALLOC_CON_gds: StgInt = unsafe { sys::ALLOC_CON_gds };

static mut ALLOC_CON_slp: StgInt = unsafe { sys::ALLOC_CON_slp };

static mut ALLOC_TUP_ctr: StgInt = unsafe { sys::ALLOC_TUP_ctr };

static mut ALLOC_TUP_adm: StgInt = unsafe { sys::ALLOC_TUP_adm };

static mut ALLOC_TUP_gds: StgInt = unsafe { sys::ALLOC_TUP_gds };

static mut ALLOC_TUP_slp: StgInt = unsafe { sys::ALLOC_TUP_slp };

static mut ALLOC_BH_ctr: StgInt = unsafe { sys::ALLOC_BH_ctr };

static mut ALLOC_BH_adm: StgInt = unsafe { sys::ALLOC_BH_adm };

static mut ALLOC_BH_gds: StgInt = unsafe { sys::ALLOC_BH_gds };

static mut ALLOC_BH_slp: StgInt = unsafe { sys::ALLOC_BH_slp };

#[unsafe(no_mangle)]
pub static mut ALLOC_PRIM_ctr: StgInt = unsafe { sys::ALLOC_PRIM_ctr };

#[unsafe(no_mangle)]
pub static mut ALLOC_PRIM_adm: StgInt = unsafe { sys::ALLOC_PRIM_adm };

#[unsafe(no_mangle)]
pub static mut ALLOC_PRIM_gds: StgInt = unsafe { sys::ALLOC_PRIM_gds };

#[unsafe(no_mangle)]
pub static mut ALLOC_PRIM_slp: StgInt = unsafe { sys::ALLOC_PRIM_slp };

#[unsafe(no_mangle)]
pub static mut ALLOC_PAP_ctr: StgInt = unsafe { sys::ALLOC_PAP_ctr };

static mut ALLOC_PAP_adm: StgInt = unsafe { sys::ALLOC_PAP_adm };

#[unsafe(no_mangle)]
pub static mut ALLOC_PAP_gds: StgInt = unsafe { sys::ALLOC_PAP_gds };

#[unsafe(no_mangle)]
pub static mut ALLOC_PAP_slp: StgInt = unsafe { sys::ALLOC_PAP_slp };

static mut ALLOC_TSO_ctr: StgInt = unsafe { sys::ALLOC_TSO_ctr };

static mut ALLOC_TSO_tot: StgInt = unsafe { sys::ALLOC_TSO_tot };

static mut ALLOC_STACK_ctr: StgInt = unsafe { sys::ALLOC_STACK_ctr };

static mut ALLOC_STACK_tot: StgInt = unsafe { sys::ALLOC_STACK_tot };

#[unsafe(no_mangle)]
pub static mut RET_NEW_ctr: StgInt = unsafe { sys::RET_NEW_ctr };

#[unsafe(no_mangle)]
pub static mut RET_OLD_ctr: StgInt = unsafe { sys::RET_OLD_ctr };

#[unsafe(no_mangle)]
pub static mut RET_UNBOXED_TUP_ctr: StgInt = unsafe { sys::RET_UNBOXED_TUP_ctr };

static mut RET_SEMI_loads_avoided: StgInt = unsafe { sys::RET_SEMI_loads_avoided };

#[unsafe(no_mangle)]
pub static mut TAG_UNTAGGED_pred: StgInt = unsafe { sys::TAG_UNTAGGED_pred };

#[unsafe(no_mangle)]
pub static mut TAG_UNTAGGED_miss: StgInt = unsafe { sys::TAG_UNTAGGED_miss };

#[unsafe(no_mangle)]
pub static mut TAG_TAGGED_pred: StgInt = unsafe { sys::TAG_TAGGED_pred };

static mut TAG_TAGGED_miss: StgInt = unsafe { sys::TAG_TAGGED_miss };

#[unsafe(no_mangle)]
pub static mut RET_NEW_hst: [StgInt; 9usize] = unsafe { sys::RET_NEW_hst };

#[unsafe(no_mangle)]
pub static mut RET_OLD_hst: [StgInt; 9usize] = unsafe { sys::RET_OLD_hst };

#[unsafe(no_mangle)]
pub static mut RET_UNBOXED_TUP_hst: [StgInt; 9usize] = unsafe { sys::RET_UNBOXED_TUP_hst };
