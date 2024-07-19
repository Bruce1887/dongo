use three_d::*;
use noise::{Perlin, NoiseFn};
use rand::Rng;


const MAP_MAX_HEIGHT: f64 = 5.0;
const MAP_MIN_HEIGHT: f64 = 0.0;

#[allow(dead_code)]
pub enum ColorMode {
    HeightMap,
    Checkerboard,    
}
pub struct MapGenerator {
    size: (usize, usize),
    vert_size: (usize, usize),
    num_verts: usize,
    positions: Vec<Vec3>,
    indeces: Vec<u32>,
    colors: Vec<Srgba>,
}

impl MapGenerator {
    pub fn new(input_size: (usize, usize)) -> Self {
        // check valid input
        assert!(input_size.0 % 2 == 0 && input_size.1 % 2 == 0); // Ensure the size is even so middle is at (0,0)
        assert!(input_size.0 > 0 && input_size.1 > 0);

        let v_size = (input_size.0 + 1, input_size.1 + 1);
        let num_v = (input_size.0 + 1) * (input_size.1 + 1);
        MapGenerator {
            size: input_size,
            vert_size: v_size,
            num_verts: num_v,
            positions: Vec::with_capacity(num_v),
            indeces: Vec::with_capacity(num_v),
            colors: Vec::with_capacity(num_v),
        }
    }

    /// Generate a mesh with a checkerboard pattern
    /// consumes self
    pub fn generate(mut self, colormode: ColorMode, context: &Context) -> Gm<Mesh, ColorMaterial> {
        self.define_positions();        
        self.define_indences();        
        self.paint_my_mesh(colormode);

        let cpu_mesh = CpuMesh {
            positions: Positions::F32(self.positions),
            colors: Some(self.colors),
            indices: Indices::U32(self.indeces),
            ..Default::default()
        };
        // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
        return Gm::new(Mesh::new(context, &cpu_mesh), ColorMaterial::default());
    }

    /// generate positions of the vertices
    fn define_positions(&mut self) {
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen();
        let noise = Perlin::new(seed);                
        
        for y in 0..self.vert_size.1 {
            for x in 0..self.vert_size.0 {
                println!("generating position: {} / {}", y * self.vert_size.0 + x +1, self.num_verts);

                let nx = x as f64 / self.vert_size.0 as f64;
                let ny = y as f64 / self.vert_size.1 as f64;
                let nz = 0.0; // You can change this value to get different noise patterns

                let noise_value = noise.get([nx, ny, nz]);
                let normalized_value = (noise_value + 1.0) / 2.0;
                let height = normalized_value * (MAP_MAX_HEIGHT - MAP_MIN_HEIGHT) + MAP_MIN_HEIGHT;
                
                

                self.positions.push(vec3(
                    x as f32 - self.size.0 as f32 / 2.0,
                    y as f32 - self.size.1 as f32 / 2.0,
                    height as f32,
                ));
            }
        }
    }
    
    /// generate indices of the vertices 
    fn define_indences(&mut self) {
        // define the indices of the vertices
        for y in 0..self.vert_size.1 {
            for x in 0..self.vert_size.0 {
                let i = y * self.vert_size.0 + x;
                if x < self.vert_size.0 - 1 && y < self.vert_size.1 - 1 {
                    // lower triangle of square
                    self.indeces.push(i as u32); // bottom left
                    self.indeces.push((i + 1) as u32); // bottom right
                    self.indeces.push((i + self.vert_size.0) as u32); // top left
    
                    // upper triangle of square
                    self.indeces.push((i + self.vert_size.0) as u32); // top left
                    self.indeces.push((i + 1) as u32); // bottom right
                    self.indeces.push((i + self.vert_size.0 + 1) as u32); // top right
                }
            }
        }
    }
    
    /// paint the mesh with colors
    fn paint_my_mesh(&mut self, colormode: ColorMode) {
        match colormode {
            ColorMode::HeightMap => {
                // color the vertices based on their height
                for i in 0..self.num_verts {
                    let height_range = MAP_MAX_HEIGHT - MAP_MIN_HEIGHT;
                    let height = self.positions[i].z;
                    if height < (height_range / 4.0) as f32 { // lowest 25% of the height range
                        self.colors.push(Srgba::BLACK);
                    } else if height < (height_range / 2.0) as f32 { // 25% to 50% of the height range
                        self.colors.push(Srgba::BLUE);
                    } else if height < (3.0 * height_range / 4.0) as f32 { // 50% to 75% of the height range
                        self.colors.push(Srgba::GREEN);
                    } else { // highest 25% of the height range
                        self.colors.push(Srgba::WHITE); 
                    }
                }
            }
            ColorMode::Checkerboard => {
                // strange symmetrical color pattern, kinda useful
                for i in 0..self.vert_size.0 {
                    for j in 0..self.vert_size.1 {
                        let color = if i % 2 == 0 && j % 2 == 0 {
                            Srgba::BLACK
                        } else if i % 2 == 0 && j % 2 == 1 {
                            Srgba::RED
                        } else if i % 2 == 1 && j % 2 == 0 {
                            Srgba::BLUE
                        } else {
                            Srgba::WHITE
                        };
                        self.colors.push(color);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_positions() {}
}
