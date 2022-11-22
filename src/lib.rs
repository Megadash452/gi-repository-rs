#![cfg_attr(feature = "dox", feature(doc_cfg))]

macro_rules! assert_initialized_main_thread {
    () => { };
}

macro_rules! skip_assert_initialized {
    () => { };
}

mod auto;
pub use auto::*;


mod typelib;
pub use typelib::*;

mod base_info;
pub use base_info::*;
