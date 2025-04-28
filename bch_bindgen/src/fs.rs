use crate::c;
use crate::errcode::{bch_errcode, errptr_to_result};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

pub struct Fs {
    pub raw: *mut c::bch_fs,
}

impl Fs {
    pub fn open(devs: &[PathBuf], mut opts: c::bch_opts) -> Result<Fs, bch_errcode> {
        let devs_cstrs : Vec<_> = devs
            .iter()
            .map(|i| CString::new(i.as_os_str().as_bytes()).unwrap())
            .collect();

        let mut devs_array: Vec<_> = devs_cstrs
            .iter()
            .map(|i| i.as_ptr())
            .collect();

        let ret = unsafe {
            let mut devs: c::darray_const_str = std::mem::zeroed();

            devs.data = devs_array[..].as_mut_ptr();
            devs.nr = devs_array.len();

            c::bch2_fs_open(&mut devs, &mut opts)
        };

        errptr_to_result(ret).map(|fs| Fs { raw: fs })
    }
}

impl Drop for Fs {
    fn drop(&mut self) {
        unsafe { c::bch2_fs_stop(self.raw) }
    }
}
