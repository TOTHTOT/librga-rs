//! Image resize example for librga
//!
//! This example demonstrates image resizing using the RGA hardware with actual image files.
//!
//! Usage:
//!   1. Place an input image as "input.jpg" in the current directory
//!   2. Run the example: cargo run --example image_resize
//!   3. Output will be saved as "output.jpg"
//!
//! Note: On real RK3566 hardware, you would typically use DRM/dma_buf to get
//! proper DMA buffers. This example uses virtual address which may work for
//! testing but is not optimal for production use.

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use librga::{ops::resize::ResizeOptions, query, resize, PixelFormat, RgaBuffer};
use std::time::Instant;

fn main() {
    println!("=== librga Image Resize Example ===\n");

    // Print RGA information
    println!("RGA Information:");
    println!("  Vendor: {}", query::query_vendor().trim());
    println!("  Version: {}", query::query_version().trim());
    println!();

    // 1. Load input image
    let input_path = "test.jpg";
    println!("Loading input image: {}", input_path);

    let img = match image::open(input_path) {
        Ok(i) => i,
        Err(e) => {
            println!("Failed to open test.jpg: {}", e);
            println!("\nUsage: Place an image named 'test.jpg' in the current directory");
            return;
        }
    };

    let (src_width, src_height) = (img.width() as i32, img.height() as i32);
    println!("Input: {}x{}", src_width, src_height);

    // 2. Convert to RGBA format and get pixel data
    let rgba = img.to_rgba8();
    let src_data = rgba.into_raw();

    // 3. Create source RgaBuffer (safe API)
    // Note: src_data is kept alive by being in the tuple with the buffer
    let (src_buf, _src_data) =
        match RgaBuffer::from_vec(src_data, src_width, src_height, PixelFormat::Rgba8888) {
            Ok(b) => b,
            Err(e) => {
                println!("Failed to create src buffer: {:?}", e);
                return;
            }
        };

    // Keep src_data alive - it will be dropped after src_buf

    // 4. Calculate target dimensions (resize to 320 width)
    let dst_width = 320;
    let dst_height = (src_height as f64 * 320.0 / src_width as f64) as i32;
    let dst_size = (dst_width * dst_height * 4) as usize;

    println!("Target: {}x{}", dst_width, dst_height);

    // 5. Allocate destination memory and create buffer (safe API)
    let dst_data = vec![0u8; dst_size];
    let (mut dst_buf, dst_data) =
        match RgaBuffer::from_vec_mut(dst_data, dst_width, dst_height, PixelFormat::Rgba8888) {
            Ok(b) => b,
            Err(e) => {
                println!("Failed to create dst buffer: {:?}", e);
                return;
            }
        };

    // 6. Execute RGA resize with timing
    println!("Performing RGA resize...");

    let scale_x = 320.0 / src_width as f64;
    let scale_y = dst_height as f64 / src_height as f64;

    let start = Instant::now();

    match resize(
        &src_buf,
        &mut dst_buf,
        ResizeOptions::with_scale(scale_x, scale_y),
    ) {
        Ok(_) => {
            let elapsed = start.elapsed();
            println!("Resize successful!");
            println!("Time: {:?}", elapsed);
        }
        Err(e) => {
            println!("Resize failed: {:?}", e);
            return;
        }
    }

    // Run multiple times for benchmark
    println!("\nRunning 10 times for benchmark...");
    let mut total_time = std::time::Duration::ZERO;
    for i in 0..10 {
        let start = Instant::now();
        resize(
            &src_buf,
            &mut dst_buf,
            ResizeOptions::with_scale(scale_x, scale_y),
        )
        .expect("Resize failed");
        let elapsed = start.elapsed();
        total_time += elapsed;
        println!("  Run {}: {:?}", i + 1, elapsed);
    }
    println!("Average: {:?}", total_time / 10);

    // 7. Save result as image
    let result =
        match ImageBuffer::<Rgba<u8>, _>::from_raw(dst_width as u32, dst_height as u32, dst_data) {
            Some(r) => r,
            None => {
                println!("Failed to create image from buffer");
                return;
            }
        };

    let result_img = DynamicImage::ImageRgba8(result);
    let output_path = "output.jpg";

    match result_img.save_with_format(output_path, ImageFormat::Jpeg) {
        Ok(_) => println!("Output saved to: {}", output_path),
        Err(e) => {
            println!("Failed to save output.jpg: {}", e);
            return;
        }
    }

    println!("\n=== Done ===");
}
