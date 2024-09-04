use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut output = File::create("image.ppm")?;

    output.write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height))?;

    for j in 0..image_height {
        print!("\rScanlines remaining: {} ", image_height - j);
        io::stdout().flush()?;
        for i in 0..image_width {
            let r = i as f64 / (image_width-1) as f64;
            let g = j as f64 / (image_height-1) as f64;
            let b = 0.0;

            let ir = (255.999*r) as i32;
            let ig = (255.999*g) as i32;
            let ib = (255.999*b) as i32;

            output.write_fmt(format_args!("{} {} {}\n", ir,ig,ib))?;
        }
    }
    print!("\rDone.                 \n");

    Ok(())
}