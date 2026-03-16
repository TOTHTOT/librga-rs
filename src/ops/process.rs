//! General process function for advanced operations

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
    rect::Rect,
    usage::Usage,
};

/// General process function
pub fn process(
    src: &RgaBuffer,
    dst: &mut RgaBuffer,
    pat: Option<&RgaBuffer>,
    src_rect: Rect,
    dst_rect: Rect,
    pat_rect: Option<Rect>,
    usage: Usage,
) -> RgaResult<()> {
    let pat_buf = match pat {
        Some(p) => p.wrap(),
        None => unsafe { std::mem::zeroed() },
    };

    let pat_rect_c = match pat_rect {
        Some(r) => r.into(),
        None => unsafe { std::mem::zeroed() },
    };

    let status = unsafe {
        improcess(
            src.wrap(),
            dst.wrap(),
            pat_buf,
            src_rect.into(),
            dst_rect.into(),
            pat_rect_c,
            usage.bits() as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}
