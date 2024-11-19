use noise::{NoiseFn, Perlin};

pub trait DongoTerrainSource {
    fn get_height_at(&self, x: f32, y: f32) -> f32;
    fn get_height_range(&self) -> (f64,f64);
}

pub struct PerlinTerrainSource {
    pub perlin: Perlin,
    pub noise_factor: f64,
    pub map_max_height: f64,
    pub map_min_height: f64,
    pub limiter: (usize,usize),
}

impl DongoTerrainSource for PerlinTerrainSource {
    fn get_height_at(&self, x: f32, y: f32) -> f32 {        
            
                let nx = x / self.limiter.0 as f32;
                let ny = y / self.limiter.1 as f32;

                let noise_value = self.perlin.get([nx as f64 * self.noise_factor, ny as f64 * self.noise_factor]); // returns a value between -1 and 1

                let normalized_value = (noise_value + 1.0) / 2.0; // set value between 0 and 1
                let height = normalized_value * (self.map_max_height - self.map_min_height) + self.map_min_height;
                height as f32            
    }

    fn get_height_range(&self) -> (f64,f64) {
        (self.map_min_height,self.map_max_height)
    }
}

pub struct FlatTerrainSource {
    pub height: f32,
}

impl DongoTerrainSource for FlatTerrainSource {
    fn get_height_at(&self, _x: f32, _y: f32) -> f32 {
        self.height
    }

    fn get_height_range(&self) -> (f64,f64) {
        (self.height as f64,self.height as f64)
    }
}

pub struct FilteredPerlinTerrainSource {
    pub perlin: Perlin,
    pub noise_factor: f64,
    pub map_max_height: f64,
    pub map_min_height: f64,
    pub limiter: (usize,usize),
    pub filter: Box<dyn Fn(f32,f32) -> f32>,
}

impl DongoTerrainSource for FilteredPerlinTerrainSource {
    fn get_height_at(&self, x: f32, y: f32) -> f32 {        
            
                let nx = x / self.limiter.0 as f32;
                let ny = y / self.limiter.1 as f32;

                let noise_value = self.perlin.get([nx as f64 * self.noise_factor, ny as f64 * self.noise_factor]); // returns a value between -1 and 1
                
                let normalized_value = (noise_value + 1.0) / 2.0; // set value between 0 and 1
                let height = normalized_value * (self.map_max_height - self.map_min_height) + self.map_min_height;
                (self.filter)(x,y) * (height as f32)            
    }

    fn get_height_range(&self) -> (f64,f64) {
        (self.map_min_height,self.map_max_height)
    }
}