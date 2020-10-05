use image::{ImageBuffer, RgbImage, Rgb};
use std::convert::TryInto;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;
const SCALE: u32 = 2;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: num outfile");
        std::process::exit(-1);
    }
    let num: u32 = args.remove(0).parse().expect("Couldn't parse the argument to an integer");
    let outfile = args.remove(0);

    println!("Generating pascal's triangle with mod {}", num);

    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    set_background(&mut imgbuf);

    draw_triangle(&mut imgbuf, num);

    imgbuf.save(outfile).unwrap();
}

fn draw_triangle(imgbuf: &mut RgbImage, num: u32) {
    let mut prev: Vec<u32> = Vec::new();
    prev.push(1);
    let mut curr: Vec<u32> = Vec::new();
    let mut line: u32 = 2;
    while get_y(line) < HEIGHT - 50  {
        // construct next line
        curr.push(1);
        for i in 0..(prev.len()-1) {
            curr.push((prev[i] + prev[i+1]) % num);
        }
        curr.push(1);
        line += 1;
        // draw the values based on mod
        for (i, value) in curr.iter().enumerate() {
            if value % num == 0 {
                draw_point(imgbuf, get_x(line, i.try_into().unwrap()), get_y(line));
            }
        }
        prev = curr;
        curr = Vec::new();
    }
}

fn get_x(line : u32, i : u32) -> u32 {
    WIDTH / 2 + SCALE * i - (line * SCALE) / 2
}
fn get_y(line : u32) -> u32 {
    50 + (((line as f64) * (3 as f64).sqrt() * (SCALE as f64) / 2.0) as u32)
}

fn draw_point(imgbuf: &mut RgbImage, x : u32, y : u32) {
    let pixel = imgbuf.get_pixel_mut(x, y);
    *pixel = Rgb([0, 0, 0]);
}

fn set_background(imgbuf: &mut RgbImage) {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.5 * x as f32) as u8;
        let b = (0.5 * y as f32) as u8;
        *pixel = Rgb([r, 150, b]);
    }
}
