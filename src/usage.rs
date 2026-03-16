//! Usage flags and enums for RGA operations

use bitflags::bitflags;

bitflags! {
    /// Usage flags for RGA operations
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Usage: u32 {
        /// Rotate 90 degrees
        const ROT_90 = 1 << 0;

        /// Rotate 180 degrees
        const ROT_180 = 1 << 1;

        /// Rotate 270 degrees
        const ROT_270 = 1 << 2;

        /// Flip horizontally
        const FLIP_H = 1 << 3;

        /// Flip vertically
        const FLIP_V = 1 << 4;

        /// Flip both horizontally and vertically
        const FLIP_H_V = 1 << 5;

        /// Alpha blend: SRC over DST (default)
        const ALPHA_BLEND_SRC_OVER = 1 << 6;

        /// Alpha blend: SRC only
        const ALPHA_BLEND_SRC = 1 << 7;

        /// Alpha blend: DST only
        const ALPHA_BLEND_DST = 1 << 8;

        /// Alpha blend: SRC in DST
        const ALPHA_BLEND_SRC_IN = 1 << 9;

        /// Alpha blend: DST in SRC
        const ALPHA_BLEND_DST_IN = 1 << 10;

        /// Alpha blend: SRC out DST
        const ALPHA_BLEND_SRC_OUT = 1 << 11;

        /// Alpha blend: DST out SRC
        const ALPHA_BLEND_DST_OUT = 1 << 12;

        /// Alpha blend: DST over SRC
        const ALPHA_BLEND_DST_OVER = 1 << 13;

        /// Alpha blend: SRC atop DST
        const ALPHA_BLEND_SRC_ATOP = 1 << 14;

        /// Alpha blend: DST atop SRC
        const ALPHA_BLEND_DST_ATOP = 1 << 15;

        /// Alpha blend: XOR
        const ALPHA_BLEND_XOR = 1 << 16;

        /// Color key normal mode
        const ALPHA_COLORKEY_NORMAL = 1 << 17;

        /// Color key inverted mode
        const ALPHA_COLORKEY_INVERTED = 1 << 18;

        /// Synchronous operation (wait for completion)
        const SYNC = 1 << 19;

        /// Crop mode (unused)
        const CROP = 1 << 20;

        /// Color fill mode
        const COLOR_FILL = 1 << 21;

        /// Color palette mode
        const COLOR_PALETTE = 1 << 22;

        /// Neural network quantize mode
        const NN_QUANTIZE = 1 << 23;

        /// ROP (raster operation) mode
        const ROP = 1 << 24;

        /// Alpha blend pre-multiplied
        const ALPHA_BLEND_PRE_MUL = 1 << 25;

        /// Asynchronous operation
        const ASYNC = 1 << 26;

        /// Mosaic mode
        const MOSAIC = 1 << 27;

        /// OSD (on-screen display) mode
        const OSD = 1 << 28;

        /// Pre-interrupt mode
        const PRE_INTR = 1 << 29;

        /// Alpha bit map mode
        const ALPHA_BIT_MAP = 1 << 30;

        /// Gaussian blur mode
        const GAUSS = 1 << 31;
    }
}

impl Default for Usage {
    fn default() -> Self {
        Self::empty()
    }
}

/// Rotation options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    /// 90 degrees clockwise
    Rot90,
    /// 180 degrees
    Rot180,
    /// 270 degrees clockwise (or 90 counter-clockwise)
    Rot270,
}

impl Rotation {
    /// Convert to usage flag
    pub fn to_usage(self) -> Usage {
        match self {
            Self::Rot90 => Usage::ROT_90,
            Self::Rot180 => Usage::ROT_180,
            Self::Rot270 => Usage::ROT_270,
        }
    }
}

/// Flip options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlipMode {
    /// Horizontal flip
    Horizontal,
    /// Vertical flip
    Vertical,
    /// Both horizontal and vertical
    Both,
}

impl FlipMode {
    /// Convert to usage flag
    pub fn to_usage(self) -> Usage {
        match self {
            Self::Horizontal => Usage::FLIP_H,
            Self::Vertical => Usage::FLIP_V,
            Self::Both => Usage::FLIP_H_V,
        }
    }
}

