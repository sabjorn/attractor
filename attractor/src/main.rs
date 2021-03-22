// Usage Example
// WIDTH=1024; HEIGHT=1024; STEPS=600; ITERATIONS=10000; ./target/debug/attractor ${WIDTH} ${HEIGHT} ${STEPS} ${ITERATIONS} -5 -5.5 2 2 2 2 2 2 | ffmpeg -y -f rawvideo -vcodec rawvideo -s ${WIDTH}x${HEIGHT} -pix_fmt bgra -r 60 -i - -c:v libx264 -pix_fmt yuv420p -an /tmp/lol.mov
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::process;
use ndarray::prelude::*;
use std::io::{self, Write, Read};
use std::mem;
use std::env;

use attractor::{Config};

fn process(canvas: &mut [u32], iterations: u32, a: f32, b:f32, c:f32, d:f32, dimensions:(usize, usize)) {
    let mut points = attractor::Points{x:1., y:1.};
    
    let (width, height) = dimensions;
    
    for _ in 0..iterations {
        points.calculate_attractor(a, b, c, d);
        let x_pos = ((points.x + 2.) / 4. * ((width - 1) as f32)) as usize;
        let y_pos = ((points.y + 2.) / 4. * ((height - 1) as f32)) as usize;
        let i = x_pos + width * y_pos;

        let colour = attractor::Colour{a:255, r:255, g:0, b:0};
        canvas[i] = colour.to_u32();
    }
}

fn as_u8_slice(v: &[u32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<u32>(),
        )
    }
}

fn as_u32_slice(v: &[u8]) -> &[u32] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u32,
            v.len() * std::mem::size_of::<u8>(),
        )
    }
}

fn windowed_processing(_canvas: &mut [u32], config: attractor::Config) {
    let a = Array::linspace(config.a.0, config.a.1, config.steps);
    let b = Array::linspace(config.b.0, config.b.1, config.steps);
    let c = Array::linspace(config.c.0, config.c.1, config.steps);
    let d = Array::linspace(config.d.0, config.d.1, config.steps);

    let mut window = Window::new(
        "Test - ESC to exit",
        config.width,
        config.height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    let mut buffer = vec![0u8; config.width * config.height * 4];
    let mut i = 0;
    let mut stdin = io::stdin();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        stdin.read_exact(&mut buffer).unwrap_or_else(|e| {
            match e.kind() {
                std::io::ErrorKind::UnexpectedEof => {
                    eprintln!("end of stream");
                    process::exit(0);
                }
                ,
                _ => {
                    eprintln!("end of stream: {}", e);
                    panic!("{:?}", e);
                }
            }
        });

        let canvas = as_u32_slice(&buffer);
        
        window
            .update_with_buffer(&canvas, config.width, config.height)
            .unwrap();

        unsafe {
          libc::memset(
              buffer.as_mut_ptr() as _,
              0,
              buffer.len() * mem::size_of::<u8>(),
          );
        }
    }
}

fn run(config: Config) {
    let mut canvas = vec![0u32; config.width * config.height];
    
    if config.window_enabled {
        windowed_processing(&mut canvas, config);
        return ()
    }
    
    let a = Array::linspace(config.a.0, config.a.1, config.steps);
    let b = Array::linspace(config.b.0, config.b.1, config.steps);
    let c = Array::linspace(config.c.0, config.c.1, config.steps);
    let d = Array::linspace(config.d.0, config.d.1, config.steps);
    for i in 0..config.steps {
        process(&mut canvas, config.iterations, a[i], b[i], c[i], d[i], (config.width, config.height));
        
        let output = as_u8_slice(&canvas);

        io::stdout().write(output).expect("failed to write to stdout");

        unsafe {
          libc::memset(
              canvas.as_mut_ptr() as _,
              0,
              canvas.len() * mem::size_of::<u32>(),
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
