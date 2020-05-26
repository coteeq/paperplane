/* 
 * Modified version of https://github.com/lattenwald/rust-tdlib
 */

use std::{
    ffi::{ CString, CStr },
    os::raw::{ c_void, c_char, c_double, c_int, c_long },
    ptr,
};

#[allow(non_camel_case_types)]
type client_ptr = *mut c_void;

#[derive(Debug, Clone)]
pub struct Tdlib {
    instance: client_ptr,
}

// TdlibClient is Send + Sync as long as we ensure
// that there is only one receiver at a time
unsafe impl Send for Tdlib {}
unsafe impl Sync for Tdlib {}

#[link(name = "tdjson")]
extern "C" {
    fn td_json_client_create() -> client_ptr;
    fn td_json_client_send(client: client_ptr, request: *const c_char);
    fn td_json_client_receive(client: client_ptr, timeout: c_double) -> *mut c_char;
    fn td_json_client_execute(client: client_ptr, request: *const c_char) -> *mut c_char;
    fn td_json_client_destroy(client: client_ptr);

    fn td_set_log_verbosity_level(level: c_int);
    fn td_set_log_file_path(path: *const c_char) -> c_int;
    fn td_set_log_max_file_size(size: c_long);
}

impl Tdlib {
    pub fn set_log_verbosity_level(level: i32) {
        unsafe { td_set_log_verbosity_level(level) };
    }

    pub fn set_log_max_file_size(size: i64) {
        unsafe { td_set_log_max_file_size(size) };
    }

    pub fn set_log_file_path(path: Option<&str>) -> Result<bool, std::ffi::NulError> {
        let result = match path {
            None => unsafe { td_set_log_file_path(ptr::null()) },
            Some(p) => {
                let cpath = CString::new(p)?;
                unsafe { td_set_log_file_path(cpath.as_ptr()) }
            }
        };
        Ok(match result {
            0 => false,
            _ => true,
        })
    }

    pub fn new() -> Self {
        let client = unsafe { td_json_client_create() };
        Tdlib { instance: client }
    }

    pub fn send(&self, request: &str) {
        let cstring = CString::new(request).unwrap();
        unsafe { td_json_client_send(self.instance, cstring.as_ptr()) }
    }

    pub fn execute(&self, request: &str) -> Option<String> {
        let cstring = CString::new(request).unwrap();
        let result = unsafe {
            td_json_client_execute(self.instance, cstring.as_ptr())
                .as_ref()
                .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
        };
        result
    }

    pub fn receive(&self, timeout: f64) -> Option<String> {
        unsafe {
            td_json_client_receive(self.instance, timeout)
                .as_ref()
                .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
        }
    }
}

impl Drop for Tdlib {
    fn drop(&mut self) {
        unsafe {
            td_json_client_destroy(self.instance);
        }
    }
}
