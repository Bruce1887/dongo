use rand::Rng;

use three_d::*;

const MAP_MAX_HEIGHT: f32 = 5.0;
const MAP_MIN_HEIGHT: f32 = 0.0;

const ELEVATION_CHANGE: f32 = 1.0; // max change in height between two adjacent vertices

#[allow(dead_code)]
pub enum ColorMode {
    Height,
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

    fn define_positions(&mut self) {
        // generate positions of the vertices
        let mut rng = rand::thread_rng();
        let mut height = rng.gen_range(MAP_MIN_HEIGHT..=MAP_MAX_HEIGHT).round();
        // println!("startheight: {}",height);
    
        for y in 0..self.vert_size.1 {
            // println!("y: {}",y);
            for x in 0..self.vert_size.0 {
                self.positions.push(vec3(
                    x as f32 - self.size.0 as f32 / 2f32,
                    y as f32 - self.size.1 as f32 / 2f32,
                    height,
                ));
    
                // change the height of the next vertex
                let elevation_change = rng.gen_range(-ELEVATION_CHANGE..=ELEVATION_CHANGE).round();
                // println!("height: {}",height);
                // println!("x: {}",x);
                height = (height + elevation_change).clamp(MAP_MIN_HEIGHT, MAP_MAX_HEIGHT);
            }
        }
    }
    
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
    
    fn paint_my_mesh(&mut self, colormode: ColorMode) {
        match colormode {
            ColorMode::Height => {
                // color the vertices based on their height
                for i in 0..self.num_verts {
                    let height_range = MAP_MAX_HEIGHT - MAP_MIN_HEIGHT;
                    let height = self.positions[i].z;
                    if height < height_range / 4.0 {
                        self.colors.push(Srgba::BLACK);
                    } else if height < height_range / 2.0 {
                        self.colors.push(Srgba::BLUE);
                    } else if height < 3.0 * height_range / 4.0 {
                        self.colors.push(Srgba::GREEN);
                    } else {
                        self.colors.push(Srgba::WHITE);
                    }
                }
            }
            ColorMode::Checkerboard => {
                // strange color pattern, kinda useful
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
