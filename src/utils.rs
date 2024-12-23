use std::ffi::CString;

pub fn perror(s: impl AsRef<str>) {
    let cstr = CString::new(s.as_ref()).unwrap();
    // SAFETY: pointer constructed from a valid CString.
    unsafe {
        libc::perror(cstr.as_ptr());
    }
}