/// Blend modes (Porter-Duff)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Source over destination (default)
    SrcOver,
    /// Source only
    Src,
    /// Destination only
    Dst,
    /// Source in destination
    SrcIn,
    /// Destination in source
    DstIn,
    /// Source out of destination
    SrcOut,
    /// Destination out of source
    DstOut,
    /// Destination over source
    DstOver,
    /// Source atop destination
    SrcAtop,
    /// Destination atop source
    DstAtop,
    /// XOR (exclusive or)
    Xor,
}

impl BlendMode {
    /// Convert to usage flag
    pub fn to_usage(self) -> Usage {
        match self {
            Self::SrcOver => Usage::ALPHA_BLEND_SRC_OVER,
            Self::Src => Usage::ALPHA_BLEND_SRC,
            Self::Dst => Usage::ALPHA_BLEND_DST,
            Self::SrcIn => Usage::ALPHA_BLEND_SRC_IN,
            Self::DstIn => Usage::ALPHA_BLEND_DST_IN,
            Self::SrcOut => Usage::ALPHA_BLEND_SRC_OUT,
            Self::DstOut => Usage::ALPHA_BLEND_DST_OUT,
            Self::DstOver => Usage::ALPHA_BLEND_DST_OVER,
            Self::SrcAtop => Usage::ALPHA_BLEND_SRC_ATOP,
            Self::DstAtop => Usage::ALPHA_BLEND_DST_ATOP,
            Self::Xor => Usage::ALPHA_BLEND_XOR,
        }
    }
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::SrcOver
    }
}

/// Mosaic block sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosaicMode {
    /// 8x8 pixel blocks
    Block8 = 0,
    /// 16x16 pixel blocks
    Block16 = 1,
    /// 32x32 pixel blocks
    Block32 = 2,
    /// 64x64 pixel blocks
    Block64 = 3,
    /// 128x128 pixel blocks
    Block128 = 4,
}

impl MosaicMode {
    /// Get block size in pixels
    pub fn block_size(self) -> i32 {
        match self {
            Self::Block8 => 8,
            Self::Block16 => 16,
            Self::Block32 => 32,
            Self::Block64 => 64,
            Self::Block128 => 128,
        }
    }
}

impl Default for MosaicMode {
    fn default() -> Self {
        Self::Block16
    }
}

/// Interpolation modes for scaling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpMode {
    /// Default interpolation (usually linear)
    Default = 0,
    /// Linear interpolation
    Linear = 1,
    /// Cubic interpolation
    Cubic = 2,
    /// Average interpolation
    Average = 3,
}

impl Default for InterpMode {
    fn default() -> Self {
        Self::Default
    }
}

/// Color space conversion modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// ROP (Raster Operation) codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RopCode {
    /// AND operation
    And = 0x88,
    /// OR operation
    Or = 0xee,
    /// NOT destination
    NotDst = 0x55,
    /// NOT source
    NotSrc = 0x33,
    /// XOR operation
    Xor = 0xf6,
    /// NOT XOR operation
    NotXor = 0xf9,
}

/// Read modes for buffer access
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode {
    /// Raster mode (default)
    Raster = 1 << 0,
    /// AFBC 16x16 compressed format
    Afbc16x16 = 1 << 1,
    /// Tile 8x8 format
    Tile8x8 = 1 << 2,
    /// Tile 4x4 format
    Tile4x4 = 1 << 3,
    /// RKFBC 64x4 compressed format
    Rkfbc64x4 = 1 << 4,
    /// AFBC 32x8 compressed format
    Afbc32x8 = 1 << 5,
}

/// RGA scheduler core selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerCore {
    /// RGA3 Core 0
    Rga3Core0 = 1 << 0,
    /// RGA3 Core 1
    Rga3Core1 = 1 << 1,
    /// RGA2 Core 0
    Rga2Core0 = 1 << 2,
    /// RGA2 Core 1
    Rga2Core1 = 1 << 3,
    /// Automatic selection
    Default = 0,
}

impl SchedulerCore {
    /// Default RGA3 core
    pub const Rga3Default: Self = Self::Rga3Core0;
    /// Default RGA2 core
    pub const Rga2Default: Self = Self::Rga2Core0;
}
