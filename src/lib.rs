//! Definitions of register addresses and bits within those registers

#![feature(llvm_asm)]
#![feature(const_fn)]
#![feature(associated_type_defaults)]
#![feature(lang_items)]
#![feature(unwind_attributes)]
#![feature(proc_macro_hygiene)]

#![no_std]

#[cfg(feature = "avr-std-stub")] extern crate avr_std_stub;

pub use self::register::{Register, RegisterBits, RegisterValue};
pub use self::pin::{DataDirection, Pin};

pub mod prelude;
pub mod legacy;
/// Low level register-based API for device-specific operations.
pub mod cores;
pub mod interrupt;
pub mod modules;

/// Configuration for the currently-targeted microcontroller.
pub mod config;

mod register;
mod pin;

