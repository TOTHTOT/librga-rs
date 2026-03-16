//! Image transformation operations (rotation, flip)

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
    usage::{FlipMode, Rotation},
};

/// Rotate image
pub fn rotate(src: &RgaBuffer, dst: &mut RgaBuffer, rotation: Rotation, sync: bool) -> RgaResult<()> {
    let status = unsafe {
        imrotate_t(
            src.wrap(),
            dst.wrap(),
            rotation.to_usage().bits() as i32,
            sync as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}

/// Flip image
pub fn flip(src: &RgaBuffer, dst: &mut RgaBuffer, mode: FlipMode, sync: bool) -> RgaResult<()> {
    let status = unsafe {
        imflip_t(
            src.wrap(),
            dst.wrap(),
            mode.to_usage().bits() as i32,
            sync as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}
