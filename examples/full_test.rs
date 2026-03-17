//! Full test for librga operations

use librga::{
    copy, fill, flip,
    ops::resize::ResizeOptions,
    query, resize, rotate,
    usage::{FlipMode, Rotation},
    PixelFormat, Rect, RgaBuffer,
};
use std::ffi::c_void;

// Simple DMA buffer allocation using malloc
fn alloc_buffer(size: usize) -> Option<(*mut c_void, usize)> {
    let ptr = unsafe { libc::malloc(size) };
    if ptr.is_null() {
        None
    } else {
        Some((ptr, size))
    }
}

fn test_copy(src: &RgaBuffer, dst: &mut RgaBuffer) {
    print!("copy... ");
    match copy(src, dst, true) {
        Ok(_) => println!("OK"),
        Err(e) => println!("FAILED: {:?}", e),
    }
}

fn test_resize(src: &RgaBuffer, dst: &mut RgaBuffer) {
    print!("resize(1920x1080 -> 1280x720)... ");
    match resize(
        src,
        dst,
        ResizeOptions::with_scale(1280.0 / 1920.0, 720.0 / 1080.0),
    ) {
        Ok(_) => println!("OK"),
        Err(e) => println!("FAILED: {:?}", e),
    }
}

fn test_rotate(src: &RgaBuffer, dst: &mut RgaBuffer) {
    print!("rotate(90)... ");
    match rotate(src, dst, Rotation::Rot90, true) {
        Ok(_) => println!("OK"),
        Err(e) => println!("FAILED: {:?}", e),
    }
}

fn test_flip(src: &RgaBuffer, dst: &mut RgaBuffer) {
    print!("flip(H)... ");
    match flip(src, dst, FlipMode::Horizontal, true) {
        Ok(_) => println!("OK"),
        Err(e) => println!("FAILED: {:?}", e),
    }
}

fn test_fill(dst: &mut RgaBuffer) {
    print!("fill(red rect)... ");
    let rect = Rect::at_origin(100, 100);
    match fill(dst, rect, 0xFF0000FF, true) {
        Ok(_) => println!("OK"),
        Err(e) => println!("FAILED: {:?}", e),
    }
}

fn main() {
    println!("=== librga Full Test ===\n");

    // Query RGA info
    println!("RGA Version: {}", query::query_version().trim());
    println!("RGA Vendor: {}\n", query::query_vendor().trim());

    // Buffer dimensions
    let width: i32 = 1920;
    let height: i32 = 1080;
    let size = (width * height * 4) as usize; // RGBA

    println!("Allocating buffers ({}x{})...", width, height);

    // Allocate source buffer
    let (src_ptr, src_size) = match alloc_buffer(size) {
        Some(v) => v,
        None => {
            println!("Failed to allocate source buffer");
            return;
        }
    };

    // Allocate destination buffer
    let (dst_ptr, dst_size) = match alloc_buffer(size) {
        Some(v) => v,
        None => {
            println!("Failed to allocate destination buffer");
            unsafe { libc::free(src_ptr) };
            return;
        }
    };

    println!("Source ptr: {:?}, size: {}", src_ptr, src_size);
    println!("Dest ptr: {:?}, size: {}\n", dst_ptr, dst_size);

    // Create RGA buffers from virtual address
    let src = match unsafe {
        RgaBuffer::from_virtual_addr_unchecked(src_ptr, width, height, PixelFormat::Rgba8888)
    } {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to create src RgaBuffer: {:?}", e);
            unsafe {
                libc::free(src_ptr);
                libc::free(dst_ptr);
            }
            return;
        }
    };

    let mut dst = match unsafe {
        RgaBuffer::from_virtual_addr_unchecked(dst_ptr, width, height, PixelFormat::Rgba8888)
    } {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to create dst RgaBuffer: {:?}", e);
            unsafe {
                libc::free(src_ptr);
                libc::free(dst_ptr);
            }
            return;
        }
    };

    println!("RgaBuffer created successfully\n");

    // Run tests
    println!("Running tests:\n");

    test_copy(&src, &mut dst);

    // For resize, we need a smaller destination
    let small_size = (1280 * 720 * 4) as usize;
    let (dst2_ptr, _) = match alloc_buffer(small_size) {
        Some(v) => v,
        None => {
            println!("Failed to allocate dst2 buffer");
            unsafe {
                libc::free(src_ptr);
                libc::free(dst_ptr);
            }
            return;
        }
    };

    let mut dst2 = match unsafe {
        RgaBuffer::from_virtual_addr_unchecked(dst2_ptr, 1280, 720, PixelFormat::Rgba8888)
    } {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to create dst2 RgaBuffer: {:?}", e);
            unsafe {
                libc::free(src_ptr);
                libc::free(dst_ptr);
                libc::free(dst2_ptr);
            }
            return;
        }
    };

    test_resize(&src, &mut dst2);
    test_rotate(&src, &mut dst);
    test_flip(&src, &mut dst);
    test_fill(&mut dst);

    // Cleanup
    unsafe {
        libc::free(src_ptr);
        libc::free(dst_ptr);
        libc::free(dst2_ptr);
    }

    println!("\n=== Test Complete ===");
}
