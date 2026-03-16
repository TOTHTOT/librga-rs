//! Image crop operations

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
    rect::Rect,
};

/// Crop image region
pub fn crop(src: &RgaBuffer, dst: &mut RgaBuffer, rect: Rect, sync: bool) -> RgaResult<()> {
    let status = unsafe {
        imcrop_t(
            src.wrap(),
            dst.wrap(),
            rect.into(),
            sync as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}

/// Translate (move) image within destination
pub fn translate(
    src: &RgaBuffer,
    dst: &mut RgaBuffer,
    x: i32,
    y: i32,
    sync: bool,
) -> RgaResult<()> {
    let status = unsafe {
        imtranslate_t(
            src.wrap(),
            dst.wrap(),
            x,
            y,
            sync as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}
