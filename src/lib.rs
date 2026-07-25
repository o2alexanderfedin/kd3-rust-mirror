#![allow(unused_imports, dead_code)]

mod kd3;
mod run_test;
use crate::run_test::__main_inner;

pub(crate) type DarwinSizeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let __r: Result<(), i32> = __main_inner();
    if __r.is_ok() {
        return 0;
    }
    return __r.unwrap_err();
}

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn free(_: *mut ()) -> ();
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8) -> ();
    fn qsort(
        __base: *mut (),
        __nel: u64,
        __width: u64,
        __compar: unsafe extern "C" fn(*const (), *const ()) -> i32,
    ) -> ();
    fn malloc(__size: u64) -> *mut ();
    fn realloc(__ptr: *mut (), __size: u64) -> *mut ();
    fn __builtin_object_size(_: *const (), _: i32) -> u64;
    fn __builtin___memcpy_chk(_: *mut (), _: *const (), _: u64, _: u64) -> *mut ();
    fn printf(_: *const i8, ...) -> i32;
    fn __builtin_expect(_: i64, _: i64) -> i64;
}
