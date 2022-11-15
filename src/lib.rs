macro_rules! assert_initialized_main_thread {
    () => { };
}

mod auto;
pub use auto::*;

mod typelib;
pub use typelib::*;