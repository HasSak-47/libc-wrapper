use bitflags::bitflags;
use std::{
    ffi::{c_char, c_void, CString, NulError},
    marker::{PhantomData, Tuple},
    path::{Path, PathBuf},
};
use thiserror::Error;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub struct DynamicLinkArg: i32 {
        const NOW = libc::RTLD_NOW;
        const LAZY = libc::RTLD_LAZY;
        const GLOBAL = libc::RTLD_GLOBAL;
        const LOCAL = libc::RTLD_LOCAL;
    }
}

#[derive(Error, Debug, Default)]
pub enum Error {
    #[default]
    #[error("could not load dynamic link")]
    CouldNotLoad,

    #[error("Symbol not found")]
    SymbolNotFound,

    #[error("Path {0} not found")]
    PathNotFound(String),

    #[error("Invalid Path")]
    InvalidPath,

    #[error("Invalid string could not make null terminated")]
    InvalidString(#[from] NulError),
}

pub struct DynamicLink {
    handler: *mut c_void,
}

pub struct CFunc<Params: Tuple, RetType> {
    loc: *mut c_void,
    pfunc: PhantomData<RetType>,
    pparam: PhantomData<Params>,
}

impl<Params: Tuple, RetType> CFunc<Params, RetType> {
    pub fn new(loc: *mut c_void) -> Self {
        return Self {
            loc,
            pparam: PhantomData {},
            pfunc: PhantomData {},
        };
    }

    pub fn call(&mut self, params: Params) -> RetType {
        unsafe {
            let f: extern "C" fn(Params) -> RetType = std::mem::transmute(self.loc);
            return f(params);
        }
    }
    pub unsafe fn leak(&mut self) -> *mut c_void {
        return self.loc;
    }
}

impl DynamicLink {
    #[allow(dead_code)]
    pub fn open<P>(path: P, flag: DynamicLinkArg) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_str();
        let str = path.ok_or(Error::InvalidPath)?;
        let cstr = CString::new(str)?;

        unsafe {
            let handler = libc::dlopen(cstr.as_ptr(), flag.bits());
            if handler.is_null() {
                return Err(Error::PathNotFound(String::from(str)));
            }
            return Ok(Self { handler });
        }
    }

    pub fn get_function<Params: Tuple, RetType, S: AsRef<str>>(
        &mut self,
        name: S,
    ) -> Result<CFunc<Params, RetType>, Error> {
        let name = name.as_ref();
        let cstr = CString::new(name)?;

        unsafe {
            let loc = libc::dlsym(self.handler, cstr.as_ptr());
            println!("loc: {loc:?}");
            if loc.is_null() {
                return Err(Error::SymbolNotFound);
            }
            return Ok(CFunc::new(loc));
        }
    }
}

impl Drop for DynamicLink {
    fn drop(&mut self) {
        unsafe {
            libc::dlclose(self.handler);
        }
    }
}
