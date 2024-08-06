use crate::*;
use three_d::*;

use noise::{NoiseFn, Perlin, Seedable};
pub struct DongoTerrainMetadata {
    perlin_noise_factor: f64, // higher value equals more mountains and valleys // TODO: Split into x and y factor, to allow for different noise in x and y direction (valleys, ravines kind of stuff)
    perlin: Perlin,
    map_max_height: f64,
    map_min_height: f64,
}

impl DongoTerrainMetadata {
    pub fn new(seed: u32, perlin_noise_factor: f64, max_height: f64, min_height:f64) -> DongoTerrainMetadata {
        DongoTerrainMetadata {
            perlin_noise_factor,
            perlin: Perlin::new(seed),
            map_max_height: max_height,
            map_min_height: min_height,
        }
    }

    pub fn _seed(&self) -> u32 {
        self.perlin.seed()
    }

    pub fn get_height_at(&self, x: f64, y: f64) -> f64 {
        let noise_value =
        self.perlin.get([x * self.perlin_noise_factor, y * self.perlin_noise_factor]); // returns a value between -1 and 1
        let normalized_value = (noise_value + 1.0) / 2.0; // set value between 0 and 1
        let height = normalized_value * (self.map_max_height - self.map_min_height) + self.map_min_height;
        height
    }
}


// data needed to create a terrain entity
pub struct TerrainData {
    square_tuple: (usize, usize),
    verts_tuple: (usize, usize),
    num_verts: usize,
    vec_distance: f32,
    positions: Vec<Vec3>,
    colors: Vec<Srgba>,
    indices: Vec<u32>,
}

impl TerrainData {
    pub fn new(size : (usize, usize), vec_distance: f32) -> TerrainData {
        assert!(size.0 % 2 == 0 && size.1 % 2 == 0); // Ensure the size is even so middle is at (0,0)
        assert!(size.0 > 0 && size.1 > 0);

        let v_size = (size.0 + 1, size.1 + 1);
        let num_v = (size.0 + 1) * (size.1 + 1);
        TerrainData {
            square_tuple: size,
            verts_tuple: v_size,
            num_verts: num_v,
            vec_distance,
            positions: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
        }
    }
    
    fn define_positions(&mut self, t_meta: &DongoTerrainMetadata) {
        let mut lowest_elevation = f64::MAX;
        let mut highest_elevation = f64::MIN;

        for y in 0..self.verts_tuple.1 {
            for x in 0..self.verts_tuple.0 {
                crate::common::print_loading_indicator(
                    (y * self.verts_tuple.0 + x + 1) as f32,
                    self.num_verts as f32,
                );

                let nx = x as f64 / self.verts_tuple.0 as f64;
                let ny = y as f64 / self.verts_tuple.1 as f64;

                let height = t_meta.get_height_at(nx, ny);
                
                if height < lowest_elevation {
                    lowest_elevation = height;
                }
                if height > highest_elevation {
                    highest_elevation = height;
                }

                let pos_x = (x as f32 - self.square_tuple.0 as f32 / 2.0) * self.vec_distance;
                let pos_y = (y as f32 - self.square_tuple.1 as f32 / 2.0) * self.vec_distance;

                self.positions.push(vec3(pos_x, pos_y, height as f32));
            }
        }
        dbg!(highest_elevation, lowest_elevation);
    }

    fn define_indences(&mut self) {
        // define the indices of the vertices
        for y in 0..self.verts_tuple.1 {
            for x in 0..self.verts_tuple.0 {
                let i = y * self.verts_tuple.0 + x;
                if x < self.verts_tuple.0 - 1 && y < self.verts_tuple.1 - 1 {
                    // lower triangle of square
                    self.indices.push((i + 1) as u32); // bottom right
                    self.indices.push(i as u32); // bottom left
                    self.indices.push((i + self.verts_tuple.0) as u32); // top left

                    // upper triangle of square
                    self.indices.push((i + 1) as u32); // bottom right
                    self.indices.push((i + self.verts_tuple.0) as u32); // top left
                    self.indices.push((i + self.verts_tuple.0 + 1) as u32); // top right
                }
            }
        }
    }

    fn paint_my_mesh(&mut self, colormode: ColorMode, t_meta: &DongoTerrainMetadata) {
    match colormode {
            ColorMode::HeightMap => {
                // color the vertices based on their height
                for i in 0..self.num_verts {
                    let height_range = t_meta.map_max_height - t_meta.map_min_height;
                    let height = self.positions[i].z;
                    if height < (height_range / 4.0) as f32 {
                        // lowest 25% of the height range
                        self.colors.push(DONGOCOLOR_GRAY);
                    } else if height < (height_range / 2.0) as f32 {
                        // 25% to 50% of the height range
                        self.colors.push(DONGOCOLOR_BLUE);
                    } else if height < (3.0 * height_range / 4.0) as f32 {
                        // 50% to 75% of the height range
                        self.colors.push(DONGOCOLOR_GREEN);
                    } else {
                        // highest 25% of the height range
                        self.colors.push(DONGOCOLOR_WHITE);
                    }
                }
            }
            ColorMode::Checkerboard => {
                // strange symmetrical color pattern, kinda useful
                for i in 0..self.verts_tuple.0 {
                    for j in 0..self.verts_tuple.1 {
                        let color = if i % 2 == 0 && j % 2 == 0 {
                            DONGOCOLOR_BLACK
                        } else if i % 2 == 0 && j % 2 == 1 {
                            DONGOCOLOR_RED
                        } else if i % 2 == 1 && j % 2 == 0 {
                            DONGOCOLOR_BLUE
                        } else {
                            DONGOCOLOR_WHITE
                        };
                        self.colors.push(color);
                    }
                }
            }
        }
    }

    /// creates a terrain entity 
    /// consumes self
    pub fn create_terrain_entity(mut self,context: &Context,t_meta: DongoTerrainMetadata, colormode: ColorMode) -> crate::DongoEntity {
        self.define_positions(&t_meta);
        self.define_indences();
        self.paint_my_mesh(colormode, &t_meta);

        let mut cpu_mesh = CpuMesh {
            positions: Positions::F32(self.positions),
            colors: Some(self.colors),
            indices: Indices::U32(self.indices),
            ..Default::default()
            };
            cpu_mesh.compute_normals();
            let gm = Gm::new(Mesh::new(context, &cpu_mesh), PhysicalMaterial::default());
            let meta = crate::DongoMetadata::new(Some("The map metadata"), vec![crate::TAG_MAP]);
        
            return crate::DongoEntity::Terrain(Box::new(gm), meta, None,t_meta)
    }
}

