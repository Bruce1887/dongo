use crate::common::{MAPFILE_PATH, MAP_MAX_HEIGHT, MAP_MIN_HEIGHT};
use crate::error::DongoError;

use noise::{NoiseFn, Perlin};
use rand::Rng;
use three_d::*;

use std::{fs::File, io::prelude::*};


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
    indices: Vec<u32>,
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
            indices: Vec::with_capacity(num_v),            
            colors: Vec::with_capacity(num_v),
        }
    }

    pub fn define_parameters(&mut self, colormode: ColorMode) {
        self.define_positions();

        self.define_indences();

        self.paint_my_mesh(colormode);     
    }
    /// Generate a mesh with a checkerboard pattern
    /// consumes self
    pub fn generate_new(
        mut self,
        colormode: ColorMode,
        context: &Context,
    ) -> Gm<Mesh, PhysicalMaterial> {
        self.define_parameters(colormode);

        self.generate(context)
    }

    /// generates a mesh form a MapGenerator, expexts the MapGenerator to have been initialized by reading from a file
    /// # panics
    /// panics if the MapGenerator has not been initialized by reading from a file
    pub fn generate(self, context: &Context) -> Gm<Mesh, PhysicalMaterial> {
        let mut cpu_mesh = CpuMesh {
            positions: Positions::F32(self.positions),
            colors: Some(self.colors),
            indices: Indices::U32(self.indices),
            //normals: Some(self.normals),
            ..Default::default()
        };
        cpu_mesh.compute_normals();
        // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
        return Gm::new(Mesh::new(context, &cpu_mesh), PhysicalMaterial::default());
    }

    /// generate positions of the vertices
    fn define_positions(&mut self) {
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen();
        let noise = Perlin::new(seed);

        for y in 0..self.vert_size.1 {
            for x in 0..self.vert_size.0 {
                println!(
                    "generating position: {} / {}",
                    y * self.vert_size.0 + x + 1,
                    self.num_verts
                );

                let nx = x as f64 / self.vert_size.0 as f64;
                let ny = y as f64 / self.vert_size.1 as f64;                

                let noise_value = noise.get([nx, ny]).abs(); // idk about this abs                
                // let normalized_value = (noise_value + 1.0) / 2.0; // not sure about this
                let height = noise_value * (MAP_MAX_HEIGHT - MAP_MIN_HEIGHT) + MAP_MIN_HEIGHT;

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
                    self.indices.push(i as u32); // bottom left
                    self.indices.push((i + 1) as u32); // bottom right
                    self.indices.push((i + self.vert_size.0) as u32); // top left

                    // upper triangle of square
                    self.indices.push((i + self.vert_size.0) as u32); // top left
                    self.indices.push((i + 1) as u32); // bottom right
                    self.indices.push((i + self.vert_size.0 + 1) as u32); // top right
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
                    if height < (height_range / 4.0) as f32 {
                        // lowest 25% of the height range
                        self.colors.push(Srgba::BLACK);
                    } else if height < (height_range / 2.0) as f32 {
                        // 25% to 50% of the height range
                        self.colors.push(Srgba::BLUE);
                    } else if height < (3.0 * height_range / 4.0) as f32 {
                        // 50% to 75% of the height range
                        self.colors.push(Srgba::GREEN);
                    } else {
                        // highest 25% of the height range
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

    pub fn write_to_file(&self) {
        let mut file = File::create(MAPFILE_PATH).expect("Failed to create file");

        // Write struct fields to file
        writeln!(file, "size: {:?}", self.size).expect("Failed to write to file");
        writeln!(file, "vert_size: {:?}", self.vert_size).expect("Failed to write to file");
        writeln!(file, "num_verts: {:?}", self.num_verts).expect("Failed to write to file");

        let mut pos_str = String::new();
        for v in &self.positions {
            pos_str.push_str(&format!("{},{},{},", v.x, v.y, v.z));
        }
        writeln!(file, "positions: {}", pos_str).expect("Failed to write to file");

        let ind_str = self
            .indices
            .iter()
            .map(|index| index.to_string())
            .collect::<Vec<String>>()
            .join(",");
        writeln!(file, "indeces: {}", ind_str).expect("Failed to write to file");

        let mut col_str = String::new();
        for c in &self.colors {
            col_str.push_str(&format!("{},{},{},{},", c.r, c.g, c.b, c.a));
        }
        writeln!(file, "colors: {}", col_str).expect("Failed to write to file");

        // old stuff
        // writeln!(file, "positions: {:?}", self.positions).expect("Failed to write to file");
        // writeln!(file, "indeces: {:?}", self.indeces).expect("Failed to write to file");
        // writeln!(file, "colors: {:?}", self.colors).expect("Failed to write to file");

        println!("Struct fields written to file");
    }

    pub fn read_from_file(filepath: &str) -> Result<MapGenerator, DongoError> {
        let mut file = match File::open(filepath) {
            Ok(file) => file,
            Err(_) => return Err(DongoError::MapGeneratorError(0)),
        };

        let mut contents = String::new();
        if let Err(_) = file.read_to_string(&mut contents) {
            return Err(DongoError::MapGeneratorError(1));
        }

        let mut size: Option<(usize, usize)> = None;
        let mut vert_size: Option<(usize, usize)> = None;
        let mut num_verts: Option<usize> = None;
        let mut positions: Option<Vec<Vec3>> = None;
        let mut indeces: Option<Vec<u32>> = None;
        let mut colors: Option<Vec<Srgba>> = None;

        for line in contents.lines() {
            let mut parts = line.splitn(2, ": ");
            if let Some(field) = parts.next() {
                if let Some(value) = parts.next() {
                    match field.trim() {
                        "size" => {
                            let value = value.replace("(", "").replace(")", "").replace(" ", "");
                            let values: Vec<usize> =
                                value.split(',').filter_map(|s| s.parse().ok()).collect();
                            if values.len() == 2 {
                                size = Some((values[0], values[1]));
                            }
                        }
                        "vert_size" => {
                            let value = value.replace("(", "").replace(")", "").replace(" ", "");
                            let values: Vec<usize> =
                                value.split(',').filter_map(|s| s.parse().ok()).collect();
                            if values.len() == 2 {
                                vert_size = Some((values[0], values[1]));
                            }
                        }
                        "num_verts" => {
                            if let Ok(value) = value.trim().parse() {
                                num_verts = Some(value);
                            }
                        }
                        "positions" => {
                            let values: Vec<&str> = value.split(",").collect();
                            let mut temp_pos = Vec::<Vec3>::new();
                            for i in 0..values.len() / 3 {
                                let x: f32 = values[i * 3].parse().unwrap();
                                let y: f32 = values[i * 3 + 1].parse().unwrap();
                                let z: f32 = values[i * 3 + 2].parse().unwrap();
                                temp_pos.push(vec3(x, y, z));
                            }
                            positions = Some(temp_pos);
                        }
                        "indeces" => {
                            indeces =
                                Some(value.split(",").filter_map(|s| s.parse().ok()).collect());
                        }
                        "colors" => {
                            let values: Vec<&str> = value.split(",").collect();
                            let mut temp_col = Vec::<Srgba>::new();
                            for i in 0..values.len() / 4 {
                                let r: u8 = values[i * 4].parse().unwrap();
                                let g: u8 = values[i * 4 + 1].parse().unwrap();
                                let b: u8 = values[i * 4 + 2].parse().unwrap();
                                let a: u8 = values[i * 4 + 3].parse().unwrap();
                                temp_col.push(Srgba::new(r, g, b, a));
                            }
                            colors = Some(temp_col);
                        }
                        _ => {}
                    }
                }
            }
        }

        if let (
            Some(size),
            Some(vert_size),
            Some(num_verts),
            Some(positions),
            Some(indeces),
            Some(colors),
        ) = (size, vert_size, num_verts, positions, indeces, colors)
        {
            Ok(MapGenerator {
                size,
                vert_size,
                num_verts,
                positions,
                indices: indeces,
                colors,
            })
        } else {
            Err(DongoError::MapGeneratorError(2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorMessage;

    #[test]
    fn test_mapgen_to_and_from_file() {
        let mut mapgen = MapGenerator::new((10, 10));
        mapgen.define_parameters(ColorMode::HeightMap);

        mapgen.write_to_file();
        let maybe_mapgen_from_file = MapGenerator::read_from_file(MAPFILE_PATH);
        if let Err(e) = maybe_mapgen_from_file {
            panic!("Error reading from file: {}", e.error_message());
        }
        let mapgen_from_file = maybe_mapgen_from_file.unwrap();
        assert_eq!(mapgen.size, mapgen_from_file.size);
        assert_eq!(mapgen.vert_size, mapgen_from_file.vert_size);
        assert_eq!(mapgen.num_verts, mapgen_from_file.num_verts);
        assert_eq!(mapgen.positions, mapgen_from_file.positions);
        assert_eq!(mapgen.indices, mapgen_from_file.indices);
        assert_eq!(mapgen.colors, mapgen_from_file.colors);
    }
}
