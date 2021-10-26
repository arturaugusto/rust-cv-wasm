mod utils;

use wasm_bindgen::prelude::*;

use cv::{feature::akaze::{Akaze, KeyPoint}};
use ::image::RgbaImage;
use image::{DynamicImage, Rgba, ImageBuffer};
//use imageproc::drawing;


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
pub fn greet(data: Vec<u8>, width: u32, height: u32) -> Vec<u8>{
    
    let mut src_image_buf: RgbaImage = ImageBuffer::new(width, height);
    
    for y in 0..height as usize {
        for x in 0..width as usize {
            let start = x + y * height as usize;
            src_image_buf.put_pixel(x as u32, y as u32, Rgba([data[start], data[start+1], data[start+2], data[start+3]]));
        }
    }

    let src_image   = DynamicImage::ImageRgba8(src_image_buf);
    // let subimg = imageops::crop(&mut img, 0, 0, 100, 100);

    // Create an instance of `Akaze` with the default settings.
    let akaze = Akaze::default();

    // Extract the features from the image using akaze.
    match akaze.extract(&src_image) {
        (key_points, _descriptors) => {
            let res_bytes = src_image.into_bytes();
            for key_point in key_points {
                println!("{}", key_point.point.1);
            }
            return res_bytes
        },
        _ => {
            unimplemented!();
        }
    } 
    // return src_image.into_bytes();
}


#[test]
fn testit_test() {
    let width = 40;
    let height = 40;
    let mut src_image_buf: RgbaImage = ImageBuffer::new(width, height);

    for y in 10..(height-4) as usize {
        for x in 10..(width-4) as usize {
            src_image_buf.put_pixel(x as u32, y as u32, Rgba([100,230,1,255]));
        }
    }

    let _res = greet(src_image_buf.to_vec(), width, height);
    // println!("{}", res[0]);
    
}
