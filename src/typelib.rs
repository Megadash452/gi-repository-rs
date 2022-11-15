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


pub struct Typelib {
    data: u8, // ?
}