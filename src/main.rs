use image::RgbImage;
use rayon::prelude::*;
use image::Rgb;

const WIDTH:u32 = 3840;
const HEIGHT:u32 = 2160;
const ITERATIONS:u32= 100000;



fn main() {
    let mut imgbuf: RgbImage = image::ImageBuffer::new(WIDTH, HEIGHT);

    imgbuf.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| 
                mandel_loop(x, y, pixel,ITERATIONS)
            );

    imgbuf.save(format!("fractal_{}_{}_iter{}.png",WIDTH,HEIGHT,ITERATIONS)).unwrap();

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
