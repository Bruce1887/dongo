use three_d::*;

// Generate a mesh with a checkerboard pattern
pub fn generate(size: (usize,usize),context: &Context) -> Gm<Mesh,ColorMaterial> {    
    assert!(size.0 % 2 == 0 && size.1 % 2 == 0); // Ensure the size is even so middle is at (0,0)
    assert!(size.0 > 0 && size.1 > 0); 

    let vert_size = (size.0 + 1,size.1 + 1); // Vertices are one more than the size
    let num_verts = vert_size.0 * vert_size.1;

    // generate positions of the vertices
    let mut positions: Vec<Vec3> = Vec::with_capacity(num_verts);
    for y in 0..vert_size.1 {
        for x in 0..vert_size.0 {
            positions.push(vec3(x as f32 - size.0 as f32 / 2f32, y as f32 - size.1 as f32 / 2f32, 0f32));
        }
    }
    // dbg!(&positions);

    // // Create a CPU-side mesh consisting of a single colored triangle
    // let positions = vec![
    //     vec3(1f32, -1f32, 0f32),  // bottom right
    //     vec3(-1f32, -1f32, 0f32), // bottom left
    //     vec3(1f32, 1f32, 0f32),   // top right
    //     vec3(-1f32, 1f32, 0f32),   // top left
    // ];
    
    // define the indices of the vertices
    let mut indeces: Vec<u32> = Vec::with_capacity(num_verts);
    for y in 0..vert_size.1 {
        for x in 0..vert_size.0 {
            let i = y * vert_size.0 + x;
            if x < vert_size.0 - 1 && y < vert_size.1 - 1 {
                // lower triangle of square
                indeces.push(i as u32); // bottom left
                indeces.push((i + 1) as u32); // bottom right
                indeces.push((i + vert_size.0) as u32); // top left

                // upper triangle of square
                indeces.push((i + vert_size.0) as u32); // top left
                indeces.push((i + 1) as u32); // bottom right 
                indeces.push((i + vert_size.0 + 1) as u32); // top right
            }
        }
    }

    // let indices: Vec<u32> = vec![
    //     0, 1, 2,
    //     2, 1, 3,
    // ];

    
    let mut colors: Vec<Srgba> = Vec::with_capacity(num_verts);
    for i in 0..num_verts {
        
        let color = if i % 2 == 0 {
            Srgba::RED
        } else {
            Srgba::BLUE
        };
        colors.push(color);
    }

    // let colors = vec![
    //     Srgba::BLACK,   // bottom right
    //     Srgba::RED, // bottom left
    //     Srgba::BLUE,  // top right
    //     Srgba::WHITE, // top left
    // ];

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        indices: Indices::U32(indeces),
        ..Default::default()
    };
    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    return Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions() {

    }
}