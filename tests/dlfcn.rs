use anyhow::Result;
use cwrapper::dlfcn::{self, CFunc};

#[test]
fn test() -> Result<()> {
    let mut link = dlfcn::DynamicLink::open("./tests/test.so", dlfcn::DynamicLinkArg::LAZY)?;
    let mut function: CFunc<(i32, i32, i32), ()> = link.get_function("test_0")?;
    function.call((10, 11, 12));

    return Ok(());
}
