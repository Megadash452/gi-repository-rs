// Generated by gir (https://github.com/gtk-rs/gir @ 3f715d99469c)
// from ../gir-files (@ 6525c06593ee)
// DO NOT EDIT

use gi_repository_sys::*;
use std::mem::{align_of, size_of};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gobject-introspection-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{name} {err}").into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG")
        .unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {cmd:?} returned {}", out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in
        RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {abi_cmd:?} failed, {output:?}").into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("GIArgInfo", Layout {size: size_of::<GIArgInfo>(), alignment: align_of::<GIArgInfo>()}),
    ("GIArgument", Layout {size: size_of::<GIArgument>(), alignment: align_of::<GIArgument>()}),
    ("GIArrayType", Layout {size: size_of::<GIArrayType>(), alignment: align_of::<GIArrayType>()}),
    ("GIAttributeIter", Layout {size: size_of::<GIAttributeIter>(), alignment: align_of::<GIAttributeIter>()}),
    ("GIBaseInfo", Layout {size: size_of::<GIBaseInfo>(), alignment: align_of::<GIBaseInfo>()}),
    ("GICallableInfo", Layout {size: size_of::<GICallableInfo>(), alignment: align_of::<GICallableInfo>()}),
    ("GICallbackInfo", Layout {size: size_of::<GICallbackInfo>(), alignment: align_of::<GICallbackInfo>()}),
    ("GIConstantInfo", Layout {size: size_of::<GIConstantInfo>(), alignment: align_of::<GIConstantInfo>()}),
    ("GIDirection", Layout {size: size_of::<GIDirection>(), alignment: align_of::<GIDirection>()}),
    ("GIEnumInfo", Layout {size: size_of::<GIEnumInfo>(), alignment: align_of::<GIEnumInfo>()}),
    ("GIFieldInfo", Layout {size: size_of::<GIFieldInfo>(), alignment: align_of::<GIFieldInfo>()}),
    ("GIFieldInfoFlags", Layout {size: size_of::<GIFieldInfoFlags>(), alignment: align_of::<GIFieldInfoFlags>()}),
    ("GIFunctionInfo", Layout {size: size_of::<GIFunctionInfo>(), alignment: align_of::<GIFunctionInfo>()}),
    ("GIFunctionInfoFlags", Layout {size: size_of::<GIFunctionInfoFlags>(), alignment: align_of::<GIFunctionInfoFlags>()}),
    ("GIInfoType", Layout {size: size_of::<GIInfoType>(), alignment: align_of::<GIInfoType>()}),
    ("GIInterfaceInfo", Layout {size: size_of::<GIInterfaceInfo>(), alignment: align_of::<GIInterfaceInfo>()}),
    ("GIObjectInfo", Layout {size: size_of::<GIObjectInfo>(), alignment: align_of::<GIObjectInfo>()}),
    ("GIPropertyInfo", Layout {size: size_of::<GIPropertyInfo>(), alignment: align_of::<GIPropertyInfo>()}),
    ("GIRegisteredTypeInfo", Layout {size: size_of::<GIRegisteredTypeInfo>(), alignment: align_of::<GIRegisteredTypeInfo>()}),
    ("GIRepository", Layout {size: size_of::<GIRepository>(), alignment: align_of::<GIRepository>()}),
    ("GIRepositoryClass", Layout {size: size_of::<GIRepositoryClass>(), alignment: align_of::<GIRepositoryClass>()}),
    ("GIRepositoryError", Layout {size: size_of::<GIRepositoryError>(), alignment: align_of::<GIRepositoryError>()}),
    ("GIRepositoryLoadFlags", Layout {size: size_of::<GIRepositoryLoadFlags>(), alignment: align_of::<GIRepositoryLoadFlags>()}),
    ("GIScopeType", Layout {size: size_of::<GIScopeType>(), alignment: align_of::<GIScopeType>()}),
    ("GISignalInfo", Layout {size: size_of::<GISignalInfo>(), alignment: align_of::<GISignalInfo>()}),
    ("GIStructInfo", Layout {size: size_of::<GIStructInfo>(), alignment: align_of::<GIStructInfo>()}),
    ("GITransfer", Layout {size: size_of::<GITransfer>(), alignment: align_of::<GITransfer>()}),
    ("GITypeInfo", Layout {size: size_of::<GITypeInfo>(), alignment: align_of::<GITypeInfo>()}),
    ("GITypeTag", Layout {size: size_of::<GITypeTag>(), alignment: align_of::<GITypeTag>()}),
    ("GIUnionInfo", Layout {size: size_of::<GIUnionInfo>(), alignment: align_of::<GIUnionInfo>()}),
    ("GIVFuncInfo", Layout {size: size_of::<GIVFuncInfo>(), alignment: align_of::<GIVFuncInfo>()}),
    ("GIVFuncInfoFlags", Layout {size: size_of::<GIVFuncInfoFlags>(), alignment: align_of::<GIVFuncInfoFlags>()}),
    ("GIValueInfo", Layout {size: size_of::<GIValueInfo>(), alignment: align_of::<GIValueInfo>()}),
    ("GInvokeError", Layout {size: size_of::<GInvokeError>(), alignment: align_of::<GInvokeError>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GI_ARRAY_TYPE_ARRAY", "1"),
    ("(gint) GI_ARRAY_TYPE_BYTE_ARRAY", "3"),
    ("(gint) GI_ARRAY_TYPE_C", "0"),
    ("(gint) GI_ARRAY_TYPE_PTR_ARRAY", "2"),
    ("(gint) GI_DIRECTION_IN", "0"),
    ("(gint) GI_DIRECTION_INOUT", "2"),
    ("(gint) GI_DIRECTION_OUT", "1"),
    ("(guint) GI_FIELD_IS_READABLE", "1"),
    ("(guint) GI_FIELD_IS_WRITABLE", "2"),
    ("(guint) GI_FUNCTION_IS_CONSTRUCTOR", "2"),
    ("(guint) GI_FUNCTION_IS_GETTER", "4"),
    ("(guint) GI_FUNCTION_IS_METHOD", "1"),
    ("(guint) GI_FUNCTION_IS_SETTER", "8"),
    ("(guint) GI_FUNCTION_THROWS", "32"),
    ("(guint) GI_FUNCTION_WRAPS_VFUNC", "16"),
    ("(gint) GI_INFO_TYPE_ARG", "17"),
    ("(gint) GI_INFO_TYPE_BOXED", "4"),
    ("(gint) GI_INFO_TYPE_CALLBACK", "2"),
    ("(gint) GI_INFO_TYPE_CONSTANT", "9"),
    ("(gint) GI_INFO_TYPE_ENUM", "5"),
    ("(gint) GI_INFO_TYPE_FIELD", "16"),
    ("(gint) GI_INFO_TYPE_FLAGS", "6"),
    ("(gint) GI_INFO_TYPE_FUNCTION", "1"),
    ("(gint) GI_INFO_TYPE_INTERFACE", "8"),
    ("(gint) GI_INFO_TYPE_INVALID", "0"),
    ("(gint) GI_INFO_TYPE_INVALID_0", "10"),
    ("(gint) GI_INFO_TYPE_OBJECT", "7"),
    ("(gint) GI_INFO_TYPE_PROPERTY", "15"),
    ("(gint) GI_INFO_TYPE_SIGNAL", "13"),
    ("(gint) GI_INFO_TYPE_STRUCT", "3"),
    ("(gint) GI_INFO_TYPE_TYPE", "18"),
    ("(gint) GI_INFO_TYPE_UNION", "11"),
    ("(gint) GI_INFO_TYPE_UNRESOLVED", "19"),
    ("(gint) GI_INFO_TYPE_VALUE", "12"),
    ("(gint) GI_INFO_TYPE_VFUNC", "14"),
    ("GI_MAJOR_VERSION", "1"),
    ("(gint) GI_SCOPE_TYPE_ASYNC", "2"),
    ("(gint) GI_SCOPE_TYPE_CALL", "1"),
    ("(gint) GI_SCOPE_TYPE_FOREVER", "4"),
    ("(gint) GI_SCOPE_TYPE_INVALID", "0"),
    ("(gint) GI_SCOPE_TYPE_NOTIFIED", "3"),
    ("(gint) GI_TRANSFER_CONTAINER", "1"),
    ("(gint) GI_TRANSFER_EVERYTHING", "2"),
    ("(gint) GI_TRANSFER_NOTHING", "0"),
    ("(gint) GI_TYPE_TAG_ARRAY", "15"),
    ("(gint) GI_TYPE_TAG_BOOLEAN", "1"),
    ("(gint) GI_TYPE_TAG_DOUBLE", "11"),
    ("(gint) GI_TYPE_TAG_ERROR", "20"),
    ("(gint) GI_TYPE_TAG_FILENAME", "14"),
    ("(gint) GI_TYPE_TAG_FLOAT", "10"),
    ("(gint) GI_TYPE_TAG_GHASH", "19"),
    ("(gint) GI_TYPE_TAG_GLIST", "17"),
    ("(gint) GI_TYPE_TAG_GSLIST", "18"),
    ("(gint) GI_TYPE_TAG_GTYPE", "12"),
    ("(gint) GI_TYPE_TAG_INT16", "4"),
    ("(gint) GI_TYPE_TAG_INT32", "6"),
    ("(gint) GI_TYPE_TAG_INT64", "8"),
    ("(gint) GI_TYPE_TAG_INT8", "2"),
    ("(gint) GI_TYPE_TAG_INTERFACE", "16"),
    ("GI_TYPE_TAG_N_TYPES", "22"),
    ("(gint) GI_TYPE_TAG_UINT16", "5"),
    ("(gint) GI_TYPE_TAG_UINT32", "7"),
    ("(gint) GI_TYPE_TAG_UINT64", "9"),
    ("(gint) GI_TYPE_TAG_UINT8", "3"),
    ("(gint) GI_TYPE_TAG_UNICHAR", "21"),
    ("(gint) GI_TYPE_TAG_UTF8", "13"),
    ("(gint) GI_TYPE_TAG_VOID", "0"),
    ("(guint) GI_VFUNC_MUST_CHAIN_UP", "1"),
    ("(guint) GI_VFUNC_MUST_NOT_OVERRIDE", "4"),
    ("(guint) GI_VFUNC_MUST_OVERRIDE", "2"),
    ("(guint) GI_VFUNC_THROWS", "8"),
    ("(gint) G_INVOKE_ERROR_ARGUMENT_MISMATCH", "2"),
    ("(gint) G_INVOKE_ERROR_FAILED", "0"),
    ("(gint) G_INVOKE_ERROR_SYMBOL_NOT_FOUND", "1"),
    ("(gint) G_IREPOSITORY_ERROR_LIBRARY_NOT_FOUND", "3"),
    ("(gint) G_IREPOSITORY_ERROR_NAMESPACE_MISMATCH", "1"),
    ("(gint) G_IREPOSITORY_ERROR_NAMESPACE_VERSION_CONFLICT", "2"),
    ("(gint) G_IREPOSITORY_ERROR_TYPELIB_NOT_FOUND", "0"),
    ("(guint) G_IREPOSITORY_LOAD_FLAG_LAZY", "1"),
];


