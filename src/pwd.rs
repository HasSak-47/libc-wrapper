use std::{ffi::CStr, path::PathBuf};

use libc::{geteuid, getgid, getpwuid};

use crate::error::{LibcError, LibcResult};

#[derive(Debug, Default)]
pub struct Passwd{
    pub name: String,
    pub passwd: String, /* unimplemented */
    pub uid: u32,
    pub gid: u32,
    pub gecos: String, /* unimplemented */
    pub dir: PathBuf,
    pub shell: String, /* unimplemented */
}

pub fn get_passwd() -> LibcResult<Passwd>{
    unsafe{
        let uid = geteuid();
        let gid = getgid();
        let pwd = getpwuid(uid);
        if pwd.is_null() {
            return Err(LibcError::ReturnedNull(format!("getpwuid({uid})")))
        }

        let name = CStr::from_ptr((*pwd).pw_name).to_str()?.to_string();
        let dir = CStr::from_ptr((*pwd).pw_dir).to_str()?.to_string();
        let dir = PathBuf::from(dir);

        Ok(Passwd {
            name,
            uid,
            gid,
            dir,
            ..Default::default()
        })
    }
}
