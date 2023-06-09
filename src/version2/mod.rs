//! The [Web Open Font Format][1] of version 2.0.
//!
//! [1]: https://www.w3.org/TR/WOFF2/

use std::io::{Error, ErrorKind, Result};
use std::path::Path;

mod ffi;

/// Compress.
pub fn compress(data: &[u8], metadata: String, quality: usize, transform: bool) -> Option<Vec<u8>> {
    let metadata_length = metadata.len();
    let metadata = match std::ffi::CString::new(metadata) {
        Ok(metadata) => metadata,
        _ => return None,
    };
    let size = unsafe {
        ffi::MaxWOFF2CompressedSize(
            data.as_ptr() as *const _,
            data.len() as libc::size_t,
            metadata.as_ptr() as *const _,
            metadata_length as libc::size_t,
        )
    };
    let mut result = vec![0; size];
    let mut result_length = result.len() as libc::size_t;
    let success = unsafe {
        ffi::ConvertTTFToWOFF2(
            data.as_ptr() as *const _,
            data.len() as libc::size_t,
            result.as_mut_ptr() as *mut _,
            &mut result_length as *mut _,
            metadata.as_ptr() as *const _,
            metadata_length as libc::size_t,
            quality as libc::c_int,
            transform as libc::c_int,
        ) != 0
    };
    if success {
        result.truncate(result_length);
        Some(result)
    } else {
        None
    }
}

/// Convert.
pub fn convert<T: AsRef<Path>>(
    source: T,
    destination: T,
    metadata: String,
    quality: usize,
    transform: bool,
) -> Result<()> {
    let data = std::fs::read(source)?;
    let data = match compress(&data, metadata, quality, transform) {
        Some(data) => data,
        _ => return Err(Error::new(ErrorKind::Other, "failed to compress")),
    };
    std::fs::write(destination, data)
}

#[cfg(test)]
mod tests {

    #[test]
    fn convert() {
        let result = super::convert(
            "tests/fixtures/Roboto-Regular.ttf",
            "tests/fixtures/Roboto-Regular.woff2",
            "".into(),
            8,
            true,
        );
        assert!(result.is_ok());
    }
}
