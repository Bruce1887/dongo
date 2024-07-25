use crate::dongo_object::*;
use three_d::*;

const SELECTION_COLOR: Srgba = Srgba::new(255, 255, 0, 150);
const SELECTION_HEIGHT_EXTRA: f32 = 10.0;

pub fn resize_selection(
    objects: &mut DongoObjectManager,
    start: Vec3,
    end: Vec3,
    context: &Context,
) {
    objects.take_obj(SELECTION_IDX);

    let cube_trimesh = create_selection_trimesh(start, end);

    CpuMesh::cube();
    let selection_mesh = Gm::new(
        Mesh::new(&context, &cube_trimesh),
        ColorMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: SELECTION_COLOR,
                ..Default::default()
            },
        ),
    );

    objects.add_object(
        SELECTION_IDX,
        Box::new(selection_mesh),
        DongoObjectType::Selection,
    )
}

fn create_selection_trimesh(mut start: Vec3, mut end: Vec3) -> CpuMesh {
    start.z = crate::common::MAP_MIN_HEIGHT as f32;
    end.z = crate::common::MAP_MAX_HEIGHT as f32 + SELECTION_HEIGHT_EXTRA;
    let vertices: [Vec3; 8] = [
        start,                         //0 front bot-left
        vec3(start.x, start.y, end.z), //1 front top-left
        vec3(end.x, start.y, end.z),   //2 front top-right
        vec3(end.x, start.y, start.z), //3 front bot-right
        end,                           //4 back top-left
        vec3(start.x, end.y, end.z),   //5 back top-right
        vec3(start.x, end.y, start.z), //6 back bot-right
        vec3(end.x, end.y, start.z),   //7 back bot-left
    ];

    let indices: [u32; 30] = [
        // Front face
        0, 2, 1, 0, 3, 2, // Right face
        3, 4, 2, 3, 7, 4, // Back face
        7, 5, 4, 7, 6, 5, // Left face
        6, 1, 5, 6, 0, 1, // Top face
        2, 5, 1, 2, 4, 5,
        // Bottom face
        // 1, 5, 6,
        // 6, 2, 1,
    ];

    let colors: [Srgba; 36] = [SELECTION_COLOR; 36];

    CpuMesh {
        positions: Positions::F32(vertices.to_vec()),
        colors: Some(colors.to_vec()),
        indices: Indices::U32(indices.to_vec()),
        //normals: Some(self.normals),
        ..Default::default()
    }
}
