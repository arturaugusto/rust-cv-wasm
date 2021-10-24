mod utils;

use wasm_bindgen::prelude::*;

use cv::{
    feature::akaze::{Akaze, KeyPoint},
    image::{
        image::{DynamicImage, Rgba, ImageBuffer},
        imageproc::drawing,
    },
    //vis::show_image::{self, event},
};
use ::image::RgbaImage;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> Vec<u8>{
    // Load the image.
    // let src_image = image::open("teste2.jpg").expect("failed to open image file");
    
    // let mut src_image_buf = RgbaImage::new(32, 32);
    let mut src_image_buf: RgbaImage = ImageBuffer::new(512, 512);

    for x in 15..=300 {
        for y in 8..300 {
            src_image_buf.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            src_image_buf.put_pixel(y, x, Rgba([255, 0, 0, 255]));
        }
    }

    let src_image   = DynamicImage::ImageRgba8(src_image_buf);
    // let subimg = imageops::crop(&mut img, 0, 0, 100, 100);

    // Create an instance of `Akaze` with the default settings.
    let akaze = Akaze::default();

    // Extract the features from the image using akaze.
    let (key_points, _descriptors) = akaze.extract(&src_image);

    // Make a canvas with the `imageproc::drawing` module.
    // We use the blend mode so that we can draw with translucency on the image.
    // We convert the image to rgba8 during this process.
    let mut image_canvas = drawing::Blend(src_image.to_rgba8());

    // Draw a cross on the image at every keypoint detected.
    for KeyPoint { point: (x, y), .. } in key_points {
        drawing::draw_cross_mut(
            &mut image_canvas,
            Rgba([0, 255, 255, 128]),
            x as i32,
            y as i32,
        );
    }

    // Get the resulting image.
    let out_image = DynamicImage::ImageRgba8(image_canvas.0);
    let res_bytes = out_image.into_bytes();
    alert(&(res_bytes[2].to_string()));
    return res_bytes
}
