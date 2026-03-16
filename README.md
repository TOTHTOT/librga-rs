# librga-rs

Rust FFI bindings for Rockchip RGA (Rockchip Graphics Accelerator) - hardware-accelerated 2D graphics operations for Rockchip SoCs like RK3566, RK3588.

## Features

- **Image Scaling**: High-quality resize with bilinear/cubic interpolation
- **Color Conversion**: RGB/YUV format conversion with BT.601/BT.709 color spaces
- **Transformations**: Rotation (90/180/270) and flipping
- **Alpha Blending**: Porter-Duff blend modes
- **Drawing**: Fill, rectangle drawing
- **Effects**: Gaussian blur, mosaic, palette mapping
- **Batch Processing**: Job-based API for multiple operations

## Supported Platforms

- RK3566 / RK3568
- RK3588 / RK3588S
- RV1106 / RV1103
- Other Rockchip SoCs with RGA hardware

## Build Requirements

- Rust toolchain (stable)
- `cross` tool for cross-compilation
- Docker (for cross-compilation environment)

## Building

### Install cross

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

### Build for RK3566 (aarch64)

```bash
cross build --release --target aarch64-unknown-linux-gnu
```

### Build examples

```bash
cross build --release --target aarch64-unknown-linux-gnu --examples
```

## Deployment

Use the included deploy script to deploy to your RK3566 device:

```bash
./deploy.sh
```

This will:
1. Build the release binaries
2. Copy them to the device (radxa@192.168.2.202)
3. Set up the library path

## Usage

### Basic Resize Example

```rust
use librga::{
    RgaBuffer, resize, ResizeOptions, OpOptions, PixelFormat,
};

// Create buffers from DMA file descriptors
let src = unsafe {
    RgaBuffer::from_fd_unchecked(src_fd, 1920, 1080, PixelFormat::Rgba8888)?
};
let mut dst = unsafe {
    RgaBuffer::from_fd_unchecked(dst_fd, 1280, 720, PixelFormat::Rgba8888)?
};

// Perform resize
resize(
    &src,
    &mut dst,
    ResizeOptions::with_scale(0.5, 0.5)
)?;
```

### Format Conversion

```rust
use librga::{
    convert_color, ColorSpaceMode, OpOptions, PixelFormat,
};

// Convert NV12 to RGBA
convert_color(
    &src,
    &mut dst,
    PixelFormat::YCbCr420SP,
    PixelFormat::Rgba8888,
    ColorSpaceMode::YuvToRgbBt601Limit,
    OpOptions::default()
)?;
```

### Batch Job Processing

```rust
use librga::Job;

let job = Job::new()?;

// Add multiple operations
job.add_resize(&src, &mut temp,
    ResizeOptions::with_scale(0.5, 0.5)
)?;
job.add_rotate(&temp, &mut dst, Rotation::Rot90)?;

// Submit all operations at once
job.submit(true)?;
```

## API Modules

- `buffer`: Buffer management (`RgaBuffer`)
- `format`: Pixel format definitions
- `rect`: Rectangle types
- `usage`: Usage flags and enums
- `error`: Error types
- `query`: RGA information queries
- `job`: Batch job processing
- `ops`: Operation functions
  - `copy`: Image copy
  - `resize`: Scaling
  - `crop`: Cropping
  - `color`: Format conversion
  - `transform`: Rotation/flip
  - `blend`: Alpha blending
  - `fill`: Fill/drawing
  - `effect`: Blur/mosaic/palette
  - `process`: General process

## Running Tests

On the target device:

```bash
cd ~/rga-rs
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
./test_runner
```

## Performance

RGA operations are hardware-accelerated and typically achieve:

- **Copy**: ~2-4 GB/s memory bandwidth
- **Resize 1080p→720p**: ~0.5-1 ms
- **NV12→RGBA conversion**: ~1-2 ms for 1080p
- **Rotation 90°**: ~1-2 ms for 1080p

Actual performance depends on:
- Memory type (system vs CMA)
- Buffer dimensions and alignment
- Operation complexity

## License

Apache-2.0 - See LICENSE file

## References

- [Rockchip RGA Developer Guide](docs/Rockchip_Developer_Guide_RGA_EN.md)
- [Rockchip RGA FAQ](docs/Rockchip_FAQ_RGA_EN.md)
