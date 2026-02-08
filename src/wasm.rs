use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat, RgbaImage};
use web_sys::console;

#[wasm_bindgen]
pub struct SpeakifyWasm {
    target_image: Option<DynamicImage>,
}

#[wasm_bindgen]
impl SpeakifyWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SpeakifyWasm, JsValue> {
        console_error_panic_hook::set_once();
        
        // Load target image (speakify.png) - needs to be embedded
        let target_bytes = include_bytes!("../assets/speakify.png");
        let target_image = image::load_from_memory(target_bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to load target: {}", e)))?;
        
        Ok(SpeakifyWasm {
            target_image: Some(target_image),
        })
    }

    #[wasm_bindgen]
    pub fn convert(
        &self,
        input_bytes: &[u8],
        resolution: u32,
        frames: usize,
    ) -> Result<Vec<u8>, JsValue> {
        console::log_1(&"Starting conversion...".into());
        
        // Load input image
        let input_image = image::load_from_memory(input_bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to load input: {}", e)))?;
        
        let target_image = self.target_image.as_ref()
            .ok_or_else(|| JsValue::from_str("Target image not loaded"))?;
        
        // Prepare images
        let src_img = prepare_image(&input_image, resolution);
        let target_img = prepare_image(target_image, resolution);
        
        console::log_1(&format!("Calculating pixel assignments for {}x{} image...", resolution, resolution).into());
        
        // Calculate assignments
        let assignments = crate::morph::calculate_assignments(&src_img, &target_img, 13);
        
        console::log_1(&format!("Generating {} frames...", frames).into());
        
        // Create GIF
        let gif_bytes = crate::morph::create_morphing_gif(
            &src_img,
            &target_img,
            &assignments,
            frames,
        ).map_err(|e| JsValue::from_str(&format!("Failed to create GIF: {}", e)))?;
        
        console::log_1(&"Conversion complete!".into());
        
        Ok(gif_bytes)
    }
}

fn prepare_image(img: &DynamicImage, size: u32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let min_dim = width.min(height);
    
    let crop_x = (width - min_dim) / 2;
    let crop_y = (height - min_dim) / 2;
    
    let cropped = img.crop_imm(crop_x, crop_y, min_dim, min_dim);
    let resized = cropped.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
    
    resized.to_rgba8()
}
