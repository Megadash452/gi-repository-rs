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

use std::ptr::{self, NonNull};
use glib::translate::{ToGlibPtr, ToGlibPtrMut, FromGlibPtrNone, FromGlibPtrFull, StashMut, from_glib_none, from_glib_full};
use libc::c_void;


/// TODO
#[doc(alias = "GITypelib")]
#[derive(Debug)]
pub struct Typelib(pub NonNull<ffi::GITypelib>);

impl Typelib {
    pub fn new_from_memory(memory: &[u8]) -> Result<Self, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let rtrn = ffi::g_typelib_new_from_const_memory(memory.as_ptr(), memory.len(), &mut error);
            if error.is_null() {
                Ok(from_glib_none(rtrn))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
    pub fn new_from_mut_memory(memory: &mut [u8]) -> Result<Self, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let rtrn = ffi::g_typelib_new_from_memory(memory.as_mut_ptr(), memory.len(), &mut error);
            if error.is_null() {
                Ok(from_glib_none(rtrn))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
    pub fn new_from_mapped_file(mfile: *mut glib_sys::GMappedFile) -> Result<Self, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let rtrn = ffi::g_typelib_new_from_mapped_file(mfile, &mut error);
            if error.is_null() {
                Ok(from_glib_none(rtrn))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn namespace(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_typelib_get_namespace(self.0.as_ptr()))
        }
    }
    pub unsafe fn load_symbol(&self, symbol_name: &str) -> Option<*mut c_void> { // TODO: what type should the symbol pointer be?
        let mut rtrn = std::ptr::null_mut();
        if ffi::g_typelib_symbol(
            self.0.as_ptr(),
            symbol_name.to_glib_none().0,
            &mut rtrn
        ) == 0 {
            None
        } else {
            Some(rtrn);
            todo!("dlopen symbol")
        }
    }
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
        Self(match NonNull::new(ptr) {
            Some(ptr) => {
                assert!(!ptr.as_ref().is_null());
                ptr
            },
            None => panic!("dereferenced null")
        })
    }
}
impl FromGlibPtrFull<*mut ffi::GITypelib> for Typelib {
    unsafe fn from_glib_full(ptr: *mut ffi::GITypelib) -> Self {
        Self(match NonNull::new(ptr) {
            Some(ptr) => {
                assert!(!ptr.as_ref().is_null());
                ptr
            },
            None => panic!("dereferenced null")
        })
    }
}
impl<'a> ToGlibPtrMut<'a, *mut ffi::GITypelib> for Typelib {
    type Storage = &'a mut Self;
    
    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut ffi::GITypelib, Self> {
        StashMut(self.0.as_ptr(), self)
    }
}
