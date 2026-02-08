use anyhow::Result;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

/// Calculate pixel assignments using genetic algorithm
pub fn calculate_assignments(
    source: &RgbImage,
    target: &RgbImage,
    proximity_importance: i64,
) -> Result<Vec<usize>> {
    let (width, _height) = source.dimensions();
    assert_eq!(source.dimensions(), target.dimensions());

    let source_pixels: Vec<(u8, u8, u8)> = source
        .pixels()
        .map(|p| (p[0], p[1], p[2]))
        .collect();

    let target_pixels: Vec<(u8, u8, u8)> = target
        .pixels()
        .map(|p| (p[0], p[1], p[2]))
        .collect();

    // Lower weight for better balance with proximity
    let weights = vec![2i64; source_pixels.len()];

    println!("Running genetic algorithm...");
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    let mut pixels = source_pixels
        .iter()
        .enumerate()
        .map(|(i, &(r, g, b))| {
            let x = (i as u32 % width) as u16;
            let y = (i as u32 / width) as u16;
            let h = calculate_heuristic(
                (x, y),
                (x, y),
                (r, g, b),
                target_pixels[i],
                weights[i],
                proximity_importance,
            );
            Pixel::new(x, y, (r, g, b), h)
        })
        .collect::<Vec<_>>();

    let mut rng = frand::Rand::with_seed(12345);
    let swaps_per_generation = 128 * pixels.len();
    let mut max_dist = width;

    loop {
        let mut swaps_made = 0;
        for _ in 0..swaps_per_generation {
            let apos = rng.gen_range(0..pixels.len() as u32) as usize;
            let ax = apos as u16 % width as u16;
            let ay = apos as u16 / width as u16;
            
            let bx = (ax as i16 + rng.gen_range(-(max_dist as i16)..(max_dist as i16 + 1)))
                .clamp(0, width as i16 - 1) as u16;
            let by = (ay as i16 + rng.gen_range(-(max_dist as i16)..(max_dist as i16 + 1)))
                .clamp(0, width as i16 - 1) as u16;
            let bpos = by as usize * width as usize + bx as usize;

            let t_a = target_pixels[apos];
            let t_b = target_pixels[bpos];

            let a_on_b_h = pixels[apos].calc_heuristic(
                (bx, by),
                t_b,
                weights[bpos],
                proximity_importance,
            );

            let b_on_a_h = pixels[bpos].calc_heuristic(
                (ax, ay),
                t_a,
                weights[apos],
                proximity_importance,
            );

            // swap 후: apos에는 b가, bpos에는 a가 감
            let improvement_a = pixels[apos].h - a_on_b_h;  // a가 b자리로
            let improvement_b = pixels[bpos].h - b_on_a_h;  // b가 a자리로
            
            if improvement_a + improvement_b > 0 {
                pixels.swap(apos, bpos);
                // swap 후: apos에는 원래 b가 있음
                pixels[apos].update_heuristic(b_on_a_h);
                // swap 후: bpos에는 원래 a가 있음
                pixels[bpos].update_heuristic(a_on_b_h);
                swaps_made += 1;
            }
        }

        let progress = (1.0 - max_dist as f32 / width as f32) * 100.0;
        pb.set_position(progress as u64);
        pb.set_message(format!("swaps: {}", swaps_made));

        if max_dist < 4 && swaps_made < 10 {
            pb.finish_with_message("Complete!");
            break;
        }

        max_dist = (max_dist as f32 * 0.99).max(2.0) as u32;
    }

    let assignments = pixels
        .iter()
        .map(|p| p.src_y as usize * width as usize + p.src_x as usize)
        .collect();

    Ok(assignments)
}

/// Create an animated GIF showing the morphing process
pub fn create_morphing_gif(
    source: &RgbImage,
    target: &RgbImage,
    assignments: &[usize],
    output_path: &Path,
    num_frames: usize,
) -> Result<()> {
    use gif::{Encoder, Frame, Repeat};
    use std::fs::File;
    use rayon::prelude::*;

    let (width, height) = source.dimensions();
    let source_pixels: Vec<(u8, u8, u8)> = source
        .pixels()
        .map(|p| (p[0], p[1], p[2]))
        .collect();

    println!("Generating frames in parallel...");
    let pb = ProgressBar::new(num_frames as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.green/blue} {pos}/{len} frames")
            .unwrap()
            .progress_chars("=>-"),
    );

    // Generate all frames in parallel
    let frames: Vec<Vec<u8>> = (0..num_frames)
        .into_par_iter()
        .map(|frame_idx| {
            let t = frame_idx as f32 / (num_frames - 1) as f32;
            // Apply ease-in-out for smoother animation
            let t_smooth = ease_in_out_cubic(t);
            generate_interpolated_frame(&source_pixels, assignments, width, height, t_smooth)
        })
        .inspect(|_| pb.inc(1))
        .collect();

    pb.finish_with_message("Frames generated!");

    println!("Writing GIF file...");
    let mut file = File::create(output_path)?;
    let mut encoder = Encoder::new(&mut file, width as u16, height as u16, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    let pb2 = ProgressBar::new(num_frames as u64);
    pb2.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.yellow/blue} {pos}/{len} encoding")
            .unwrap()
            .progress_chars("=>-"),
    );

    for frame_data in frames {
        let mut frame = Frame::from_rgb(width as u16, height as u16, &frame_data);
        frame.delay = 5; // 5 * 10ms = 50ms per frame (slower)
        encoder.write_frame(&frame)?;
        pb2.inc(1);
    }

    pb2.finish_with_message("GIF created");

    Ok(())
}

/// Ease-in-out cubic function for smooth animation
fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

