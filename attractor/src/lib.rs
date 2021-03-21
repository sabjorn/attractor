pub struct Config
{
    pub width: usize,
    pub height: usize,
    pub steps: usize,
    pub iterations: u32,
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub c: (f32, f32),
    pub d: (f32, f32)
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 12 {
            return Err("not enough arguments");
        }
        let width = args[1].parse::<usize>().unwrap();
        let height = args[2].parse::<usize>().unwrap();
        let steps = args[3].parse::<usize>().unwrap();
        let iterations = args[4].parse::<u32>().unwrap();

        let a = (args[5].parse::<f32>().unwrap(), args[6].parse::<f32>().unwrap()); 
        let b = (args[7].parse::<f32>().unwrap(), args[8].parse::<f32>().unwrap()); 
        let c = (args[9].parse::<f32>().unwrap(), args[10].parse::<f32>().unwrap()); 
        let d = (args[11].parse::<f32>().unwrap(), args[12].parse::<f32>().unwrap()); 
        
        Ok(Config { width, height, steps, iterations, a, b, c, d })
    }
}

pub struct Points
{
    pub x: f32,
    pub y: f32
}

impl Points {
    pub fn calculate_attractor(&mut self, a: f32, b: f32, c: f32, d: f32) {
        let x = (a*self.y).sin() - (b*self.x).cos();
        let y = (c*self.x).cos() - (d*self.y).cos();
        self.x = x;
        self.y = y;
    }
}