// Usage Example
// WIDTH=1024; HEIGHT=1024; STEPS=600; ITERATIONS=10000; ./target/debug/attractor ${WIDTH} ${HEIGHT} ${STEPS} ${ITERATIONS} -5 -5.5 2 2 2 2 2 2 | ffmpeg -y -f rawvideo -vcodec rawvideo -s ${WIDTH}x${HEIGHT} -pix_fmt rgb24 -r 60 -i - -c:v libx264 -pix_fmt yuv420p -an /tmp/lol.mov
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::process;
use ndarray::prelude::*;
use std::io::{self, Write};
use std::mem;
use std::env;

use attractor::{Config, Points};

static CHANNELS: usize = 4;

fn process(canvas: &mut [u8], iterations: u32, a: f32, b:f32, c:f32, d:f32, dimensions:(usize, usize)) {
    let mut points = attractor::Points{x:1., y:1.};
    
    let (width, height) = dimensions;
    
    for _ in 0..iterations {
        points.calculate_attractor(a, b, c, d);
        let x_pos = ((points.x + 2.) / 4. * ((width - 1) as f32)) as usize;
        let y_pos = ((points.y + 2.) / 4. * ((height - 1) as f32)) as usize;
        let i = (x_pos + width * y_pos) * CHANNELS;
        canvas[i] = 255;
        canvas[i+1] = 255;
    }
}

fn run(config: Config) {
    let a = Array::linspace(config.a.0, config.a.1, config.steps);
    let b = Array::linspace(config.b.0, config.b.1, config.steps);
    let c = Array::linspace(config.c.0, config.c.1, config.steps);
    let d = Array::linspace(config.d.0, config.d.1, config.steps);
    
    let mut canvas = vec![0u8; config.width * config.height * CHANNELS];

    // let mut window = Window::new(
    //         "Test - ESC to exit",
    //         config.width,
    //         config.height,
    //         WindowOptions::default(),
    //     ).unwrap_or_else(|e| {
    //         panic!("{}", e);
    //     });

    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // let mut i = 0;
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     process(&mut canvas, config.iterations, a[i], b[i], c[i], d[i], (config.width, config.height));
    //     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    //     window
    //         .update_with_buffer(&canvas, config.width, config.height)
    //         .unwrap();

    //     unsafe {
    //       libc::memset(
    //           canvas.as_mut_ptr() as _,
    //           0,
    //           canvas.len() * mem::size_of::<u32>(),
    //       );
    //     }
    //     i = (i + 1) % config.steps;
    // }
    for i in 0..config.steps {
        process(&mut canvas, config.iterations, a[i], b[i], c[i], d[i], (config.width, config.height));
       
        // let p = canvas.as_mut_ptr() as *mut u8;
        // let len = canvas.len() * std::mem::size_of::<u32>();
        // let cap = canvas.capacity() * std::mem::size_of::<u32>();
        // let rebuild = unsafe { Vec::from_raw_parts(p, len, cap)};
        // io::stdout().write(&rebuild).expect("failed to write to stdout");
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
