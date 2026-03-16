//! Image copy operations

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
};

/// Copy image from source to destination
///
/// # Arguments
/// * `src` - Source buffer
/// * `dst` - Destination buffer
/// * `sync` - Whether to wait for completion
pub fn copy(src: &RgaBuffer, dst: &mut RgaBuffer, sync: bool) -> RgaResult<()> {
    let status = unsafe {
        imcopy_t(
            src.wrap(),
            dst.wrap(),
            sync as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}
