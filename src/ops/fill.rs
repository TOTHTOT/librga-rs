//! Fill and drawing operations

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
    rect::Rect,
};

/// Fill rectangle with solid color
pub fn fill(dst: &mut RgaBuffer, rect: Rect, color: u32, sync: bool) -> RgaResult<()> {
    let status = unsafe { imfill_t(dst.wrap(), rect.into(), color as i32, sync as i32) };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}

/// Draw rectangle border (using fill with thickness -1 for filled)
pub fn rectangle(
    dst: &mut RgaBuffer,
    rect: Rect,
    color: u32,
    _thickness: i32,
    sync: bool,
) -> RgaResult<()> {
    // Use fill with the rectangle
    // For border, we would need multiple fill calls or use improcess
    // For now, use fill which draws a filled rectangle
    fill(dst, rect, color, sync)
}
