pub mod gemm; 

pub(crate) mod nn_direct;
pub(crate) mod nn_blocked;
pub(crate) mod nn_direct_microkernel; 
pub(crate) mod nn_blocked_microkernel;

pub(crate) mod nt_direct; 
pub(crate) mod nt_blocked;
pub(crate) mod nt_direct_microkernel;
pub(crate) mod nt_blocked_microkernel;

pub(crate) mod tn_blocked; 
pub(crate) mod tn_blocked_microkernel;

pub(crate) mod tt_blocked;
pub(crate) mod tt_blocked_microkernel;

