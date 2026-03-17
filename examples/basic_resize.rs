//! Basic resize example for librga
//!
//! This example demonstrates basic image resizing using the RGA hardware.
//!
//! Note: On real RK3566 hardware, you would typically use DRM/dma_buf to get
//! proper DMA buffers. This example uses virtual address which may work for
//! testing but is not optimal for production use.

use librga::{ops::resize::ResizeOptions, query, resize, PixelFormat, RgaBuffer};
use std::ffi::c_void;

fn alloc_buffer(size: usize) -> Option<(*mut c_void, usize)> {
    let ptr = unsafe { libc::malloc(size) };
    if ptr.is_null() {
        None
    } else {
        Some((ptr, size))
    }
}

fn main() {
    println!("=== librga Basic Resize Example ===\n");

    // Print RGA information
    println!("RGA Information:");
    println!("  Vendor: {}", query::query_vendor().trim());
    println!("  Version: {}", query::query_version().trim());
    println!();

    // Dimensions
    let src_width = 1920;
    let src_height = 1080;
    let dst_width = 1280;
    let dst_height = 720;
    let format = PixelFormat::Rgba8888;

    println!("Source: {}x{} {:?}", src_width, src_height, format);
    println!("Destination: {}x{} {:?}\n", dst_width, dst_height, format);

    // Allocate source buffer (1920x1080 RGBA)
    let src_size = (src_width * src_height * 4) as usize;
    let (src_ptr, _) = match alloc_buffer(src_size) {
        Some(v) => v,
        None => {
            println!("Failed to allocate source buffer");
            return;
        }
    };

    // Allocate destination buffer (1280x720 RGBA)
    let dst_size = (dst_width * dst_height * 4) as usize;
    let (dst_ptr, _) = match alloc_buffer(dst_size) {
        Some(v) => v,
        None => {
            println!("Failed to allocate destination buffer");
            unsafe { libc::free(src_ptr) };
            return;
        }
    };

    println!("Source buffer: {:?} ({} bytes)", src_ptr, src_size);
    println!("Dest buffer: {:?} ({} bytes)\n", dst_ptr, dst_size);

    // Create RGA buffers
    let src = match unsafe {
        RgaBuffer::from_virtual_addr_unchecked(src_ptr, src_width, src_height, format)
    } {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to create source RgaBuffer: {:?}", e);
            unsafe {
                libc::free(src_ptr);
                libc::free(dst_ptr);
            }
            return;
        }
    };

    let mut dst = match unsafe {
        RgaBuffer::from_virtual_addr_unchecked(dst_ptr, dst_width, dst_height, format)
    } {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to create destination RgaBuffer: {:?}", e);
            unsafe {
                libc::free(src_ptr);
                libc::free(dst_ptr);
            }
            return;
        }
    };

    println!("RgaBuffer created successfully\n");

    // Perform resize
    println!("Performing resize...");

    // Test different resize options
    println!("\n1. Using scale ratio (1280x720):");
    match resize(
        &src,
        &mut dst,
        ResizeOptions::with_scale(1280.0 / 1920.0, 720.0 / 1080.0),
    ) {
        Ok(_) => println!("   SUCCESS!"),
        Err(e) => println!("   FAILED: {:?}", e),
    }

    // Test different interpolation modes
    println!("\n2. Using different interpolation modes:");

    // Linear interpolation
    match resize(
        &src,
        &mut dst,
        ResizeOptions::with_interpolation(librga::InterpMode::Linear),
    ) {
        Ok(_) => println!("   Linear: SUCCESS!"),
        Err(e) => println!("   Linear: FAILED - {:?}", e),
    }

    // Cubic interpolation
    match resize(
        &src,
        &mut dst,
        ResizeOptions::with_interpolation(librga::InterpMode::Cubic),
    ) {
        Ok(_) => println!("   Cubic: SUCCESS!"),
        Err(e) => println!("   Cubic: FAILED - {:?}", e),
    }

    // Average (for downscaling)
    match resize(
        &src,
        &mut dst,
        ResizeOptions::with_interpolation(librga::InterpMode::Average),
    ) {
        Ok(_) => println!("   Average: SUCCESS!"),
        Err(e) => println!("   Average: FAILED - {:?}", e),
    }

    // Cleanup
    unsafe {
        libc::free(src_ptr);
        libc::free(dst_ptr);
    }

    println!("\n=== Resize Complete ===");
}
