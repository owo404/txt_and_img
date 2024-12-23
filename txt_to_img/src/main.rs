use std::fs::File;
use std::io::{self, Read};
use image::{RgbImage, Rgb};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    let s = buffer.trim();

    let bytes = s.as_bytes();
    let mut v_bytes: Vec<u8> = bytes.to_vec();

    let length = (((bytes.len() + (3 - (bytes.len() % 3))) as f64) / 3.0).sqrt().ceil() as u32;
    let pixel = length * length;

    while v_bytes.len() < (pixel * 3) as usize {
        v_bytes.push(0);
    }

    v_bytes.reverse();

    let mut img: RgbImage = RgbImage::new(length, length);

    for y in 0..length {
        for x in 0..length {
            let r = v_bytes.pop().unwrap();
            let g = v_bytes.pop().unwrap();
            let b = v_bytes.pop().unwrap();

            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    img.save("output.png").unwrap();

    Ok(())
}