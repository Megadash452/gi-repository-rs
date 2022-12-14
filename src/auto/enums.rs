// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

/// The type of array in a `GITypeInfo`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GIArrayType")]
pub enum ArrayType {
    /// a C array, char[] for instance
    #[doc(alias = "GI_ARRAY_TYPE_C")]
    C,
    /// a `GArray` array
    #[doc(alias = "GI_ARRAY_TYPE_ARRAY")]
    Array,
    /// a `GPtrArray` array
    #[doc(alias = "GI_ARRAY_TYPE_PTR_ARRAY")]
    PtrArray,
    /// a `GByteArray` array
    #[doc(alias = "GI_ARRAY_TYPE_BYTE_ARRAY")]
    ByteArray,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ArrayType::{}",
            match *self {
                Self::C => "C",
                Self::Array => "Array",
                Self::PtrArray => "PtrArray",
                Self::ByteArray => "ByteArray",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ArrayType {
    type GlibType = ffi::GIArrayType;

    fn into_glib(self) -> ffi::GIArrayType {
        match self {
            Self::C => ffi::GI_ARRAY_TYPE_C,
            Self::Array => ffi::GI_ARRAY_TYPE_ARRAY,
            Self::PtrArray => ffi::GI_ARRAY_TYPE_PTR_ARRAY,
            Self::ByteArray => ffi::GI_ARRAY_TYPE_BYTE_ARRAY,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GIArrayType> for ArrayType {
    unsafe fn from_glib(value: ffi::GIArrayType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GI_ARRAY_TYPE_C => Self::C,
            ffi::GI_ARRAY_TYPE_ARRAY => Self::Array,
            ffi::GI_ARRAY_TYPE_PTR_ARRAY => Self::PtrArray,
            ffi::GI_ARRAY_TYPE_BYTE_ARRAY => Self::ByteArray,
            value => Self::__Unknown(value),
        }
    }
}

/// The direction of a `GIArgInfo`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GIDirection")]
pub enum Direction {
    /// in argument.
    #[doc(alias = "GI_DIRECTION_IN")]
    In,
    /// out argument.
    #[doc(alias = "GI_DIRECTION_OUT")]
    Out,
    /// in and out argument.
    #[doc(alias = "GI_DIRECTION_INOUT")]
    Inout,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Direction::{}",
            match *self {
                Self::In => "In",
                Self::Out => "Out",
                Self::Inout => "Inout",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Direction {
    type GlibType = ffi::GIDirection;

    fn into_glib(self) -> ffi::GIDirection {
        match self {
            Self::In => ffi::GI_DIRECTION_IN,
            Self::Out => ffi::GI_DIRECTION_OUT,
            Self::Inout => ffi::GI_DIRECTION_INOUT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GIDirection> for Direction {
    unsafe fn from_glib(value: ffi::GIDirection) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GI_DIRECTION_IN => Self::In,
            ffi::GI_DIRECTION_OUT => Self::Out,
            ffi::GI_DIRECTION_INOUT => Self::Inout,
            value => Self::__Unknown(value),
        }
    }
}

/// The type of a GIBaseInfo struct.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GIInfoType")]
pub enum InfoType {
    /// invalid type
    #[doc(alias = "GI_INFO_TYPE_INVALID")]
    Invalid,
    /// function, see `GIFunctionInfo`
    #[doc(alias = "GI_INFO_TYPE_FUNCTION")]
    Function,
    /// callback, see `GIFunctionInfo`
    #[doc(alias = "GI_INFO_TYPE_CALLBACK")]
    Callback,
    /// struct, see `GIStructInfo`
    #[doc(alias = "GI_INFO_TYPE_STRUCT")]
    Struct,
    /// boxed, see `GIStructInfo` or `GIUnionInfo`
    #[doc(alias = "GI_INFO_TYPE_BOXED")]
    Boxed,
    /// enum, see `GIEnumInfo`
    #[doc(alias = "GI_INFO_TYPE_ENUM")]
    Enum,
    /// flags, see `GIEnumInfo`
    #[doc(alias = "GI_INFO_TYPE_FLAGS")]
    Flags,
    /// object, see `GIObjectInfo`
    #[doc(alias = "GI_INFO_TYPE_OBJECT")]
    Object,
    /// interface, see `GIInterfaceInfo`
    #[doc(alias = "GI_INFO_TYPE_INTERFACE")]
    Interface,
    /// contant, see `GIConstantInfo`
    #[doc(alias = "GI_INFO_TYPE_CONSTANT")]
    Constant,
    /// deleted, used to be GI_INFO_TYPE_ERROR_DOMAIN.
    #[doc(alias = "GI_INFO_TYPE_INVALID_0")]
    Invalid0,
    /// union, see `GIUnionInfo`
    #[doc(alias = "GI_INFO_TYPE_UNION")]
    Union,
    /// enum value, see `GIValueInfo`
    #[doc(alias = "GI_INFO_TYPE_VALUE")]
    Value,
    /// signal, see `GISignalInfo`
    #[doc(alias = "GI_INFO_TYPE_SIGNAL")]
    Signal,
    /// virtual function, see `GIVFuncInfo`
    #[doc(alias = "GI_INFO_TYPE_VFUNC")]
    Vfunc,
    /// GObject property, see `GIPropertyInfo`
    #[doc(alias = "GI_INFO_TYPE_PROPERTY")]
    Property,
    /// struct or union field, see `GIFieldInfo`
    #[doc(alias = "GI_INFO_TYPE_FIELD")]
    Field,
    /// argument of a function or callback, see `GIArgInfo`
    #[doc(alias = "GI_INFO_TYPE_ARG")]
    Arg,
    /// type information, see `GITypeInfo`
    #[doc(alias = "GI_INFO_TYPE_TYPE")]
    Type,
    /// unresolved type, a type which is not present in
    ///  the typelib, or any of its dependencies.
    #[doc(alias = "GI_INFO_TYPE_UNRESOLVED")]
    Unresolved,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for InfoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InfoType::{}",
            match *self {
                Self::Invalid => "Invalid",
                Self::Function => "Function",
                Self::Callback => "Callback",
                Self::Struct => "Struct",
                Self::Boxed => "Boxed",
                Self::Enum => "Enum",
                Self::Flags => "Flags",
                Self::Object => "Object",
                Self::Interface => "Interface",
                Self::Constant => "Constant",
                Self::Invalid0 => "Invalid0",
                Self::Union => "Union",
                Self::Value => "Value",
                Self::Signal => "Signal",
                Self::Vfunc => "Vfunc",
                Self::Property => "Property",
                Self::Field => "Field",
                Self::Arg => "Arg",
                Self::Type => "Type",
                Self::Unresolved => "Unresolved",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for InfoType {
    type GlibType = ffi::GIInfoType;

    fn into_glib(self) -> ffi::GIInfoType {
        match self {
            Self::Invalid => ffi::GI_INFO_TYPE_INVALID,
            Self::Function => ffi::GI_INFO_TYPE_FUNCTION,
            Self::Callback => ffi::GI_INFO_TYPE_CALLBACK,
            Self::Struct => ffi::GI_INFO_TYPE_STRUCT,
            Self::Boxed => ffi::GI_INFO_TYPE_BOXED,
            Self::Enum => ffi::GI_INFO_TYPE_ENUM,
            Self::Flags => ffi::GI_INFO_TYPE_FLAGS,
            Self::Object => ffi::GI_INFO_TYPE_OBJECT,
            Self::Interface => ffi::GI_INFO_TYPE_INTERFACE,
            Self::Constant => ffi::GI_INFO_TYPE_CONSTANT,
            Self::Invalid0 => ffi::GI_INFO_TYPE_INVALID_0,
            Self::Union => ffi::GI_INFO_TYPE_UNION,
            Self::Value => ffi::GI_INFO_TYPE_VALUE,
            Self::Signal => ffi::GI_INFO_TYPE_SIGNAL,
            Self::Vfunc => ffi::GI_INFO_TYPE_VFUNC,
            Self::Property => ffi::GI_INFO_TYPE_PROPERTY,
            Self::Field => ffi::GI_INFO_TYPE_FIELD,
            Self::Arg => ffi::GI_INFO_TYPE_ARG,
            Self::Type => ffi::GI_INFO_TYPE_TYPE,
            Self::Unresolved => ffi::GI_INFO_TYPE_UNRESOLVED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GIInfoType> for InfoType {
    unsafe fn from_glib(value: ffi::GIInfoType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GI_INFO_TYPE_INVALID => Self::Invalid,
            ffi::GI_INFO_TYPE_FUNCTION => Self::Function,
            ffi::GI_INFO_TYPE_CALLBACK => Self::Callback,
            ffi::GI_INFO_TYPE_STRUCT => Self::Struct,
            ffi::GI_INFO_TYPE_BOXED => Self::Boxed,
            ffi::GI_INFO_TYPE_ENUM => Self::Enum,
            ffi::GI_INFO_TYPE_FLAGS => Self::Flags,
            ffi::GI_INFO_TYPE_OBJECT => Self::Object,
            ffi::GI_INFO_TYPE_INTERFACE => Self::Interface,
            ffi::GI_INFO_TYPE_CONSTANT => Self::Constant,
            ffi::GI_INFO_TYPE_INVALID_0 => Self::Invalid0,
            ffi::GI_INFO_TYPE_UNION => Self::Union,
            ffi::GI_INFO_TYPE_VALUE => Self::Value,
            ffi::GI_INFO_TYPE_SIGNAL => Self::Signal,
            ffi::GI_INFO_TYPE_VFUNC => Self::Vfunc,
            ffi::GI_INFO_TYPE_PROPERTY => Self::Property,
            ffi::GI_INFO_TYPE_FIELD => Self::Field,
            ffi::GI_INFO_TYPE_ARG => Self::Arg,
            ffi::GI_INFO_TYPE_TYPE => Self::Type,
            ffi::GI_INFO_TYPE_UNRESOLVED => Self::Unresolved,
            value => Self::__Unknown(value),
        }
    }
}

/// An error code used with `G_IREPOSITORY_ERROR` in a [`glib::Error`][crate::glib::Error] returned
/// from a [`Repository`][crate::Repository] routine.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GIRepositoryError")]
pub enum RepositoryError {
    /// the typelib could not be found.
    #[doc(alias = "G_IREPOSITORY_ERROR_TYPELIB_NOT_FOUND")]
    TypelibNotFound,
    /// the namespace does not match the
    ///  requested namespace.
    #[doc(alias = "G_IREPOSITORY_ERROR_NAMESPACE_MISMATCH")]
    NamespaceMismatch,
    /// the version of the
    ///  typelib does not match the requested version.
    #[doc(alias = "G_IREPOSITORY_ERROR_NAMESPACE_VERSION_CONFLICT")]
    NamespaceVersionConflict,
    /// the library used by the typelib
    ///  could not be found.
    #[doc(alias = "G_IREPOSITORY_ERROR_LIBRARY_NOT_FOUND")]
    LibraryNotFound,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RepositoryError::{}",
            match *self {
                Self::TypelibNotFound => "TypelibNotFound",
                Self::NamespaceMismatch => "NamespaceMismatch",
                Self::NamespaceVersionConflict => "NamespaceVersionConflict",
                Self::LibraryNotFound => "LibraryNotFound",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for RepositoryError {
    type GlibType = ffi::GIRepositoryError;

    fn into_glib(self) -> ffi::GIRepositoryError {
        match self {
            Self::TypelibNotFound => ffi::G_IREPOSITORY_ERROR_TYPELIB_NOT_FOUND,
            Self::NamespaceMismatch => ffi::G_IREPOSITORY_ERROR_NAMESPACE_MISMATCH,
            Self::NamespaceVersionConflict => ffi::G_IREPOSITORY_ERROR_NAMESPACE_VERSION_CONFLICT,
            Self::LibraryNotFound => ffi::G_IREPOSITORY_ERROR_LIBRARY_NOT_FOUND,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GIRepositoryError> for RepositoryError {
    unsafe fn from_glib(value: ffi::GIRepositoryError) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::G_IREPOSITORY_ERROR_TYPELIB_NOT_FOUND => Self::TypelibNotFound,
            ffi::G_IREPOSITORY_ERROR_NAMESPACE_MISMATCH => Self::NamespaceMismatch,
            ffi::G_IREPOSITORY_ERROR_NAMESPACE_VERSION_CONFLICT => Self::NamespaceVersionConflict,
            ffi::G_IREPOSITORY_ERROR_LIBRARY_NOT_FOUND => Self::LibraryNotFound,
            value => Self::__Unknown(value),
        }
    }
}

/// Scope type of a `GIArgInfo` representing callback, determines how the
/// callback is invoked and is used to decided when the invoke structs
/// can be freed.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GIScopeType")]
pub enum ScopeType {
    /// The argument is not of callback type.
    #[doc(alias = "GI_SCOPE_TYPE_INVALID")]
    Invalid,
    /// The callback and associated user_data is only
    ///  used during the call to this function.
    #[doc(alias = "GI_SCOPE_TYPE_CALL")]
    Call,
    /// The callback and associated user_data is
    ///  only used until the callback is invoked, and the callback.
    ///  is invoked always exactly once.
    #[doc(alias = "GI_SCOPE_TYPE_ASYNC")]
    Async,
    /// The callback and associated
    ///  user_data is used until the caller is notfied via the destroy_notify.
    #[doc(alias = "GI_SCOPE_TYPE_NOTIFIED")]
    Notified,
    /// The callback and associated user_data is
    ///  used until the process terminates
    #[doc(alias = "GI_SCOPE_TYPE_FOREVER")]
    Forever,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ScopeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ScopeType::{}",
            match *self {
                Self::Invalid => "Invalid",
                Self::Call => "Call",
                Self::Async => "Async",
                Self::Notified => "Notified",
                Self::Forever => "Forever",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ScopeType {
    type GlibType = ffi::GIScopeType;

    fn into_glib(self) -> ffi::GIScopeType {
        match self {
            Self::Invalid => ffi::GI_SCOPE_TYPE_INVALID,
            Self::Call => ffi::GI_SCOPE_TYPE_CALL,
            Self::Async => ffi::GI_SCOPE_TYPE_ASYNC,
            Self::Notified => ffi::GI_SCOPE_TYPE_NOTIFIED,
            Self::Forever => ffi::GI_SCOPE_TYPE_FOREVER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GIScopeType> for ScopeType {
    unsafe fn from_glib(value: ffi::GIScopeType) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GI_SCOPE_TYPE_INVALID => Self::Invalid,
            ffi::GI_SCOPE_TYPE_CALL => Self::Call,
            ffi::GI_SCOPE_TYPE_ASYNC => Self::Async,
            ffi::GI_SCOPE_TYPE_NOTIFIED => Self::Notified,
            ffi::GI_SCOPE_TYPE_FOREVER => Self::Forever,
            value => Self::__Unknown(value),
        }
    }
}

/// The transfer is the exchange of data between two parts, from the callee to
/// the caller. The callee is either a function/method/signal or an
/// object/interface where a property is defined. The caller is the side
/// accessing a property or calling a function.
/// [`Transfer`][crate::Transfer] specifies who's responsible for freeing the resources after the
/// ownership transfer is complete. In case of a containing type such as a list,
/// an array or a hash table the container itself is specified differently from
/// the items within the container itself. Each container is freed differently,
/// check the documentation for the types themselves for information on how to
/// free them.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GITransfer")]
pub enum Transfer {
    /// transfer nothing from the callee (function or the type
    /// instance the property belongs to) to the caller. The callee retains the
    /// ownership of the transfer and the caller doesn't need to do anything to free
    /// up the resources of this transfer.
    #[doc(alias = "GI_TRANSFER_NOTHING")]
    Nothing,
    /// transfer the container (list, array, hash table) from
    /// the callee to the caller. The callee retains the ownership of the individual
    /// items in the container and the caller has to free up the container resources
    /// (`g_list_free()`/`g_hash_table_destroy()` etc) of this transfer.
    #[doc(alias = "GI_TRANSFER_CONTAINER")]
    Container,
    /// transfer everything, eg the container and its
    /// contents from the callee to the caller. This is the case when the callee
    /// creates a copy of all the data it returns. The caller is responsible for
    /// cleaning up the container and item resources of this transfer.
    #[doc(alias = "GI_TRANSFER_EVERYTHING")]
    Everything,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Transfer::{}",
            match *self {
                Self::Nothing => "Nothing",
                Self::Container => "Container",
                Self::Everything => "Everything",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Transfer {
    type GlibType = ffi::GITransfer;

    fn into_glib(self) -> ffi::GITransfer {
        match self {
            Self::Nothing => ffi::GI_TRANSFER_NOTHING,
            Self::Container => ffi::GI_TRANSFER_CONTAINER,
            Self::Everything => ffi::GI_TRANSFER_EVERYTHING,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GITransfer> for Transfer {
    unsafe fn from_glib(value: ffi::GITransfer) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GI_TRANSFER_NOTHING => Self::Nothing,
            ffi::GI_TRANSFER_CONTAINER => Self::Container,
            ffi::GI_TRANSFER_EVERYTHING => Self::Everything,
            value => Self::__Unknown(value),
        }
    }
}

/// The type tag of a `GITypeInfo`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GITypeTag")]
pub enum TypeTag {
    /// void
    #[doc(alias = "GI_TYPE_TAG_VOID")]
    Void,
    /// boolean
    #[doc(alias = "GI_TYPE_TAG_BOOLEAN")]
    Boolean,
    /// 8-bit signed integer
    #[doc(alias = "GI_TYPE_TAG_INT8")]
    Int8,
    /// 8-bit unsigned integer
    #[doc(alias = "GI_TYPE_TAG_UINT8")]
    Uint8,
    /// 16-bit signed integer
    #[doc(alias = "GI_TYPE_TAG_INT16")]
    Int16,
    /// 16-bit unsigned integer
    #[doc(alias = "GI_TYPE_TAG_UINT16")]
    Uint16,
    /// 32-bit signed integer
    #[doc(alias = "GI_TYPE_TAG_INT32")]
    Int32,
    /// 32-bit unsigned integer
    #[doc(alias = "GI_TYPE_TAG_UINT32")]
    Uint32,
    /// 64-bit signed integer
    #[doc(alias = "GI_TYPE_TAG_INT64")]
    Int64,
    /// 64-bit unsigned integer
    #[doc(alias = "GI_TYPE_TAG_UINT64")]
    Uint64,
    /// float
    #[doc(alias = "GI_TYPE_TAG_FLOAT")]
    Float,
    /// double floating point
    #[doc(alias = "GI_TYPE_TAG_DOUBLE")]
    Double,
    /// a `GType`
    #[doc(alias = "GI_TYPE_TAG_GTYPE")]
    Gtype,
    /// a UTF-8 encoded string
    #[doc(alias = "GI_TYPE_TAG_UTF8")]
    Utf8,
    /// a filename, encoded in the same encoding
    ///  as the native filesystem is using.
    #[doc(alias = "GI_TYPE_TAG_FILENAME")]
    Filename,
    /// an array
    #[doc(alias = "GI_TYPE_TAG_ARRAY")]
    Array,
    /// an extended interface object
    #[doc(alias = "GI_TYPE_TAG_INTERFACE")]
    Interface,
    /// a `GList`
    #[doc(alias = "GI_TYPE_TAG_GLIST")]
    Glist,
    /// a `GSList`
    #[doc(alias = "GI_TYPE_TAG_GSLIST")]
    Gslist,
    /// a `GHashTable`
    #[doc(alias = "GI_TYPE_TAG_GHASH")]
    Ghash,
    /// a [`glib::Error`][crate::glib::Error]
    #[doc(alias = "GI_TYPE_TAG_ERROR")]
    Error,
    /// Unicode character
    #[doc(alias = "GI_TYPE_TAG_UNICHAR")]
    Unichar,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TypeTag::{}",
            match *self {
                Self::Void => "Void",
                Self::Boolean => "Boolean",
                Self::Int8 => "Int8",
                Self::Uint8 => "Uint8",
                Self::Int16 => "Int16",
                Self::Uint16 => "Uint16",
                Self::Int32 => "Int32",
                Self::Uint32 => "Uint32",
                Self::Int64 => "Int64",
                Self::Uint64 => "Uint64",
                Self::Float => "Float",
                Self::Double => "Double",
                Self::Gtype => "Gtype",
                Self::Utf8 => "Utf8",
                Self::Filename => "Filename",
                Self::Array => "Array",
                Self::Interface => "Interface",
                Self::Glist => "Glist",
                Self::Gslist => "Gslist",
                Self::Ghash => "Ghash",
                Self::Error => "Error",
                Self::Unichar => "Unichar",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for TypeTag {
    type GlibType = ffi::GITypeTag;

    fn into_glib(self) -> ffi::GITypeTag {
        match self {
            Self::Void => ffi::GI_TYPE_TAG_VOID,
            Self::Boolean => ffi::GI_TYPE_TAG_BOOLEAN,
            Self::Int8 => ffi::GI_TYPE_TAG_INT8,
            Self::Uint8 => ffi::GI_TYPE_TAG_UINT8,
            Self::Int16 => ffi::GI_TYPE_TAG_INT16,
            Self::Uint16 => ffi::GI_TYPE_TAG_UINT16,
            Self::Int32 => ffi::GI_TYPE_TAG_INT32,
            Self::Uint32 => ffi::GI_TYPE_TAG_UINT32,
            Self::Int64 => ffi::GI_TYPE_TAG_INT64,
            Self::Uint64 => ffi::GI_TYPE_TAG_UINT64,
            Self::Float => ffi::GI_TYPE_TAG_FLOAT,
            Self::Double => ffi::GI_TYPE_TAG_DOUBLE,
            Self::Gtype => ffi::GI_TYPE_TAG_GTYPE,
            Self::Utf8 => ffi::GI_TYPE_TAG_UTF8,
            Self::Filename => ffi::GI_TYPE_TAG_FILENAME,
            Self::Array => ffi::GI_TYPE_TAG_ARRAY,
            Self::Interface => ffi::GI_TYPE_TAG_INTERFACE,
            Self::Glist => ffi::GI_TYPE_TAG_GLIST,
            Self::Gslist => ffi::GI_TYPE_TAG_GSLIST,
            Self::Ghash => ffi::GI_TYPE_TAG_GHASH,
            Self::Error => ffi::GI_TYPE_TAG_ERROR,
            Self::Unichar => ffi::GI_TYPE_TAG_UNICHAR,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GITypeTag> for TypeTag {
    unsafe fn from_glib(value: ffi::GITypeTag) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GI_TYPE_TAG_VOID => Self::Void,
            ffi::GI_TYPE_TAG_BOOLEAN => Self::Boolean,
            ffi::GI_TYPE_TAG_INT8 => Self::Int8,
            ffi::GI_TYPE_TAG_UINT8 => Self::Uint8,
            ffi::GI_TYPE_TAG_INT16 => Self::Int16,
            ffi::GI_TYPE_TAG_UINT16 => Self::Uint16,
            ffi::GI_TYPE_TAG_INT32 => Self::Int32,
            ffi::GI_TYPE_TAG_UINT32 => Self::Uint32,
            ffi::GI_TYPE_TAG_INT64 => Self::Int64,
            ffi::GI_TYPE_TAG_UINT64 => Self::Uint64,
            ffi::GI_TYPE_TAG_FLOAT => Self::Float,
            ffi::GI_TYPE_TAG_DOUBLE => Self::Double,
            ffi::GI_TYPE_TAG_GTYPE => Self::Gtype,
            ffi::GI_TYPE_TAG_UTF8 => Self::Utf8,
            ffi::GI_TYPE_TAG_FILENAME => Self::Filename,
            ffi::GI_TYPE_TAG_ARRAY => Self::Array,
            ffi::GI_TYPE_TAG_INTERFACE => Self::Interface,
            ffi::GI_TYPE_TAG_GLIST => Self::Glist,
            ffi::GI_TYPE_TAG_GSLIST => Self::Gslist,
            ffi::GI_TYPE_TAG_GHASH => Self::Ghash,
            ffi::GI_TYPE_TAG_ERROR => Self::Error,
            ffi::GI_TYPE_TAG_UNICHAR => Self::Unichar,
            value => Self::__Unknown(value),
        }
    }
}
