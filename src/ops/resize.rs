//! Image resize operations

use crate::{
    buffer::RgaBuffer,
    error::{RgaError, RgaResult},
    ffi::*,
    usage::InterpMode,
};

/// Options for resize operations
#[derive(Debug, Clone, Copy)]
pub struct ResizeOptions {
    /// X-direction resize factor (0 for automatic)
    pub fx: f64,
    /// Y-direction resize factor (0 for automatic)
    pub fy: f64,
    /// Interpolation mode
    pub interpolation: InterpMode,
}

impl Default for ResizeOptions {
    fn default() -> Self {
        Self {
            fx: 0.0,
            fy: 0.0,
            interpolation: InterpMode::Default,
        }
    }
}

impl ResizeOptions {
    /// Create options for specific scale factors
    pub fn with_scale(fx: f64, fy: f64) -> Self {
        Self {
            fx,
            fy,
            ..Default::default()
        }
    }

    /// Create options for specific interpolation
    pub fn with_interpolation(interpolation: InterpMode) -> Self {
        Self {
            interpolation,
            ..Default::default()
        }
    }
}

/// Resize image
pub fn resize(src: &RgaBuffer, dst: &mut RgaBuffer, opts: ResizeOptions) -> RgaResult<()> {
    let status = unsafe {
        imresize_t(
            src.wrap(),
            dst.wrap(),
            opts.fx,
            opts.fy,
            opts.interpolation as i32,
            1, // sync
        )
    };

    if status == IM_STATUS_IM_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(RgaError::from_status(status))
    }
}
