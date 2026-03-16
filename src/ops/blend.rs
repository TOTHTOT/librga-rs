//! Image blending and compositing operations

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
    usage::BlendMode,
};

/// Blend foreground image onto background
/// Note: imblend_t takes srcA, srcB (foreground), dst
pub fn blend(
    fg: &RgaBuffer,
    bg: &mut RgaBuffer,
    mode: BlendMode,
    sync: bool,
) -> RgaResult<()> {
    // imblend_t takes: srcA, srcB, dst, mode, sync
    // For single-image blend, srcA can be zeroed
    let src_a = unsafe { std::mem::zeroed() };
    let status = unsafe {
        imblend_t(
            src_a,
            fg.wrap(),
            bg.wrap(),
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

/// Color key blending
pub fn color_key(
    src: &RgaBuffer,
    dst: &mut RgaBuffer,
    min_color: i32,
    max_color: i32,
    inverted: bool,
    sync: bool,
) -> RgaResult<()> {
    let range = im_colorkey_range {
        max: max_color,
        min: min_color,
    };

    let mode: crate::IM_USAGE = if inverted {
        IM_USAGE_IM_ALPHA_COLORKEY_INVERTED
    } else {
        IM_USAGE_IM_ALPHA_COLORKEY_NORMAL
    };

    let status = unsafe {
        imcolorkey_t(
            src.wrap(),
            dst.wrap(),
            range,
            mode as i32,
            sync as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}
