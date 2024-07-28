use common::*;
use dongo::*;
use map_generator::*;
use three_d::*;

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Dongo!".to_string(),
        min_size: (10, 10),
        max_size: Some((1280, 720)),
        borderless: false,
        surface_settings: Default::default(),
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();
    context.set_cull(Cull::FrontAndBack);

    let mut camera = Camera::new_perspective(
        window.viewport(),
        CAM_START_POS,
        CAM_START_TARGET,
        CAM_START_UP,
        CAM_START_FOV,
        CAM_START_Z_NEAR,
        CAM_START_Z_FAR,
    );

    let mut objects = DongoObjectManager::new();

    let map_generator = MapGenerator::read_from_file(common::MAPFILE_PATH).unwrap();
    let map_obj = map_generator.generate(&context);

    objects.add_object_with_idx(MAP_ID,Box::new(map_obj), DongoObjectType::Map);
    

    let mut cube_trimesh = CpuMesh::cube();
    cube_trimesh.colors = Some(Vec::from([DONGOCOLOR_RED; 36]));

    cube_trimesh
        .transform(&Mat4::from_translation(vec3(
            0.0,
            0.0,
            MAP_MAX_HEIGHT as f32 + 1.0,
        )))
        .expect("Failed to transform cube");

    let cube_obj = Gm::new(
        Mesh::new(&context, &cube_trimesh),
        PhysicalMaterial::default(),
    );
    objects.add_object(Box::new(cube_obj), DongoObjectType::MapEntity);

    // tree
    let obj_path = "assets/low-poly-pinetree/massaged_low-poly-pinetree.obj";
    let mut loaded = three_d_asset::io::load(&[obj_path]).unwrap();
    let model = loaded.deserialize("low-poly-pinetree.obj").unwrap();
    let mut model_mat = three_d::Model::<PhysicalMaterial>::new(&context, &model).unwrap();
    model_mat.iter_mut().for_each(|m| {
        m.material.render_states.cull = Cull::Back;
        // m.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 210.0)));
        m.set_transformation(Mat4::from_scale(5.0));
        });
        
    objects.add_model(model_mat, DongoObjectType::MapEntity);



    let mut directional_light =
        renderer::light::DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(1.0, 0.0, -1.0));
    
    let ambient_light = renderer::light::AmbientLight::new(&context, 0.05, Srgba::WHITE);

    let mut ev_handler = event_handler::EventHandler::new();

    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
    {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);
 
        // Check for events
        ev_handler.handle_events(&frame_input.events, &mut camera, &context, &mut objects);

        let obj_vec = objects.get_objects_vec(|o: &DongoObject| o.get_type() != &DongoObjectType::Selection);

        directional_light.generate_shadow_map(2048, &obj_vec);

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera, objects.get_objects_vec(no_predicate), &[&directional_light,&ambient_light]
            );
        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}
