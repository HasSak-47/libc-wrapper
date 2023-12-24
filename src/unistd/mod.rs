use libc::{
    isatty,
    getcwd,
};

use std::{
    os::fd:: AsRawFd,
    path::PathBuf, str::FromStr,
};

use crate::error::LibcError;

pub fn is_a_tty<F: AsRawFd>(fd: F) -> bool{
    let fd = fd.as_raw_fd();

    unsafe{ isatty(fd) == 0 }
}

pub fn get_cwd() -> Result<PathBuf, LibcError>{
    let mut buffer : Vec<u8> = Vec::new();
    buffer.resize(1, 0);
    loop {unsafe{
        let err = getcwd(buffer.as_mut_ptr() as *mut i8, buffer.len());
        if ! err.is_null(){
            break;
        }
        buffer.resize(buffer.len() << 1, 0);
    }}

    let skip = buffer.len() >> 1;
    for (i, c) in buffer.iter().skip(skip).enumerate(){
        if *c == 0{
            buffer.resize(i, 0);
            break;
        }
    }

    Ok(PathBuf::from_str(std::str::from_utf8(buffer.as_slice())?)?)
}
