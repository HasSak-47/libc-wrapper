use bitflags::bitflags;
use std::{
    ffi::{c_void, CString, NulError},
    path::Path,
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not load dynamic link: {0}")]
    CouldNotLoad(String),

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
                let error = CString::from_raw(libc::dlerror());
                return Err(Error::CouldNotLoad(error.to_str().unwrap().to_string()));
            }
            return Ok(Self { handler });
        }
    }

    pub unsafe fn get_function<RetType, S>(
        &mut self,
        name: S,
    ) -> Result<extern "C" fn(...) -> RetType, Error>
    where
        S: AsRef<str>,
    {
        let cstr = CString::new(name.as_ref())?;

        unsafe {
            let handler = libc::dlsym(self.handler, cstr.as_ptr());
            if handler.is_null() {
                return Err(Error::SymbolNotFound);
            }
            return Ok(std::mem::transmute(handler));
        }
    }

    pub unsafe fn get_variable<T, S: AsRef<str>>(&mut self, name: S) -> Result<&mut T, Error> {
        let cstr = CString::new(name.as_ref())?;

        unsafe {
            let handler = libc::dlsym(self.handler, cstr.as_ptr());
            if handler.is_null() {
                return Err(Error::SymbolNotFound);
            }
            return Ok(&mut *(handler as *mut T));
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
