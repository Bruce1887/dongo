use noise::{NoiseFn, Perlin};
use crate::common::*;

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
/// Default terrain filter. 
/// Creates mountains at the edges and plains at the center.
#[inline]
pub fn default_terrain_filter(x: f32,y: f32) -> f32 {        
    let map_x = MAP_VERTEX_DISTANCE * MAP_SIZE.0 as f32;
    let map_y = MAP_VERTEX_DISTANCE * MAP_SIZE.1 as f32;        
        
    // // this will add a big wall at the very edge of the map. 
    // if x.abs() * 2.0 == map_x || y.abs() * 2.0 == map_y {
    //     return 20.0;
    // }

    // center is always at (0,0), because i explicitly chose for it to be there
    let distance_from_center = ((x).powf(2.0) + (y).powf(2.0)).sqrt();        
    let max_distance = (map_x.powf(2.0) + map_y.powf(2.0)).sqrt();        
    let raw_cost = distance_from_center / max_distance;        
    
    // Ensure raw_cost is between 0 and 1
    debug_assert!(raw_cost >= 0.0 && raw_cost <= 1.0);
    
    let factor = (0.2, 4.0);
    // Scale the raw cost to the desired range
    let scaled_cost = raw_cost * (factor.1 - factor.0) + factor.0;        
    
    scaled_cost
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