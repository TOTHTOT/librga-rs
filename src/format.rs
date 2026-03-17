//! Pixel format definitions for RGA

/// Pixel formats supported by RGA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[derive(Default)]
pub enum PixelFormat {
    /// RGBA 8888 (32-bit, 8 bits per channel)
    #[default]
    Rgba8888 = 0x0, // RK_FORMAT_RGBA_8888

    /// RGBX 8888 (32-bit, no alpha)
    Rgbx8888 = 0x1, // RK_FORMAT_RGBX_8888

    /// RGB 888 (24-bit)
    Rgb888 = 0x2, // RK_FORMAT_RGB_888

    /// BGRA 8888 (32-bit)
    Bgra8888 = 0x3, // RK_FORMAT_BGRA_8888

    /// BGR 888 (24-bit)
    Bgr888 = 0x4, // RK_FORMAT_BGR_888

    /// BGRX 8888 (32-bit, no alpha)
    Bgrx8888 = 0x5, // RK_FORMAT_BGRX_8888

    /// RGB 565 (16-bit)
    Rgb565 = 0x6, // RK_FORMAT_RGB_565

    /// BGR 565 (16-bit)
    Bgr565 = 0x7, // RK_FORMAT_BGR_565

    /// RGBA 5551 (16-bit, 1-bit alpha)
    Rgba5551 = 0x8, // RK_FORMAT_RGBA_5551

    /// BGRA 5551 (16-bit, 1-bit alpha)
    Bgra5551 = 0x9, // RK_FORMAT_BGRA_5551

    /// RGBA 4444 (16-bit, 4-bit per channel)
    Rgba4444 = 0xa, // RK_FORMAT_RGBA_4444

    /// BGRA 4444 (16-bit, 4-bit per channel)
    Bgra4444 = 0xb, // RK_FORMAT_BGRA_4444

    /// ARGB 8888 (32-bit)
    Argb8888 = 0xc, // RK_FORMAT_ARGB_8888

    /// XRGB 8888 (32-bit, no alpha)
    Xrgb8888 = 0xd, // RK_FORMAT_XRGB_8888

    /// ABGR 8888 (32-bit)
    Abgr8888 = 0xe, // RK_FORMAT_ABGR_8888

    /// XBGR 8888 (32-bit, no alpha)
    Xbgr8888 = 0xf, // RK_FORMAT_XBGR_8888

    /// YCbCr 422 SP (NV16)
    YCbCr422SP = 0x10, // RK_FORMAT_YCbCr_422_SP

    /// YCbCr 420 SP (NV12)
    YCbCr420SP = 0x11, // RK_FORMAT_YCbCr_420_SP

    /// YCrCb 422 SP (NV61)
    YCrCb422SP = 0x12, // RK_FORMAT_YCrCb_422_SP

    /// YCrCb 420 SP (NV21)
    YCrCb420SP = 0x13, // RK_FORMAT_YCrCb_420_SP

    /// YCbCr 422 P (I422)
    YCbCr422P = 0x14, // RK_FORMAT_YCbCr_422_P

    /// YCbCr 420 P (I420/YUV420P)
    YCbCr420P = 0x15, // RK_FORMAT_YCbCr_420_P

    /// YCrCb 422 P
    YCrCb422P = 0x16, // RK_FORMAT_YCrCb_422_P

    /// YCrCb 420 P (YV12)
    YCrCb420P = 0x17, // RK_FORMAT_YCrCb_420_P

    /// YVYU 422
    Yvyu422 = 0x18, // RK_FORMAT_YVYU_422

    /// VYUY 422
    Vyuy422 = 0x19, // RK_FORMAT_VYUY_422

    /// YUYV 422
    Yuyv422 = 0x1a, // RK_FORMAT_YUYV_422

    /// UYVY 422
    Uyvy422 = 0x1b, // RK_FORMAT_UYVY_422

    /// YCbCr 420 SP 10-bit
    YCbCr420SP10B = 0x1c, // RK_FORMAT_YCbCr_420_SP_10B

    /// YCrCb 420 SP 10-bit
    YCrCb420SP10B = 0x1d, // RK_FORMAT_YCrCb_420_SP_10B

