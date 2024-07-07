use std::{ffi::c_void, ptr::NonNull};

use crate::{ffi, Context, Profiler};
use glib::translate::*;

type SysprofCaptureWriter = c_void;

impl Profiler {
    /// # Safety
    /// `capture` must be a valid `libsysprof::SysprofCaptureWriter`
    #[doc(alias = "gjs_profiler_set_capture_writer")]
    pub unsafe fn set_capture_writer(&mut self, capture: Option<NonNull<SysprofCaptureWriter>>) {
        unsafe {
            ffi::gjs_profiler_set_capture_writer(
                self.to_glib_none_mut().0,
                capture.map(|p| p.as_ptr()).unwrap_or(std::ptr::null_mut()),
            )
        }
    }
}

impl Context {
    #[doc(alias = "gjs_profiler_chain_signal")]
    pub fn profiler_chain_signal(&self, mut info: libc::siginfo_t) -> bool {
        let res = unsafe {
            ffi::gjs_profiler_chain_signal(
                self.to_glib_none().0,
                (&mut info as *mut libc::siginfo_t) as *mut c_void,
            )
        };
        res == glib::ffi::GTRUE
    }
}
