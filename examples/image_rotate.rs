//! Image rotation example for librga
//!
//! This example demonstrates image rotation using the RGA hardware with actual image files.
//!
//! Usage:
//!   1. Place an input image as "test.jpg" in the current directory
//!   2. Run the example: cargo run --example image_rotate
//!   3. Output will be saved as "output_rotate_xxx.jpg"
//!
//! Note: On real RK3566 hardware, you would typically use DRM/dma_buf to get
//! proper DMA buffers. This example uses virtual address which may work for
//! testing but is not optimal for production use.

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use librga::usage::Rotation;
use librga::{query, rotate, PixelFormat, RgaBuffer};
use std::time::Instant;

/// Check if rotation swaps width and height dimensions
fn rotation_swaps_dimensions(rotation: Rotation) -> bool {
    matches!(rotation, Rotation::Rot90 | Rotation::Rot270)
}

/// Get rotation name for file naming
fn rotation_name(rotation: Rotation) -> &'static str {
    match rotation {
        Rotation::Rot90 => "90",
        Rotation::Rot180 => "180",
        Rotation::Rot270 => "270",
    }
}

fn main() {
    println!("=== librga Image Rotation Example ===\n");

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
    let (src_buf, _src_data) =
        match RgaBuffer::from_vec(src_data, src_width, src_height, PixelFormat::Rgba8888) {
            Ok(b) => b,
            Err(e) => {
                println!("Failed to create src buffer: {:?}", e);
                return;
            }
        };

    // Keep src_data alive - it will be dropped after src_buf

    // Test all rotation angles
    let rotations = [Rotation::Rot90, Rotation::Rot180, Rotation::Rot270];

    for rotation in &rotations {
        let rotation_deg = rotation_name(*rotation);
        println!("\n--- Testing Rotation {}° ---", rotation_deg);

        // 90°/270° rotations swap width and height
        let (dst_width, dst_height) = if rotation_swaps_dimensions(*rotation) {
            (src_height, src_width)
        } else {
            (src_width, src_height)
        };

        println!("Output dimensions: {}x{}", dst_width, dst_height);

        let dst_size = (dst_width * dst_height * 4) as usize;

        // 4. Allocate destination memory and create buffer (safe API)
        let dst_data = vec![0u8; dst_size];
        let (mut dst_buf, dst_data_owned) =
            match RgaBuffer::from_vec_mut(dst_data, dst_width, dst_height, PixelFormat::Rgba8888) {
                Ok(b) => b,
                Err(e) => {
                    println!("Failed to create dst buffer: {:?}", e);
                    return;
                }
            };

        // 5. Execute RGA rotation with timing
        println!("Performing RGA rotation...");

        let start = Instant::now();

        match rotate(&src_buf, &mut dst_buf, *rotation, true) {
            Ok(_) => {
                let elapsed = start.elapsed();
                println!("Rotation successful!");
                println!("Time: {:?}", elapsed);
            }
            Err(e) => {
                println!("Rotation failed: {:?}", e);
                return;
            }
        }

        // Run multiple times for benchmark
        println!("\nRunning 10 times for benchmark...");
        let mut total_time = std::time::Duration::ZERO;
        for i in 0..10 {
            let start = Instant::now();
            rotate(&src_buf, &mut dst_buf, *rotation, true).expect("Rotation failed");
            let elapsed = start.elapsed();
            total_time += elapsed;
            println!("  Run {}: {:?}", i + 1, elapsed);
        }
        println!("Average: {:?}", total_time / 10);

        // 6. Save result as image
        let result = match ImageBuffer::<Rgba<u8>, _>::from_raw(
            dst_width as u32,
            dst_height as u32,
            dst_data_owned,
        ) {
            Some(r) => r,
            None => {
                println!("Failed to create image from buffer");
                return;
            }
        };

        let result_img = DynamicImage::ImageRgba8(result);
        let output_path = format!("output_rotate_{}.jpg", rotation_deg);

        match result_img.save_with_format(&output_path, ImageFormat::Jpeg) {
            Ok(_) => println!("Output saved to: {}", output_path),
            Err(e) => {
                println!("Failed to save {}: {}", output_path, e);
                return;
            }
        }
    }

    println!("\n=== Done ===");
}
