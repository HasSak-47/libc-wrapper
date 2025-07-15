use std::ffi::c_int;

use anyhow::Result;
use cwrapper::dlfcn::{DynamicLink, DynamicLinkArg};

#[test]
fn test_0() -> Result<()> {
    let mut link = DynamicLink::open("./tests/test.so", DynamicLinkArg::LAZY)?;
    unsafe {
        let test_0: extern "C" fn(c_int, c_int, c_int) -> c_int =
            std::mem::transmute(link.get_function::<c_int, &str>("test_0")?);
        let test_1: extern "C" fn(c_int, c_int, c_int) -> c_int =
            std::mem::transmute(link.get_function::<c_int, &str>("test_1")?);
        test_0(10, 11, 12);
        test_1(10, 11, 12);
    }

    return Ok(());
}
