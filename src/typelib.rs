// In generated gi-repository-sys:
// #[repr(C)]
// pub struct _GITypelib {
//     _data: [u8; 0],
//     _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
// }
// pub type GITypelib = *mut _GITypelib;

// In gobject-introspection/gi-repository C library:
// struct _GITypelib {
//     /*< private >*/
//     guchar *data;
//     gsize len;
//     gboolean owns_memory;
//     GMappedFile *mfile;
//     GList *modules;
//     gboolean open_attempted;
//   };
// typedef struct _GITypelib GITypelib;

use std::cell::RefCell;
use glib::translate::{ToGlibPtrMut, FromGlibPtrNone, FromGlibPtrFull, StashMut, from_glib_none};
use libc::c_void;


#[doc(alias = "GITypelib")]
pub struct Typelib(RefCell<ffi::GITypelib>);
// TODO: why is the macro giving me these errors?
// glib::wrapper! {
//     #[doc(alias = "GIRepository")]
//     pub struct Typelib(Boxed<ffi::GITypelib>);

//     match fn {
//         free => || ffi::g_typelib_free(),
//     }
// }

impl Typelib {
    pub fn namespace(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_typelib_get_namespace(self.0.as_ptr()))
        }
    }
    pub fn load_symbol(&self, symbol_name: &str) -> Option<*mut c_void> { // TODO: what type should the symbol pointer be?
        unsafe {
            let mut rtrn = std::ptr::null_mut();
            if ffi::g_typelib_symbol(
                self.0.as_ptr(),
                symbol_name.as_bytes().as_ptr() as *const i8,
                &mut rtrn
            ) == 0 {
                None
            } else {
                Some(rtrn)
            }
        }
    }
    // pub fn g_typelib_new_from_const_memory(memory: *const u8, len: size_t, error: *mut *mut glib::GError) -> *mut GITypelib;
    // pub fn g_typelib_new_from_mapped_file(mfile: *mut glib::GMappedFile, error: *mut *mut glib::GError) -> *mut GITypelib;
    // pub fn g_typelib_new_from_memory(memory: *mut u8, len: size_t, error: *mut *mut glib::GError) -> *mut GITypelib;
}

impl Drop for Typelib {
    fn drop(&mut self) {
        unsafe {
            ffi::g_typelib_free(self.0.as_ptr())
        }
    }
}
impl FromGlibPtrNone<*mut ffi::GITypelib> for Typelib {
    unsafe fn from_glib_none(ptr: *mut ffi::GITypelib) -> Self {
        assert!(!ptr.is_null() && !(*ptr).is_null());
        Self(RefCell::new(*ptr))
    }
}
impl FromGlibPtrFull<*mut ffi::GITypelib> for Typelib {
    unsafe fn from_glib_full(ptr: *mut ffi::GITypelib) -> Self {
        assert!(!ptr.is_null() && !(*ptr).is_null());
        Self(RefCell::new(*ptr))
    }
}
impl<'a> ToGlibPtrMut<'a, *mut ffi::GITypelib> for Typelib {
    type Storage = &'a mut Self;
    
    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut ffi::GITypelib, Self> {
        StashMut(self.0.as_ptr(), self)
    }
}