//! Peripheral access API for RP2350 microcontrollers
//!
//! This top-level `lib.rs` is just a compile-time switch between two blocks of
//! auto-generated code - one for RISC-V and one for Cortex-M

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::empty_docs)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::wrong_self_convention)]
#![no_std]

// Use the Cortex-M version by default

#[cfg(target_arch = "arm")]
#[doc(hidden)]
#[path = "inner/mod_cortex_m.rs"]
mod inner;

// On riscv32*-*-* use the RISC-V version

#[cfg(not(target_arch = "arm"))]
#[doc(hidden)]
#[path = "inner/mod_risc_v.rs"]
mod inner;

pub(crate) use inner::generic::*;
pub use inner::*;
