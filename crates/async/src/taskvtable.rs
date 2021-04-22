#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_debug_implementations)]

use core::task::RawWaker;

pub(crate) struct TaskVTable {
    pub(crate) schedule: unsafe fn(*const ()),
    pub(crate) drop_future: unsafe fn(*const ()),
    pub(crate) get_output: unsafe fn(*const ()) -> *const (),
    pub(crate) drop_ref: unsafe fn(ptr: *const ()),
    pub(crate) destroy: unsafe fn(*const ()),
    pub(crate) run: unsafe fn(*const ()) -> bool,
    pub(crate) clone_waker: unsafe fn(ptr: *const ()) -> RawWaker,
}
