use glib::translate::*;

use crate::{ffi, Context};

impl Context {
    #[doc(alias = "gjs_context_get_native_context")]
    #[doc(alias = "get_native_context")]
    pub fn native_context(&self) -> *mut std::os::raw::c_void {
        unsafe { ffi::gjs_context_get_native_context(self.to_glib_none().0) }
    }
}