    /// YCbCr 422 SP 10-bit
    YCbCr422SP10B = 0x1e, // RK_FORMAT_YCbCr_422_SP_10B

    /// YCrCb 422 SP 10-bit
    YCrCb422SP10B = 0x1f, // RK_FORMAT_YCrCb_422_SP_10B

    /// BPP 2 (2-bit per pixel)
    Bpp2 = 0x20, // RK_FORMAT_RGBA2BPP

    /// Alpha 8 (8-bit alpha only)
    A8 = 0x21, // RK_FORMAT_A8

    /// Y4 (4-bit grayscale)
    Y4 = 0x22, // RK_FORMAT_Y4

    /// YCbCr 400 (8-bit grayscale)
    YCbCr400 = 0x23, // RK_FORMAT_YCbCr_400
}

impl PixelFormat {
    /// Get bits per pixel for this format
    pub fn bits_per_pixel(&self) -> i32 {
        match self {
            Self::Rgba8888
            | Self::Rgbx8888
            | Self::Bgra8888
            | Self::Bgrx8888
            | Self::Argb8888
            | Self::Xrgb8888
            | Self::Abgr8888
            | Self::Xbgr8888 => 32,

            Self::Rgb888 | Self::Bgr888 => 24,

            Self::Rgb565
            | Self::Bgr565
            | Self::Rgba5551
            | Self::Bgra5551
            | Self::Rgba4444
            | Self::Bgra4444
            | Self::Yvyu422
            | Self::Vyuy422
            | Self::Yuyv422
            | Self::Uyvy422 => 16,

            Self::Bpp2 => 2,
            Self::A8 | Self::Y4 | Self::YCbCr400 => 8,

            // YUV 4:2:0 formats: 12 bits per pixel average
            Self::YCbCr420SP
            | Self::YCrCb420SP
            | Self::YCbCr420P
            | Self::YCrCb420P
            | Self::YCbCr420SP10B
            | Self::YCrCb420SP10B => 12,

            // YUV 4:2:2 formats: 16 bits per pixel average
            Self::YCbCr422SP
            | Self::YCrCb422SP
            | Self::YCbCr422P
            | Self::YCrCb422P
            | Self::YCbCr422SP10B
            | Self::YCrCb422SP10B => 16,
        }
    }

    /// Get bytes per pixel (rounded up)
    pub fn bytes_per_pixel(&self) -> i32 {
        (self.bits_per_pixel() + 7) / 8
    }

    /// Check if this is an RGB format
    pub fn is_rgb(&self) -> bool {
        matches!(
            self,
            Self::Rgba8888
                | Self::Rgbx8888
                | Self::Rgb888
                | Self::Bgra8888
                | Self::Bgr888
                | Self::Bgrx8888
                | Self::Rgb565
                | Self::Bgr565
                | Self::Rgba5551
                | Self::Bgra5551
                | Self::Rgba4444
                | Self::Bgra4444
                | Self::Argb8888
                | Self::Xrgb8888
                | Self::Abgr8888
                | Self::Xbgr8888
        )
    }

    /// Check if this is a YUV format
    pub fn is_yuv(&self) -> bool {
        matches!(
            self,
            Self::YCbCr422SP
                | Self::YCbCr420SP
                | Self::YCrCb422SP
                | Self::YCrCb420SP
                | Self::YCbCr422P
                | Self::YCbCr420P
                | Self::YCrCb422P
                | Self::YCrCb420P
                | Self::Yvyu422
                | Self::Vyuy422
                | Self::Yuyv422
                | Self::Uyvy422
                | Self::YCbCr420SP10B
                | Self::YCrCb420SP10B
                | Self::YCbCr422SP10B
                | Self::YCrCb422SP10B
                | Self::YCbCr400
        )
    }

    /// Check if this format has alpha channel
    pub fn has_alpha(&self) -> bool {
        matches!(
            self,
            Self::Rgba8888
                | Self::Bgra8888
                | Self::Rgba5551
                | Self::Bgra5551
                | Self::Rgba4444
                | Self::Bgra4444
                | Self::Argb8888
                | Self::Abgr8888
        )
    }
}
