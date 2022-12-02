use crate::InfoType;
use crate::Typelib;
use glib::translate::{from_glib, from_glib_none, ToGlibPtr};
use libc::c_void;
use std::fmt::Debug;
use std::ptr;

// rustdoc-stripper-ignore-next
/// Iterate over all attributes associated with this node
///
/// # Gir Doc:
///
// rustdoc-stripper-ignore-next-stop
/// An opaque structure used to iterate over attributes
/// in a [`BaseInfo`][crate::BaseInfo] struct.
pub struct AttributeIter<'a>(&'a BaseInfo, ffi::GIAttributeIter);

impl<'a> AttributeIter<'a> {
    pub fn new(node: &'a BaseInfo) -> Self {
        Self(
            node,
            ffi::GIAttributeIter {
                data: 0 as *mut c_void,
                data2: 0 as *mut c_void,
                data3: 0 as *mut c_void,
                data4: 0 as *mut c_void,
            },
        )
    }
}

impl Iterator for AttributeIter<'_> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut name = ptr::null_mut();
            let mut value = ptr::null_mut();

            if ffi::g_base_info_iterate_attributes(
                self.0.to_glib_none().0,
                &mut self.1, // TODO: pass a an owned pointer (like box)
                &mut name,
                &mut value,
            ) == 0 {
                None
            } else {
                Some((from_glib_none(name), from_glib_none(value)))
            }
        }
    }
}
impl Debug for AttributeIter<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

glib::wrapper! {
    /// GIBaseInfo is the common base struct of all other Info structs
    /// accessible through the [`Repository`][crate::Repository] API.
    ///
    /// All info structures can be cast to a [`BaseInfo`][crate::BaseInfo], for instance:
    ///
    /// **⚠️ The following code is in C ⚠️**
    ///
    /// ```C
    ///    GIFunctionInfo *function_info = ...;
    ///    GIBaseInfo *info = (GIBaseInfo *) function_info;
    /// ```
    ///
    /// Most [`Repository`][crate::Repository] APIs returning a [`BaseInfo`][crate::BaseInfo] is actually
    /// creating a new struct; in other words, `g_base_info_unref()` has to
    /// be called when done accessing the data.
    ///
    /// [`BaseInfo`][crate::BaseInfo] structuress are normally accessed by calling either
    /// [`RepositoryExt::find_by_name()`][crate::prelude::RepositoryExt::find_by_name()], [`RepositoryExt::find_by_gtype()`][crate::prelude::RepositoryExt::find_by_gtype()] or
    /// [`RepositoryExt::info()`][crate::prelude::RepositoryExt::info()].
    ///
    /// **⚠️ The following code is in C ⚠️**
    ///
    /// ```C
    /// GIBaseInfo *button_info =
    ///   g_irepository_find_by_name (NULL, "Gtk", "Button");
    ///
    /// // ... use button_info ...
    ///
    /// g_base_info_unref (button_info);
    /// ```
    ///
    /// ## Hierarchy
    ///
    /// **⚠️ The following code is in plain ⚠️**
    ///
    /// ```plain
    ///   GIBaseInfo
    ///    +---- GIArgInfo
    ///    +---- GICallableInfo
    ///    +---- GIConstantInfo
    ///    +---- GIFieldInfo
    ///    +---- GIPropertyInfo
    ///    +---- GIRegisteredTypeInfo
    ///    +---- GITypeInfo
    /// ```
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct BaseInfo(Shared<ffi::GIBaseInfo>);

    match fn {
        ref => |ptr| ffi::g_base_info_ref(ptr),
        unref => |ptr| ffi::g_base_info_unref(ptr),
        type_ => || ffi::g_base_info_gtype_get_type(),
    }
}

