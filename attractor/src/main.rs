// Usage Example
// WIDTH=1024; HEIGHT=1024; STEPS=600; ITERATIONS=10000; ./target/debug/attractor ${WIDTH} ${HEIGHT} ${STEPS} ${ITERATIONS} -5 -5.5 2 2 2 2 2 2 | ffmpeg -y -f rawvideo -vcodec rawvideo -s ${WIDTH}x${HEIGHT} -pix_fmt rgb24 -r 60 -i - -c:v libx264 -pix_fmt yuv420p -an /tmp/lol.mov

use ndarray::prelude::*;
use std::io::{self, Write};
use std::mem;
use std::env;

fn calulate(x: f32, y: f32, a: f32, b: f32, c: f32, d: f32) -> (f32, f32)
{
    let x_ = (a*y).sin() - (b*x).cos();
    let y_ = (c*x).cos() - (d*y).cos();
    (x_, y_)
}

fn process(canvas: &mut [u8], iterations: u32, a: f32, b:f32, c:f32, d:f32, dimensions:(usize, usize)) {
    let mut x = 1 as f32;
    let mut y = 1 as f32;
    
    let (width, height) = dimensions;
    
    for _ in 0..iterations {
        let (x_, y_) = calulate(x as f32, y as f32, a, b, c, d);
        let x_pos = ((x + 2.) / 4. * ((width - 1) as f32)) as usize;
        let y_pos = ((y + 2.) / 4. * ((height - 1) as f32)) as usize;
        let i = (x_pos + width * y_pos) * 3;
        canvas[i] = 255;
        x = x_;
        y = y_;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let width = &args[1].parse::<usize>().unwrap();
    let height = &args[2].parse::<usize>().unwrap();
    let steps = &args[3].parse::<usize>().unwrap();
    let iterations = &args[4].parse::<u32>().unwrap();

    let a_range = (&args[5].parse::<f32>().unwrap(), &args[6].parse::<f32>().unwrap()); 
    let b_range = (&args[7].parse::<f32>().unwrap(), &args[8].parse::<f32>().unwrap()); 
    let c_range = (&args[9].parse::<f32>().unwrap(), &args[10].parse::<f32>().unwrap()); 
    let d_range = (&args[11].parse::<f32>().unwrap(), &args[12].parse::<f32>().unwrap()); 

    let a = Array::linspace(*a_range.0, *a_range.1, *steps);
    let b = Array::linspace(*b_range.0, *b_range.1, *steps);
    let c = Array::linspace(*c_range.0, *c_range.1, *steps);
    let d = Array::linspace(*d_range.0, *d_range.1, *steps);
    
    let mut canvas = vec![0u8; width * height * 3];
    for i in 0..*steps {
        process(&mut canvas, *iterations, a[i], b[i], c[i], d[i], (*width, *height));
        
        io::stdout().write(&canvas).expect("failed to write to stdout");

        unsafe {
          libc::memset(
              canvas.as_mut_ptr() as _,
              0,
              canvas.len() * mem::size_of::<u8>(),
          );
        }
    }
    Ok(())
}
