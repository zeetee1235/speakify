mod morph;

use anyhow::Result;
use clap::Parser;
use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

/// Speakify - Transform any image to look like 스핔이
/// 
/// Supported input formats: PNG, JPEG, WebP
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image path (supports: PNG, JPEG, WebP)
    #[arg(short, long)]
    input: PathBuf,

    /// Resolution (width and height will be set to this value)
    #[arg(short, long, default_value_t = 128)]
    resolution: u32,

    /// Number of frames in the morphing animation
    #[arg(short, long, default_value_t = 100)]
    frames: usize,

    /// Proximity importance for morphing algorithm
    #[arg(short, long, default_value_t = 13)]
    proximity: i64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output_path = output_path_for_input(&args.input);

    println!("Speakify");
    println!("================================");
    println!("Input: {:?}", args.input);
    println!("Output: {:?}", output_path);
    println!("Resolution: {}x{}", args.resolution, args.resolution);
    println!("Frames: {}", args.frames);
    println!();

    // Validate input file exists
    if !args.input.exists() {
        anyhow::bail!("Input file not found: {:?}", args.input);
    }

    // Load and prepare source image
    println!("Loading input image...");
    let source_img = match image::open(&args.input) {
        Ok(img) => img,
        Err(e) => {
            anyhow::bail!(
                "Failed to load image: {}\nSupported formats: PNG, JPEG, WebP", 
                e
            );
        }
    };
    let source_img = prepare_image(source_img, args.resolution);
    
    // Load target image (스핔이.png)
    println!("Loading target image (스핔이.png)...");
    let target_bytes = include_bytes!("../assets/speakify.png");
    let target_img = image::load_from_memory(target_bytes)?;
    let target_img = prepare_image(target_img, args.resolution);

    // Calculate morphing
    println!("Calculating morphing assignments...");
    let assignments = morph::calculate_assignments(&source_img, &target_img, args.proximity);

    // Generate animation frames and create GIF
    println!("Generating {} animation frames and creating GIF...", args.frames);
    let gif_bytes = morph::create_morphing_gif(
        &source_img,
        &target_img,
        &assignments,
        args.frames,
    )?;

    std::fs::write(&output_path, gif_bytes)?;
    println!("Cuayo~ Cuayo~ Output saved to: {:?}", output_path);
    
    Ok(())
}

fn output_path_for_input(input: &PathBuf) -> PathBuf {
    let parent = input.parent().unwrap_or_else(|| std::path::Path::new("."));
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    parent.join(format!("{stem}_Cuayo.gif"))
}

fn prepare_image(img: DynamicImage, size: u32) -> image::RgbImage {
    // Crop to square (center crop)
    let (width, height) = img.dimensions();
    let min_dim = width.min(height);
    
    let x_offset = (width - min_dim) / 2;
    let y_offset = (height - min_dim) / 2;
    
    let cropped = img.crop_imm(x_offset, y_offset, min_dim, min_dim);
    
    // Resize to target size and convert to RGB
    let resized = image::imageops::resize(
        &cropped.to_rgb8(),
        size,
        size,
        image::imageops::FilterType::Lanczos3,
    );
    
    resized
}
