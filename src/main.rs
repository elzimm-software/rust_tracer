mod vec3;

use std::fs::File;
use std::io;
use std::io::Write;
use image::ImageBuffer;

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut imgbuf = ImageBuffer::new(image_width, image_height);

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        io::stdout().flush()?;
        for i in 0..image_width {
            let r = i as f64 / (image_width-1) as f64;
            let g = j as f64 / (image_height-1) as f64;
            let b = 0.0;

            let ir = (255.999*r) as u8;
            let ig = (255.999*g) as u8;
            let ib = (255.999*b) as u8;

            let pixel = imgbuf.get_pixel_mut(i,j);
            *pixel = image::Rgb([ir,ig,ib]);
        }
    }
    imgbuf.save("image.png").unwrap();

    print!("\rDone.                 \n");

    Ok(())
}