impl BaseInfo {
    /// Retrieve an arbitrary attribute associated with this node.
    /// ## `name`
    /// a freeform string naming an attribute
    ///
    /// # Returns
    ///
    /// The value of the attribute, or [`None`] if no such attribute exists
    #[doc(alias = "g_base_info_get_attribute")]
    #[doc(alias = "get_attribute")]
    pub fn attribute(&self, name: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_base_info_get_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    /// Obtain the container of the `self`. The container is the parent
    /// GIBaseInfo. For instance, the parent of a `GIFunctionInfo` is an
    /// `GIObjectInfo` or `GIInterfaceInfo`.
    ///
    /// # Returns
    ///
    /// the container
    #[doc(alias = "g_base_info_get_container")]
    #[doc(alias = "get_container")]
    #[must_use]
    pub fn container(&self) -> Option<BaseInfo> {
        unsafe { from_glib_none(ffi::g_base_info_get_container(self.to_glib_none().0)) }
    }

    /// Obtain the name of the `self`. What the name represents depends on
    /// the [`InfoType`][crate::InfoType] of the `self`. For instance for `GIFunctionInfo` it is
    /// the name of the function.
    ///
    /// # Returns
    ///
    /// the name of `self` or [`None`] if it lacks a name.
    #[doc(alias = "g_base_info_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::g_base_info_get_name(self.to_glib_none().0)) }
    }

    /// Obtain the namespace of `self`.
    ///
    /// # Returns
    ///
    /// the namespace
    #[doc(alias = "g_base_info_get_namespace")]
    #[doc(alias = "get_namespace")]
    pub fn namespace(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::g_base_info_get_namespace(self.to_glib_none().0)) }
    }

    /// Obtain the info type of the GIBaseInfo.
    ///
    /// # Returns
    ///
    /// the info type of `self`
    #[doc(alias = "g_base_info_get_type")]
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> InfoType {
        unsafe { from_glib(ffi::g_base_info_get_type(self.to_glib_none().0)) }
    }

    /// Obtain the typelib this `self` belongs to
    ///
    /// # Returns
    ///
    /// the typelib.
    #[doc(alias = "g_base_info_get_typelib")]
    #[doc(alias = "get_typelib")]
    pub fn typelib(&self) -> Option<Typelib> {
        unsafe { from_glib_none(ffi::g_base_info_get_typelib(self.to_glib_none().0)) }
    }

    /// Obtain whether the `self` is represents a metadata which is
    /// deprecated or not.
    ///
    /// # Returns
    ///
    /// [`true`] if deprecated
    #[doc(alias = "g_base_info_is_deprecated")]
    pub fn is_deprecated(&self) -> bool {
        unsafe { from_glib(ffi::g_base_info_is_deprecated(self.to_glib_none().0)) }
    }

    /// Iterate over all attributes associated with this node. The iterator
    /// structure is typically stack allocated, and must have its first
    /// member initialized to [`None`]. Attributes are arbitrary namespaced key–value
    /// pairs which can be attached to almost any item. They are intended for use
    /// by software higher in the toolchain than bindings, and are distinct from
    /// normal GIR annotations.
    ///
    /// Both the `name` and `value` should be treated as constants
    /// and must not be freed.
    ///
    /// **⚠️ The following code is in C ⚠️**
    ///
    /// ```C
    /// void
    /// print_attributes (GIBaseInfo *info)
    /// {
    ///   GIAttributeIter iter = { 0, };
    ///   char *name;
    ///   char *value;
    ///   while (g_base_info_iterate_attributes (info, &iter, &name, &value))
    ///     {
    ///       g_print ("attribute name: %s value: %s", name, value);
    ///     }
    /// }
    /// ```
    /// ## `iterator`
    /// a [`AttributeIter`][crate::AttributeIter] structure, must be initialized; see below
    ///
    /// # Returns
    ///
    /// [`true`] if there are more attributes
    ///
    /// ## `name`
    /// Returned name, must not be freed
    ///
    /// ## `value`
    /// Returned name, must not be freed
    #[inline]
    #[doc(alias = "g_base_info_iterate_attributes")]
    pub fn iterate_attributes(&self) -> AttributeIter {
        AttributeIter::new(self)
    }
}

impl PartialEq for BaseInfo {
    #[doc(alias = "g_base_info_equal")]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::g_base_info_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }
}

impl Eq for BaseInfo {}
