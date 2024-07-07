#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

use gjs_sys as ffi;
mod auto;
mod context;
mod profiler;

pub use auto::{Context, Coverage, Error, JSError, Profiler};

pub mod builders {
    pub use super::auto::builders::*;
}

pub mod functions {
    pub use super::auto::functions::*;
}
