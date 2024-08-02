use three_d::*;

pub fn create_box_positions(start: Vec3, end: Vec3) -> [Vec3; 8] {
    // if start.z > end.z { // this looks weird// this looks weird if if selection intersects the terrain
    // }
    // else {
    //     end.z += SELECTION_HEIGHT_EXTRA;
    // }
    let vertices: [Vec3; 8] = [
        //     start.z += SELECTION_HEIGHT_EXTRA;
        start,                         //0 front bot-left
        vec3(start.x, start.y, end.z), //1 front top-left
        vec3(end.x, start.y, end.z),   //2 front top-right
        vec3(end.x, start.y, start.z), //3 front bot-right
        end,                           //4 back top-left
        vec3(start.x, end.y, end.z),   //5 back top-right
        vec3(start.x, end.y, start.z), //6 back bot-right
        vec3(end.x, end.y, start.z),   //7 back bot-left
    ];
    vertices
}
pub fn create_box_trimesh(start: Vec3, end: Vec3, color: Srgba) -> CpuMesh {
    let vertices = create_box_positions(start, end);
    let indices: [u32; 30] = [
        // Front face
        0, 2, 1, 0, 3, 2, // Right face
        3, 4, 2, 3, 7, 4, // Back face
        7, 5, 4, 7, 6, 5, // Left face
        6, 1, 5, 6, 0, 1, // Top face
        2, 5, 1, 2, 4,
        5,
        // Bottom face // dont bother with bottom, since it is not visible
        // 1, 5, 6,
        // 6, 2, 1,
    ];

    let colors: [Srgba; 8] = [color; 8];

    CpuMesh {
        positions: Positions::F32(vertices.to_vec()),
        colors: Some(colors.to_vec()),
        indices: Indices::U32(indices.to_vec()),
        //normals: Some(self.normals),
        ..Default::default()
    }
}

// start is either bottom or top point, depending on the sign of height
// height determines the pointiness of the pyramid
pub fn create_marker_positions(height: f32, top_side: f32) -> [Vec3; 5] {
    const START: Vec3 = vec3(0.0, 0.0, 0.0);
    let vertices: [Vec3; 5] = [
        START,
        vec3(
            START.x,
            START.y - top_side / 2.0,
            START.z + height,
        ),
        vec3(
            START.x + top_side / 2.0,
            START.y,
            START.z + height,
        ),
        vec3(
            START.x,
            START.y + top_side / 2.0,
            START.z + height,
        ),
        vec3(
            START.x - top_side / 2.0,
            START.y,
            START.z + height,
        ),
    ];
    vertices
}

pub fn create_marker_trimesh(height: f32, width: f32, color: Srgba) -> CpuMesh {
    let vertices = create_marker_positions(height, width);
    let indices: [u32; 18] = [0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 1, 4, 1, 2, 3, 1, 3, 4];

    let colors: [Srgba; 5] = [color; 5];

    CpuMesh {
        positions: Positions::F32(vertices.to_vec()),
        colors: Some(colors.to_vec()),
        indices: Indices::U32(indices.to_vec()),
        //normals: Some(self.normals),
        ..Default::default()
    }
}
