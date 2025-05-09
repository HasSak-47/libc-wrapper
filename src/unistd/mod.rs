
use crate::error::LibcResult;

use std::{
    os::fd::AsRawFd, path::PathBuf, str::FromStr
};

use crate::error::LibcError;


pub fn is_a_tty<F: AsRawFd>(fd: F) -> bool{
    use libc::isatty;
    let fd = fd.as_raw_fd();

    unsafe{ isatty(fd) == 0 }
}

pub fn get_cwd() -> LibcResult<PathBuf> {unsafe {
    use libc::{_PC_PATH_MAX, getcwd, fpathconf};
    let len =  fpathconf(0, _PC_PATH_MAX) as usize;
    let mut buffer = Vec::<u8>::new();
    buffer.resize(len, 0);

    let err = getcwd(buffer.as_mut_ptr() as *mut i8, len);
    if err.is_null() {
        return Err(LibcError::GenericError("could not get cwd"))
    }

    return Ok(PathBuf::from_str( std::str::from_utf8(&buffer)? )?);
}}

pub fn get_host_name() -> LibcResult<String> { unsafe{
    use libc::gethostname;
    let mut host : Vec<u8> = Vec::new();
    host.resize(1024, 0);
    gethostname(host.as_mut_ptr() as *mut i8, 1024);
    let mut index = 0;
    while host[index] != 0 && index < 1024{
        index += 1;
    }
    host.resize(index, 0);

    Ok(String::from_utf8(host)?)
}}
