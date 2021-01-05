#![no_std]

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(only_hack_old_rustc)]
pub use time_macros_impl::{date, datetime, offset, time};
