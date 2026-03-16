//! Alpha blending example for librga
//!
//! This example demonstrates alpha blending operations.

use librga::{
    blend, composite, BlendMode, OpOptions, RgaBuffer,
};

fn main() -> anyhow::Result<()> {
    println!("=== librga Alpha Blending Example ===\n");

    println!("Blend Modes (Porter-Duff):");
    println!();

    println!("1. SrcOver (default) - Source over destination:");
    println!("   blend(&foreground, &mut background, BlendMode::SrcOver, OpOptions::default())?;");
    println!();

    println!("2. Src - Source only (replace destination):");
    println!("   blend(&foreground, &mut background, BlendMode::Src, OpOptions::default())?;");
    println!();

    println!("3. Dst - Destination only (keep destination):");
    println!("   blend(&foreground, &mut background, BlendMode::Dst, OpOptions::default())?;");
    println!();

    println!("4. Composite two images to third destination:");
    println!("   composite(&image_a, &image_b, &mut output, BlendMode::SrcOver, OpOptions::default())?;");
    println!();

    // Blend mode descriptions
    println!("Available Blend Modes:");
    println!("  - SrcOver: Default blending, source over destination");
    println!("  - Src: Use source only");
    println!("  - Dst: Use destination only");
    println!("  - SrcIn: Source inside destination alpha");
    println!("  - DstIn: Destination inside source alpha");
    println!("  - SrcOut: Source outside destination alpha");
    println!("  - DstOut: Destination outside source alpha");
    println!("  - DstOver: Destination over source");
    println!("  - SrcAtop: Source atop destination");
    println!("  - DstAtop: Destination atop source");
    println!("  - Xor: XOR of source and destination");
    println!();

    // Color key example
    println!("Color Key (Chroma Key):");
    println!("  Remove background color from foreground image:");
    println!();
    println!("  use librga::ops::blend::color_key;");
    println!("  color_key(");
    println!("      &foreground,");
    println!("      &mut output,");
    println!("      0x00FF00, // Green color key");
    println!("      0x00FF00, // Range");
    println!("      false,    // Not inverted");
    println!("      OpOptions::default()");
    println!("  )?;");
    println!();

    Ok(())
}
