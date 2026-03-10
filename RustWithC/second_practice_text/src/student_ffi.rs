use std::ffi::{CString, NulError};
use std::fmt;
use std::os::raw::{c_char, c_int};

#[derive(Debug, PartialEq, Eq)]
pub enum FfiError {
    InteriorNul,
    CStatus(i32),
    NotImplemented(&'static str),
}

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InteriorNul => write!(f, "input contains an interior NUL byte"),
            Self::CStatus(code) => write!(f, "C function returned status code {code}"),
            Self::NotImplemented(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<NulError> for FfiError {
    fn from(_: NulError) -> Self {
        Self::InteriorNul
    }
}

unsafe extern "C" {
    fn txt_count_vowels(input: *const c_char, out_count: *mut c_int) -> c_int;
    #[allow(dead_code)]
    fn txt_to_upper_ascii(buffer: *mut c_char, len: usize) -> c_int;
    #[allow(dead_code)]
    fn txt_starts_with(
        input: *const c_char,
        prefix: *const c_char,
        out_result: *mut c_int,
    ) -> c_int;
}

fn map_status(status: c_int) -> Result<(), FfiError> {
    if status == 0 {
        Ok(())
    } else {
        Err(FfiError::CStatus(status))
    }
}

pub fn count_vowels(input: &str) -> Result<i32, FfiError> {
    let c_input = CString::new(input)?;
    let mut out_count: c_int = 0;

    let status = unsafe { txt_count_vowels(c_input.as_ptr(), &mut out_count) };

    map_status(status)?;
    Ok(out_count)
}

pub fn to_upper_ascii(_input: &str) -> Result<String, FfiError> {
    Err(FfiError::NotImplemented(
        "TODO 2: implement to_upper_ascii in src/student_ffi.rs",
    ))
}

pub fn starts_with(_input: &str, _prefix: &str) -> Result<bool, FfiError> {
    Err(FfiError::NotImplemented(
        "TODO 3: implement starts_with in src/student_ffi.rs",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_vowels_works() {
        let count = count_vowels("AaEeIiOoUu")
            .expect("count_vowels should work for a regular ASCII string");
        assert_eq!(count, 10);
    }

    #[test]
    fn count_vowels_rejects_interior_nul() {
        let result = count_vowels("ab\0cd");
        assert_eq!(result, Err(FfiError::InteriorNul));
    }

    #[test]
    #[ignore = "Enable after TODO 2 is implemented"]
    fn to_upper_ascii_works() {
        let value =
            to_upper_ascii("ffi Starter").expect("to_upper_ascii should return uppercased text");
        assert_eq!(value, "FFI STARTER");
    }

    #[test]
    #[ignore = "Enable after TODO 3 is implemented"]
    fn starts_with_works() {
        let value = starts_with("RUSTACEAN", "RUST").expect("starts_with should return a bool");
        assert!(value);
    }
}
