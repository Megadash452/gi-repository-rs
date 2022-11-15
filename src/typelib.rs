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

use glib::translate::{ToGlibPtrMut, FromGlibPtrNone, FromGlibPtrFull, StashMut};


#[doc(alias = "GITypelib")]
pub struct Typelib(ffi::GITypelib);
// TODO: why is the macro giving me these errors?
// glib::wrapper! {
//     #[doc(alias = "GIRepository")]
//     pub struct Typelib(Boxed<ffi::GITypelib>);

//     match fn {
//         free => || ffi::g_typelib_free(),
//     }
// }

impl From<ffi::GITypelib> for Typelib {
    fn from(ptr: ffi::GITypelib) -> Self {
       Self(ptr)
    }
}

impl Drop for Typelib {
    fn drop(&mut self) {
        unsafe {
            ffi::g_typelib_free(&mut self.0)
        }
    }
}
impl FromGlibPtrNone<*mut ffi::GITypelib> for Typelib {
    unsafe fn from_glib_none(ptr: *mut ffi::GITypelib) -> Self {
        assert!(!ptr.is_null() && !(*ptr).is_null());
        Self(*ptr)
    }
}
impl FromGlibPtrFull<*mut ffi::GITypelib> for Typelib {
    unsafe fn from_glib_full(ptr: *mut ffi::GITypelib) -> Self {
        assert!(!ptr.is_null() && !(*ptr).is_null());
        Self(*ptr)
    }
}
impl<'a> ToGlibPtrMut<'a, *mut ffi::GITypelib> for Typelib {
    type Storage = &'a mut Self;
    
    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut ffi::GITypelib, Self> {
        StashMut(&mut self.0, self)
    }
}