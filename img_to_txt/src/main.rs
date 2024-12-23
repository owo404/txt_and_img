use image;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let img = image::open("input.png").unwrap();
    let rgb_img = img.to_rgb8();

    let (width, height) = rgb_img.dimensions();

    let mut output: Vec<u8> = vec![];

    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = rgb_img.get_pixel(x, y);

            if pixel[0] == 0 {
                break 'outer;
            }  else if pixel[1] == 0 {
                output.push(pixel[0]);
                break 'outer;
            } else if pixel[2] == 0 {
                output.push(pixel[0]);
                output.push(pixel[1]);
                break 'outer;
            }

            output.push(pixel[0]);
            output.push(pixel[1]);
            output.push(pixel[2]);
        }
    }

    let mut file = File::create("output.txt")?;
    file.write_all(&output)?;

    Ok(())
}