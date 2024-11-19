use common::*;
use dongo::*;
// use noise::Perlin;
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

    let mut camera = Camera::new_perspective(
        window.viewport(),
        CAM_START_POS,
        CAM_START_TARGET,
        CAM_START_UP,
        CAM_START_FOV,
        CAM_START_Z_NEAR,
        CAM_START_Z_FAR,
    );

    // declare the entity manager. Like a big fat list of all entities
    let mut entities = DongoEntityManager::new();    
    
    // ############ TERRAIN ############
    let terrain_builder = TerrainBuilder::new(MAP_SIZE, MAP_VERTEX_DISTANCE);
    // let terrain_meta = DongoTerrainMetadata::new(DongoTerrainSource::Perlin(Perlin::new(MAP_SEED), MAP_PERLIN_NOISE_FACTOR, MAP_MIN_HEIGHT, MAP_MAX_HEIGHT, MAP_PERLIN_LIMITER));    
    let terrain_meta = DongoTerrainMetadata::new(DongoTerrainSource::Flat);
    let terrain_entity = terrain_builder.create_terrain_entity(&context, terrain_meta, MAP_COLOR_MODE);
    entities.add_entity(terrain_entity);
    
    // ############ CUBE ############
    let mut cube_trimesh = CpuMesh::cube();
    cube_trimesh.colors = Some(Vec::from([DONGOCOLOR_RED; 36]));
    let cube_gm = Gm::new(
        Mesh::new(&context, &cube_trimesh),
        PhysicalMaterial::default(),
    );
    let mut cube_entity = DongoEntity::from_gm(cube_gm, DongoMetadata::new(Some("cube"), vec![TAG_SELECTABLE]));
    cube_entity.set_transform(Mat4::from_scale(20.0));
    cube_entity.set_pos(vec3(0.0, MAP_SIZE.0 as f32, 10.0));
    entities.add_entity(cube_entity);

    // ############ TREE ############
    let mut tree_entity = DongoEntity::from_obj_file(&context, "low-poly-pinetree", DongoMetadata::new(Some("tree"), vec![TAG_SELECTABLE]));
    tree_entity.set_transform(Mat4::from_scale(8.0));
    tree_entity.set_pos(vec3(20.0, 0.0, MAP_MAX_HEIGHT as f32 + 10.0));
    entities.add_entity(tree_entity);

    // ############ LIGHTS ############
    let mut directional_light =
        renderer::light::DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(2.0, 0.0, -1.0));
    let ambient_light = renderer::light::AmbientLight::new(&context, 0.05, Srgba::WHITE);

    // ############ EVENT HANDLER ############
    let mut ev_handler = event_handler::EventHandler::new();
    
    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
    {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);
        ev_handler.handle_events(&frame_input.events, &mut camera, &context, &mut entities);

        entities.filter_to_entities_mut(|e| e.has_tag(TAG_HAS_ANIMATION)).iter_mut().for_each(|e| {
            e.animate(frame_input.accumulated_time as f32);
        });

        directional_light.generate_shadow_map(2048,entities.filter_to_objects(|e| !e.has_tag(TAG_NO_LIGHT)));
        
        let all_objects = entities.get_objects();

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera, all_objects, &[&directional_light,&ambient_light]
            );
        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}