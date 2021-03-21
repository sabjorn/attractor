pub struct Config
{
    pub width: usize,
    pub height: usize,
    pub steps: usize,
    pub iterations: u32,
    pub window_enabled: bool,
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub c: (f32, f32),
    pub d: (f32, f32)
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 13 {
            return Err("not enough arguments");
        }
        let width = args[1].parse::<usize>().unwrap();
        let height = args[2].parse::<usize>().unwrap();
        let steps = args[3].parse::<usize>().unwrap();
        let iterations = args[4].parse::<u32>().unwrap();
        let window_enabled = args[5].parse::<bool>().unwrap();

        let a = (args[6].parse::<f32>().unwrap(), args[7].parse::<f32>().unwrap()); 
        let b = (args[8].parse::<f32>().unwrap(), args[9].parse::<f32>().unwrap()); 
        let c = (args[10].parse::<f32>().unwrap(), args[11].parse::<f32>().unwrap()); 
        let d = (args[12].parse::<f32>().unwrap(), args[13].parse::<f32>().unwrap()); 
        
        Ok(Config { width, height, steps, iterations, window_enabled, a, b, c, d })
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

pub struct Colour
{
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Colour {
    fn as_array(&self) -> [u8; 4]
    {
        // order expected by minifb
        [self.b, self.g, self.r, self.a]
    }
    pub fn to_u32(&self) -> u32 {
        u32::from_ne_bytes(self.as_array())
    }
}