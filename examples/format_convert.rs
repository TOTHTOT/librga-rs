//! Format conversion example for librga
//!
//! This example demonstrates color format conversion using RGA hardware.

use librga::query;

fn main() -> anyhow::Result<()> {
    println!("=== librga Format Conversion Example ===\n");

    // Print RGA information
    println!("RGA Information:");
    println!("  Vendor: {}", query::query_vendor());
    println!("  Version: {}", query::query_version());
    println!();

    // Common format conversions
    println!("Common Format Conversions:");
    println!();

    println!("1. NV12 to RGBA (Video decode output to display):");
    println!("   convert_color(");
    println!("       &src, &mut dst,");
    println!("       PixelFormat::YCbCr420SP,");
    println!("       PixelFormat::Rgba8888,");
    println!("       ColorSpaceMode::YuvToRgbBt601Limit,");
    println!("       OpOptions::default()");
    println!("   )?;");
    println!();

    println!("2. RGBA to NV12 (Display capture to video encode):");
    println!("   convert_color(");
    println!("       &src, &mut dst,");
    println!("       PixelFormat::Rgba8888,");
    println!("       PixelFormat::YCbCr420SP,");
    println!("       ColorSpaceMode::RgbToYuvBt601Limit,");
    println!("       OpOptions::default()");
    println!("   )?;");
    println!();

    println!("3. RGB565 to RGBA (Legacy format conversion):");
    println!("   convert_color(");
    println!("       &src, &mut dst,");
    println!("       PixelFormat::Rgb565,");
    println!("       PixelFormat::Rgba8888,");
    println!("       ColorSpaceMode::Default,");
    println!("       OpOptions::default()");
    println!("   )?;");
    println!();

    // Color space modes
    println!("Available Color Space Modes:");
    println!("  - YuvToRgbBt601Limit - YUV to RGB using BT.601 limited range");
    println!("  - YuvToRgbBt601Full - YUV to RGB using BT.601 full range");
    println!("  - YuvToRgbBt709Limit - YUV to RGB using BT.709 limited range");
    println!("  - RgbToYuvBt601Full - RGB to YUV using BT.601 full range");
    println!("  - RgbToYuvBt601Limit - RGB to YUV using BT.601 limited range");
    println!("  - RgbToYuvBt709Limit - RGB to YUV using BT.709 limited range");
    println!();

    // Supported formats
    println!("Supported Pixel Formats:");
    println!("  RGB Formats:");
    println!("    - RGBA8888, RGBX8888, RGB888");
    println!("    - BGRA8888, BGR888, BGRX8888");
    println!("    - RGB565, BGR565");
    println!("    - RGBA5551, BGRA5551");
    println!("    - RGBA4444, BGRA4444");
    println!("  YUV Formats:");
    println!("    - NV12 (YCbCr420SP), NV21 (YCrCb420SP)");
    println!("    - I420 (YCbCr420P), YV12 (YCrCb420P)");
    println!("    - YUYV, UYVY, YVYU, VYUY");
    println!();

    Ok(())
}
