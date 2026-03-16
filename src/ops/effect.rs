//! Special effects operations (mosaic, gaussian blur, palette)

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
    rect::Rect,
    usage::MosaicMode,
};

/// Apply mosaic effect to a region
/// Note: immosaic function may not be available in all versions
pub fn mosaic(dst: &mut RgaBuffer, rect: Rect, mode: MosaicMode, sync: bool) -> RgaResult<()> {
    // Use improcess instead with mosaic flag
    let usage = crate::usage::Usage::from_bits(IM_USAGE_IM_MOSAIC as u32)
        .unwrap_or(crate::usage::Usage::empty());

    let status = unsafe {
        improcess(
            dst.wrap(),
            dst.wrap(),
            std::mem::zeroed(),
            rect.into(),
            rect.into(),
            std::mem::zeroed(),
            usage.bits() as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}

/// Options for Gaussian blur
#[derive(Debug, Clone, Copy)]
pub struct GaussianBlurOptions {
    pub width: i32,
    pub height: i32,
    pub sigma_x: i32,
    pub sigma_y: i32,
}

impl Default for GaussianBlurOptions {
    fn default() -> Self {
        Self {
            width: 3,
            height: 3,
            sigma_x: 0,
            sigma_y: 0,
        }
    }
}

/// Apply Gaussian blur
pub fn gaussian_blur(
    src: &RgaBuffer,
    dst: &mut RgaBuffer,
    ksize: (i32, i32),
    sigma: (i32, i32),
    sync: bool,
) -> RgaResult<()> {
    let status = unsafe {
        imgaussianBlur_t(
            src.wrap(),
            dst.wrap(),
            ksize.0,
            ksize.1,
            sigma.0,
            sigma.1,
            sync as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}

/// Apply palette mapping (LUT transformation)
pub fn palette(src: &RgaBuffer, dst: &mut RgaBuffer, lut: &RgaBuffer, sync: bool) -> RgaResult<()> {
    let status = unsafe {
        impalette_t(
            src.wrap(),
            dst.wrap(),
            lut.wrap(),
            sync as i32,
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}
