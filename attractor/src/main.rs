// Usage Example
// WIDTH=1024; HEIGHT=1024; STEPS=600; ITERATIONS=10000; ./target/debug/attractor ${WIDTH} ${HEIGHT} ${STEPS} ${ITERATIONS} -5 -5.5 2 2 2 2 2 2 | ffmpeg -y -f rawvideo -vcodec rawvideo -s ${WIDTH}x${HEIGHT} -pix_fmt rgb24 -r 60 -i - -c:v libx264 -pix_fmt yuv420p -an /tmp/lol.mov
use std::process;
use ndarray::prelude::*;
use std::io::{self, Write};
use std::mem;
use std::env;

use attractor::Config;

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

fn run(config: Config) {
    let a = Array::linspace(config.a.0, config.a.1, config.steps);
    let b = Array::linspace(config.b.0, config.b.1, config.steps);
    let c = Array::linspace(config.c.0, config.c.1, config.steps);
    let d = Array::linspace(config.d.0, config.d.1, config.steps);
    
    let mut canvas = vec![0u8; config.width * config.height * 3];
    for i in 0..config.steps {
        process(&mut canvas, config.iterations, a[i], b[i], c[i], d[i], (config.width, config.height));
        
        io::stdout().write(&canvas).expect("failed to write to stdout");

        unsafe {
          libc::memset(
              canvas.as_mut_ptr() as _,
              0,
              canvas.len() * mem::size_of::<u8>(),
          );
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    run(config);

    Ok(())
}
