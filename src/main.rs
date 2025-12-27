use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// translate screen coords to points in the complex plane
fn px2complex(px: usize, py: usize, w: usize, h: usize) -> (f64, f64) {
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.0;
    let y_max = 1.0;

    let x = x_min + (px as f64 / (w as f64 - 1.0)) * (x_max - x_min);
    let y = y_min + (py as f64 / (h as f64 - 1.0)) * (y_max - y_min);

    (x, y)
}

fn mandelbrot(cx: f64, cy: f64, maxIter: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iter = 0;

    while x*x + y*y <= 4.0 && iter < maxIter {
        let xt = x*x - y*y + cx;
        y = 2.0*x*y - cy;
        x = xt;
        iter += 1;
    }

    iter
}

// compute values for a single row in the image
// map iter to values between 0 and 255
fn computeRow(y: usize, w: usize, h: usize, maxIter: u32) -> Vec<u8> {
    let mut row: Vec<u8> = Vec::with_capacity(w);
    for x in 0..w {
        let (cx, cy) = px2complex(x, y, w, h);
        let iter = mandelbrot(cx, cy, maxIter);
        row.push((iter % 256) as u8);
    }

    row
}

// a single-threaded approach to mandelbrot
fn mandelDumb(w: usize, h: usize, maxIter: u32) -> Vec<Vec<u8>> {
    let mut image: Vec<Vec<u8>> = Vec::with_capacity(h);
    for y in 0..h {
        let row = computeRow(y, w, h, maxIter);
        image.push(row);
    }
    image
}

// write the fractal out to a PGM greyscale image file
fn genPgmImage(image: &Vec<Vec<u8>>, w: usize, h: usize) -> io::Result<()> {
    let path = Path::new("mandelbrot.pgm");
    let mut file = File::create(&path)?;

    // write the file header
    file.write_all("P2\n".as_bytes())?;
    file.write_all(format!("{0} {1}\n255\n", w, h).as_bytes())?;

    //write the image data in ASCII
    for row in image {
        for px in row {
            write!(file, "{:03} ", *px)?;
        }
        write!(file, "\n")?;
    }

    Ok(())
}

fn main() {
    let w= 1920;
    let h = 1080;

    let image = mandelDumb(w,h, 1000 );

    match genPgmImage(&image, w, h) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: failed to generate PGM {}", e);
        }
    }
}
