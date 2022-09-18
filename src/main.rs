use image::RgbImage;

fn main() {
    mandelbrot(3840, 2160,50000);
}

fn mandelbrot(width:u32, height:u32,iterations:u32){


    let mut imgbuf: RgbImage = image::ImageBuffer::new(width, height);
    let mut iter_count:u32 = 0;

    for (x,y,pixel) in imgbuf.enumerate_pixels_mut(){
        let mut float_x = 0 as f64;
        let mut float_y = 0 as f64;
        let x0 = ((x as f64) / width as f64) * 3.5 - 2.5;
        let y0 = ((y as f64) / height as f64) * 2.0 - 1.0; 
        while (float_x*float_x + float_y*float_y )<=4.0 && iter_count < iterations{
            let xtemp = float_x*float_x - float_y*float_y + x0;
            float_y = 2.0*float_x*float_y + y0;
            float_x = xtemp;
            iter_count += 1;
        }
        let color = ((iter_count as f64 / iterations as f64) * 255.0) as f64;
        let r = (color * 255.0) as i32;

        let colors = r as u8;

        *pixel = image::Rgb([colors/3,colors/3,colors]);
        iter_count = 0;
    }

    imgbuf.save(format!("fractal_{}_{}_iter{}.png",width,height,iterations)).unwrap();

}
