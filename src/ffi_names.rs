//! FFI name mappings for librga
//!
//! This module re-exports FFI constants and functions with cleaner names.

pub use crate::ffi::*;

// Re-export status constants with simpler names
pub const IM_STATUS_SUCCESS: IM_STATUS = IM_STATUS_IM_STATUS_SUCCESS;
pub const IM_STATUS_NOERROR: IM_STATUS = IM_STATUS_IM_STATUS_NOERROR;
pub const IM_STATUS_NOT_SUPPORTED: IM_STATUS = IM_STATUS_IM_STATUS_NOT_SUPPORTED;
pub const IM_STATUS_OUT_OF_MEMORY: IM_STATUS = IM_STATUS_IM_STATUS_OUT_OF_MEMORY;
pub const IM_STATUS_INVALID_PARAM: IM_STATUS = IM_STATUS_IM_STATUS_INVALID_PARAM;
pub const IM_STATUS_ILLEGAL_PARAM: IM_STATUS = IM_STATUS_IM_STATUS_ILLEGAL_PARAM;
pub const IM_STATUS_ERROR_VERSION: IM_STATUS = IM_STATUS_IM_STATUS_ERROR_VERSION;
pub const IM_STATUS_NO_SESSION: IM_STATUS = IM_STATUS_IM_STATUS_NO_SESSION;
pub const IM_STATUS_FAILED: IM_STATUS = IM_STATUS_IM_STATUS_FAILED;

// Re-export functions with simpler names (only those that exist in FFI)
pub use crate::ffi::imcopy_t as imcopy;
pub use crate::ffi::imresize_t as imresize;
pub use crate::ffi::imcrop_t as imcrop;
pub use crate::ffi::imtranslate_t as imtranslate;
pub use crate::ffi::imcvtcolor_t as imcvtcolor;
pub use crate::ffi::imrotate_t as imrotate;
pub use crate::ffi::imflip_t as imflip;
pub use crate::ffi::imblend_t as imblend;
pub use crate::ffi::imcolorkey_t as imcolorkey;
pub use crate::ffi::imquantize_t as imquantize;
pub use crate::ffi::imrop_t as imrop;
pub use crate::ffi::imfill_t as imfill;
pub use crate::ffi::imgaussianBlur_t as imgaussian_blur;
pub use crate::ffi::impalette_t as impalette;
pub use crate::ffi::imStrError_t as imstr_error;
pub use crate::ffi::imcheck_t as imcheck;

// Re-export IM_USAGE constants with simpler names
pub use crate::ffi::IM_USAGE_IM_HAL_TRANSFORM_ROT_90 as IM_HAL_TRANSFORM_ROT_90;
pub use crate::ffi::IM_USAGE_IM_HAL_TRANSFORM_ROT_180 as IM_HAL_TRANSFORM_ROT_180;
pub use crate::ffi::IM_USAGE_IM_HAL_TRANSFORM_ROT_270 as IM_HAL_TRANSFORM_ROT_270;
pub use crate::ffi::IM_USAGE_IM_HAL_TRANSFORM_FLIP_H as IM_HAL_TRANSFORM_FLIP_H;
pub use crate::ffi::IM_USAGE_IM_HAL_TRANSFORM_FLIP_V as IM_HAL_TRANSFORM_FLIP_V;
pub use crate::ffi::IM_USAGE_IM_ALPHA_BLEND_SRC_OVER as IM_ALPHA_BLEND_SRC_OVER;
pub use crate::ffi::IM_USAGE_IM_ALPHA_BLEND_SRC as IM_ALPHA_BLEND_SRC;
pub use crate::ffi::IM_USAGE_IM_ALPHA_BLEND_DST as IM_ALPHA_BLEND_DST;
pub use crate::ffi::IM_USAGE_IM_SYNC as IM_SYNC;
pub use crate::ffi::IM_USAGE_IM_ASYNC as IM_ASYNC;
pub use crate::ffi::IM_USAGE_IM_COLOR_FILL as IM_COLOR_FILL;
pub use crate::ffi::IM_USAGE_IM_COLOR_PALETTE as IM_COLOR_PALETTE;
pub use crate::ffi::IM_USAGE_IM_MOSAIC as IM_MOSAIC;
pub use crate::ffi::IM_USAGE_IM_GAUSS as IM_GAUSS;
pub use crate::ffi::IM_USAGE_IM_ALPHA_COLORKEY_NORMAL as IM_ALPHA_COLORKEY_NORMAL;
pub use crate::ffi::IM_USAGE_IM_ALPHA_COLORKEY_INVERTED as IM_ALPHA_COLORKEY_INVERTED;
pub use crate::ffi::IM_USAGE_IM_NN_QUANTIZE as IM_NN_QUANTIZE;
pub use crate::ffi::IM_USAGE_IM_ROP as IM_ROP;
pub use crate::ffi::IM_USAGE_IM_OSD as IM_OSD;
pub use crate::ffi::IM_USAGE_IM_PRE_INTR as IM_PRE_INTR;
