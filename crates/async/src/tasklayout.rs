#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_debug_implementations)]

use std::alloc::Layout;

#[derive(Clone, Copy)]
pub(crate) struct TaskLayout {
    pub(crate) layout: Layout,
    pub(crate) offset_s: usize,
    pub(crate) offset_f: usize,
    pub(crate) offset_r: usize,
}
