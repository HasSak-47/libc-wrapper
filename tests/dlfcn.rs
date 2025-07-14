use std::ffi::c_int;

use anyhow::Result;
use cwrapper::dlfcn::{self, CFunc};

#[test]
fn test_0() -> Result<()> {
    let mut link = dlfcn::DynamicLink::open("./tests/test.so", dlfcn::DynamicLinkArg::LAZY)?;
    let mut function: CFunc<(c_int, c_int, c_int), ()> = link.get_function("test_0")?;
    function.call((10, 11, 12));

    return Ok(());
}

#[test]
fn test_1() -> Result<()> {
    let mut link = dlfcn::DynamicLink::open("./tests/test.so", dlfcn::DynamicLinkArg::LAZY)?;
    let mut function: CFunc<(c_int, c_int, c_int), c_int> = link.get_function("test_1")?;
    unsafe {
        let f = function.leak();
        let b = f((10, 11, 12));
        assert_eq!(b, 10 + 11 + 12, "b = 10 + 11 + 12");
    }
    let a = function.call((1, 2, 3));
    assert_eq!(a, 1 + 2 + 3, "a = 1 + 2 + 3");

    return Ok(());
}
