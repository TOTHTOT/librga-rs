//! Basic resize example for librga
//!
//! This example demonstrates basic image resizing using the RGA hardware.

use librga::{
    query, RgaBuffer, PixelFormat,
};

fn main() -> anyhow::Result<()> {
    println!("=== librga Basic Resize Example ===\n");

    // Print RGA information
    println!("RGA Information:");
    println!("  Vendor: {}", query::query_vendor());
    println!("  Version: {}", query::query_version());
    println!("  Features: {}", query::query_features());
    println!();

    // Example dimensions
    let src_width = 1920;
    let src_height = 1080;
    let dst_width = 1280;
    let dst_height = 720;
    let format = PixelFormat::Rgba8888;

    println!("Source: {}x{} {:?}", src_width, src_height, format);
    println!("Destination: {}x{} {:?}", dst_width, dst_height, format);
    println!();

    // In a real application, you would allocate DMA buffers here
    // For this example, we just print what would happen
    println!("Note: This example requires actual DMA buffers.");
    println!("Usage with DMA buffers:");
    println!();
    println!("  // Allocate DMA buffers");
    println!("  let src_buf = alloc_dma_buffer(src_width * src_height * 4)?;");
    println!("  let dst_buf = alloc_dma_buffer(dst_width * dst_height * 4)?;");
    println!();
    println!("  // Create RGA buffers");
    println!("  let src = unsafe {{ RgaBuffer::from_fd_unchecked(src_fd, src_width, src_height, format)? }};");
    println!("  let mut dst = unsafe {{ RgaBuffer::from_fd_unchecked(dst_fd, dst_width, dst_height, format)? }};");
    println!();
    println!("  // Perform resize");
    println!("  use librga::resize;");
    println!("  resize(&src, &mut dst, ResizeOptions::default())?;");
    println!();

    // Show different resize options
    println!("Available Resize Options:");
    println!("  - ResizeOptions::default() - Automatic scaling");
    println!("  - ResizeOptions::with_scale(0.5, 0.5) - Downscale by 2x");
    println!("  - ResizeOptions::with_scale(2.0, 2.0) - Upscale by 2x");
    println!("  - ResizeOptions::with_interpolation(InterpMode::Linear) - Use linear interpolation");
    println!();

    Ok(())
}
