#![feature(iter_array_chunks)]
use image::RgbImage;
use rayon::prelude::*;
use image::Rgb;
use std::time::Instant;

const WIDTH:usize =1920;
const HEIGHT:u32 = 1080;
const MAX_ITERATIONS:u32= 1_000;


fn main() {
    let mut imgbuf: RgbImage = image::ImageBuffer::new(WIDTH as u32, HEIGHT);

    println!("Using per pixel algorithm");
    let instant = Instant::now();
    imgbuf.enumerate_pixels_mut()
         .par_bridge()
         .for_each(|(x, y, pixel)| 
                 mandel_loop(x, y, pixel,MAX_ITERATIONS)
             );

    let elapsed = instant.elapsed().as_millis() as f32 / 1000.0;

    println!("It took {} seconds",elapsed);

    println!("Using per row algorithm");
    let instant = Instant::now();

    imgbuf.enumerate_pixels_mut()
        .map(|(x,y,pixel)| (x,y,pixel))
        .array_chunks::<WIDTH>()
        .par_bridge()
        .for_each(|list|{
            let mut vector = Vec::new();
            for elem in list{
                vector.push(elem);
            } 
            mandel_loop_array(vector, MAX_ITERATIONS)});
    let elapsed = instant.elapsed().as_millis() as f32 / 1000.0;
    imgbuf.save(format!("fractal_{}_{}_iter{}.png",WIDTH,HEIGHT,MAX_ITERATIONS)).unwrap();
    println!("Image Saved");
    println!("It took {} seconds",elapsed);
}

fn mandel_loop(x:u32, y:u32, pixel:&mut Rgb<u8>, iterations:u32){
    let mut iter_count:u32 = 0;
    let mut float_x:f64;
        let mut float_y:f64;
        let x0 = ((x as f64) / WIDTH as f64) * 3.5 - 2.5;
        let y0 = ((y as f64) / HEIGHT as f64) * 2.0 - 1.0; 

        let mut x2 = 0 as f64;
        let mut y2 = 0 as f64;
        let mut w = 0 as f64;
        while (x2+y2)<=4.0 && iter_count < iterations{
            float_x = x2 -y2 +x0;
            float_y = w - x2 -y2 +y0;
            x2 = float_x * float_x;
            y2 = float_y * float_y;
            w = (float_x + float_y) * (float_x + float_y);
            iter_count += 1;
        }
        let color = ((iter_count as f64 / iterations as f64) * 255.0) as f64;
        let r = (color * 255.0) as i32;

        let colors = r as u8;

        *pixel = image::Rgb([colors/3,colors/3,colors]);
}

fn mandel_loop_array(list: Vec<(u32,u32,&mut Rgb<u8>)>, iterations:u32){
    for (x,y,pixel) in list{
        let mut iter_count:u32 = 0;
        let mut float_x:f64;
            let mut float_y:f64;
            let x0 = ((x as f64) / WIDTH as f64) * 3.5 - 2.5;
            let y0 = ((y as f64) / HEIGHT as f64) * 2.0 - 1.0; 

            let mut x2 = 0 as f64;
            let mut y2 = 0 as f64;
            let mut w = 0 as f64;
            while (x2+y2)<=4.0 && iter_count < iterations{
                float_x = x2 -y2 +x0;
                float_y = w - x2 -y2 +y0;
                x2 = float_x * float_x;
                y2 = float_y * float_y;
                w = (float_x + float_y) * (float_x + float_y);
                iter_count += 1;
            }
            let color = ((iter_count as f64 / iterations as f64) * 255.0) as f64;
            let r = (color * 255.0) as i32;

            let colors = r as u8;

            *pixel = image::Rgb([colors/3,colors/3,colors]);
    }
}