fn generate_interpolated_frame(
    source_pixels: &[(u8, u8, u8)],
    assignments: &[usize],
    width: u32,
    height: u32,
    t: f32,
) -> Vec<u8> {
    let size = (width * height) as usize;
    
    // Splat accumulation buffers (f32 for precision)
    let mut acc_r = vec![0.0f32; size];
    let mut acc_g = vec![0.0f32; size];
    let mut acc_b = vec![0.0f32; size];
    let mut acc_w = vec![0.0f32; size];
    
    // Splat each pixel with bilinear distribution
    for target_idx in 0..assignments.len() {
        let source_idx = assignments[target_idx];
        let (sr, sg, sb) = source_pixels[source_idx];
        
        // Use original source color (no blending)
        let r = sr as f32;
        let g = sg as f32;
        let b = sb as f32;
        
        // Calculate interpolated position
        let target_x = (target_idx % width as usize) as f32;
        let target_y = (target_idx / width as usize) as f32;
        let source_x = (source_idx % width as usize) as f32;
        let source_y = (source_idx / width as usize) as f32;
        
        let fx = source_x * (1.0 - t) + target_x * t;
        let fy = source_y * (1.0 - t) + target_y * t;
        
        // Splat to 3x3 neighbors for better coverage
        let x0 = fx.floor() as i32;
        let y0 = fy.floor() as i32;
        let dx = fx - x0 as f32;
        let dy = fy - y0 as f32;

        for oy in -1..=1 {
            let wy = if oy == -1 {
                (1.0 - dy).powi(2)
            } else if oy == 0 {
                1.0 - ((dx + dy) * 0.0 + (dy - 0.5).abs() * 2.0).min(1.0)
            } else {
                dy.powi(2)
            };
            for ox in -1..=1 {
                let wx = if ox == -1 {
                    (1.0 - dx).powi(2)
                } else if ox == 0 {
                    1.0 - ((dx - 0.5).abs() * 2.0).min(1.0)
                } else {
                    dx.powi(2)
                };
                let weight = wx * wy;
                if weight <= 0.0 {
                    continue;
                }
                let nx = x0 + ox;
                let ny = y0 + oy;
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let idx = (ny as u32 * width + nx as u32) as usize;
                    acc_r[idx] += r * weight;
                    acc_g[idx] += g * weight;
                    acc_b[idx] += b * weight;
                    acc_w[idx] += weight;
                }
            }
        }
    }
    
    // Normalize and convert to u8
    let mut frame_data = vec![0u8; size * 3];
    let mut filled = vec![false; size];
    for i in 0..size {
        if acc_w[i] > 0.0 {
            let idx = i * 3;
            frame_data[idx] = (acc_r[i] / acc_w[i]).round().clamp(0.0, 255.0) as u8;
            frame_data[idx + 1] = (acc_g[i] / acc_w[i]).round().clamp(0.0, 255.0) as u8;
            frame_data[idx + 2] = (acc_b[i] / acc_w[i]).round().clamp(0.0, 255.0) as u8;
            filled[i] = true;
        }
    }

    // Fill holes by propagating nearest filled colors (prevents empty pixels)
    if filled.iter().any(|v| !v) {
        use std::collections::VecDeque;

        let w = width as i32;
        let h = height as i32;
        let mut q = VecDeque::with_capacity(size);
        for i in 0..size {
            if filled[i] {
                q.push_back(i);
            }
        }

        while let Some(i) = q.pop_front() {
            let x = (i as i32 % w) as i32;
            let y = (i as i32 / w) as i32;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue;
                }
                let ni = (ny * w + nx) as usize;
                if !filled[ni] {
                    let src = i * 3;
                    let dst = ni * 3;
                    frame_data[dst] = frame_data[src];
                    frame_data[dst + 1] = frame_data[src + 1];
                    frame_data[dst + 2] = frame_data[src + 2];
                    filled[ni] = true;
                    q.push_back(ni);
                }
            }
        }
    }

    frame_data
}

#[inline]
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 * (1.0 - t) + b as f32 * t).round().clamp(0.0, 255.0) as u8
}

#[derive(Clone, Copy)]
struct Pixel {
    src_x: u16,
    src_y: u16,
    rgb: (u8, u8, u8),
    h: i64,
}

impl Pixel {
    fn new(src_x: u16, src_y: u16, rgb: (u8, u8, u8), h: i64) -> Self {
        Self {
            src_x,
            src_y,
            rgb,
            h,
        }
    }

    fn update_heuristic(&mut self, new_h: i64) {
        self.h = new_h;
    }

    fn calc_heuristic(
        &self,
        target_pos: (u16, u16),
        target_col: (u8, u8, u8),
        weight: i64,
        proximity_importance: i64,
    ) -> i64 {
        calculate_heuristic(
            (self.src_x, self.src_y),
            target_pos,
            self.rgb,
            target_col,
            weight,
            proximity_importance,
        )
    }
}

#[inline(always)]
fn calculate_heuristic(
    apos: (u16, u16),
    bpos: (u16, u16),
    a: (u8, u8, u8),
    b: (u8, u8, u8),
    color_weight: i64,
    spatial_weight: i64,
) -> i64 {
    // squared distances (안정적)
    let spatial = (apos.0 as i64 - bpos.0 as i64).pow(2) + (apos.1 as i64 - bpos.1 as i64).pow(2);
    let color = (a.0 as i64 - b.0 as i64).pow(2)
        + (a.1 as i64 - b.1 as i64).pow(2)
        + (a.2 as i64 - b.2 as i64).pow(2);
    color * color_weight + spatial * spatial_weight
}
