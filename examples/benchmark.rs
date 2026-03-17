//! Benchmark example for librga
//!
//! This example benchmarks various RGA operations and reports timing.

use librga::{
    copy, flip,
    ops::resize::ResizeOptions,
    query, resize, rotate,
    usage::{FlipMode, Rotation},
    PixelFormat, RgaBuffer,
};
use std::ffi::c_void;
use std::time::Instant;

fn alloc_buffer(size: usize) -> Option<(*mut c_void, usize)> {
    let ptr = unsafe { libc::malloc(size) };
    if ptr.is_null() {
        None
    } else {
        Some((ptr, size))
    }
}

fn main() {
    println!("=== librga Performance Benchmark ===\n");

    // Query RGA info
    println!("RGA Version: {}", query::query_version().trim());
    println!("RGA Vendor: {}\n", query::query_vendor().trim());

    // Buffer dimensions
    let width: i32 = 1920;
    let height: i32 = 1080;
    let size = (width * height * 4) as usize; // RGBA

    println!("Test configuration: {}x{} RGBA\n", width, height);

    // Allocate buffers
    let (src_ptr, _) = alloc_buffer(size).expect("Failed to allocate src buffer");
    let (dst_ptr, _) = alloc_buffer(size).expect("Failed to allocate dst buffer");

    // Create RGA buffers
    let src = unsafe {
        RgaBuffer::from_virtual_addr_unchecked(src_ptr, width, height, PixelFormat::Rgba8888)
    }
    .expect("Failed to create src buffer");

    let mut dst = unsafe {
        RgaBuffer::from_virtual_addr_unchecked(dst_ptr, width, height, PixelFormat::Rgba8888)
    }
    .expect("Failed to create dst buffer");

    // Small buffer for resize
    let small_size = (1280 * 720 * 4) as usize;
    let (dst2_ptr, _) = alloc_buffer(small_size).expect("Failed to allocate dst2 buffer");
    let mut dst2 = unsafe {
        RgaBuffer::from_virtual_addr_unchecked(dst2_ptr, 1280, 720, PixelFormat::Rgba8888)
    }
    .expect("Failed to create dst2 buffer");

    let iterations = 100;

    // Benchmark copy
    println!("Benchmarking copy ({} iterations)...", iterations);
    let start = Instant::now();
    for _ in 0..iterations {
        copy(&src, &mut dst, true).expect("copy failed");
    }
    let copy_time = start.elapsed();
    println!(
        "  Total: {:?} ({:.3}ms per operation)",
        copy_time,
        copy_time.as_millis() as f64 / iterations as f64
    );

    // Benchmark resize
    println!(
        "\nBenchmarking resize 1920x1080 -> 1280x720 ({} iterations)...",
        iterations
    );
    let start = Instant::now();
    for _ in 0..iterations {
        resize(&src, &mut dst2, ResizeOptions::default()).expect("resize failed");
    }
    let resize_time = start.elapsed();
    println!(
        "  Total: {:?} ({:.3}ms per operation)",
        resize_time,
        resize_time.as_millis() as f64 / iterations as f64
    );

    // Benchmark rotate
    println!("\nBenchmarking rotate 90 ({} iterations)...", iterations);
    let start = Instant::now();
    for _ in 0..iterations {
        rotate(&src, &mut dst, Rotation::Rot90, true).expect("rotate failed");
    }
    let rotate_time = start.elapsed();
    println!(
        "  Total: {:?} ({:.3}ms per operation)",
        rotate_time,
        rotate_time.as_millis() as f64 / iterations as f64
    );

    // Benchmark flip
    println!(
        "\nBenchmarking flip horizontal ({} iterations)...",
        iterations
    );
    let start = Instant::now();
    for _ in 0..iterations {
        flip(&src, &mut dst, FlipMode::Horizontal, true).expect("flip failed");
    }
    let flip_time = start.elapsed();
    println!(
        "  Total: {:?} ({:.3}ms per operation)",
        flip_time,
        flip_time.as_millis() as f64 / iterations as f64
    );

    // Combined operation benchmark
    println!(
        "\nBenchmarking combined operations (copy -> resize -> rotate) ({} iterations)...",
        iterations
    );
    let start = Instant::now();
    for _ in 0..iterations {
        copy(&src, &mut dst, true).expect("copy failed");
        resize(&src, &mut dst2, ResizeOptions::default()).expect("resize failed");
        rotate(&src, &mut dst, Rotation::Rot90, true).expect("rotate failed");
    }
    let combined_time = start.elapsed();
    println!(
        "  Total: {:?} ({:.3}ms per iteration)",
        combined_time,
        combined_time.as_millis() as f64 / iterations as f64
    );

    println!("\n=== Benchmark Complete ===");

    // Cleanup
    unsafe {
        libc::free(src_ptr);
        libc::free(dst_ptr);
        libc::free(dst2_ptr);
    }
}
