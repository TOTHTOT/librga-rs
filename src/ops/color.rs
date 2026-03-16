//! Color format conversion operations

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
    format::PixelFormat,
};

/// Color space conversion modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ColorSpaceMode {
    /// Default color space
    Default = 0,
    /// YUV to RGB: BT.601 limited range
    YuvToRgbBt601Limit = 1,
    /// YUV to RGB: BT.601 full range
    YuvToRgbBt601Full = 2,
    /// YUV to RGB: BT.709 limited range
    YuvToRgbBt709Limit = 3,
    /// RGB to YUV: BT.601 full range
    RgbToYuvBt601Full = 1 << 2,
    /// RGB to YUV: BT.601 limited range
    RgbToYuvBt601Limit = 2 << 2,
    /// RGB to YUV: BT.709 limited range
    RgbToYuvBt709Limit = 3 << 2,
}

impl Default for ColorSpaceMode {
    fn default() -> Self {
        Self::Default
    }
}

/// Options for color conversion
#[derive(Debug, Clone, Copy)]
pub struct ColorConvertOptions {
    /// Source format (can be different from buffer format)
    pub src_fmt: PixelFormat,
    /// Destination format (can be different from buffer format)
    pub dst_fmt: PixelFormat,
    /// Color space conversion mode
    pub mode: ColorSpaceMode,
}

impl Default for ColorConvertOptions {
    fn default() -> Self {
        Self {
            src_fmt: PixelFormat::Rgba8888,
            dst_fmt: PixelFormat::Rgba8888,
            mode: ColorSpaceMode::Default,
        }
    }
}

/// Convert color format
pub fn convert_color(
    src: &RgaBuffer,
    dst: &mut RgaBuffer,
    src_fmt: PixelFormat,
    dst_fmt: PixelFormat,
    mode: ColorSpaceMode,
    sync: bool,
) -> RgaResult<()> {
    let status = unsafe {
        imcvtcolor_t(
            src.wrap(),
            dst.wrap(),
            src_fmt as i32,
            dst_fmt as i32,
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
