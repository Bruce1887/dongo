use common::*;
use dongo::*;
use three_d::*;
use user_control::camera_controller;
use winit::event as wEvent;

pub fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window_builder = winit::window::WindowBuilder::new()
        .with_title(PROJECT_NAME)
        .with_min_inner_size(winit::dpi::LogicalSize::new(720, 720))
        .with_inner_size(winit::dpi::LogicalSize::new(720, 720));
    let window = window_builder.build(&event_loop).unwrap();

    // fullscreen and no cursor
    window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    window.set_cursor_visible(false);

    let context = WindowedContext::from_winit_window(
        &window,
        three_d::SurfaceSettings {
            vsync: false, // Wayland hangs in swap_buffers when one window is minimized or occluded
            ..three_d::SurfaceSettings::default()
        },
    )
    .unwrap();

    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        CAM_START_POS,
        CAM_START_TARGET,
        CAM_START_UP,
        CAM_START_FOV,
        CAM_START_Z_NEAR,
        CAM_START_Z_FAR,
    );

    let mut frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);

    // declare the entity manager. Like a big fat list of all entities
    let mut entities = DongoEntityManager::new();

    // ############ TERRAIN ############
    // let terrain_source = FilteredPerlinTerrainSource {
    //     perlin: noise::Perlin::new(MAP_SEED),
    //     noise_factor: MAP_PERLIN_NOISE_FACTOR,
    //     map_max_height: MAP_MAX_HEIGHT,
    //     map_min_height: MAP_MIN_HEIGHT,
    //     limiter: MAP_PERLIN_LIMITER,
    //     filter: Box::new(default_terrain_filter),
    // };
    let terrain_source = FlatTerrainSource { height: MAP_MIN_HEIGHT as f32 };

    let terrain_meta = DongoTerrainMetadata::new(terrain_source);
    let terrain_builder = TerrainBuilder::new(MAP_SIZE, MAP_VERTEX_DISTANCE);        
    let terrain_entity =
        terrain_builder.create_terrain_entity(&context, terrain_meta, MAP_COLOR_MODE);
    let terrain_id = entities.add_entity(terrain_entity);

    // ############ CUBE ############
    let cpu_mat = CpuMaterial::default();
    let mut phys_mat = PhysicalMaterial::new(&context, &cpu_mat);
    phys_mat.metallic = 2.0;

    let mut cube_trimesh = CpuMesh::cube();
    cube_trimesh.colors = Some(Vec::from([DONGOCOLOR_RED; 36]));
    let cube_gm = Gm::new(Mesh::new(&context, &cube_trimesh), phys_mat);
    let mut cube_entity = DongoEntity::from_gm(
        cube_gm,
        DongoMetadata::new(Some("cube"), vec![TAG_SELECTABLE]),
    );
    cube_entity.set_transform(Mat4::from_scale(50.0));
    cube_entity.set_pos(vec3(0.0, 0.0, 200.0));
    entities.add_entity(cube_entity);

    // ############ LIZZO ############
    // let mut croc_entity =
    //     DongoEntity::from_obj_file(&context, "Gator_Float", DongoMetadata::new_empty());

    let mut croc_entity = DongoEntity::from_gm(
        Gm::new(
            Mesh::new(&context, &cube_trimesh),
            PhysicalMaterial::new(&context, &cpu_mat),
        ),
        DongoMetadata::new_empty(),
    );
    croc_entity.set_transform(Mat4::from_scale(100.0));
    croc_entity.set_pos(vec3(0.0, 500.0, 600.0));
    let croc_id = entities.add_entity(croc_entity);


    let move_lizzo = |croc_id: ENTITYID,
                      terrain_id: ENTITYID,
                      entities: &mut DongoEntityManager,
                      camera: &Camera|
     -> bool {
        let terrain = entities.get_entity_by_id(terrain_id).unwrap();

        let croc_pos = entities.get_entity_by_id(croc_id).unwrap().pos().clone();
        let height = terrain.get_height_at(croc_pos.x, croc_pos.y);
        let direction = camera.position() - croc_pos;

        let mut new_pos = croc_pos + direction.normalize() * 0.5;
        new_pos.z = height + 300.0;

        let croc = entities.get_entity_by_id_mut(croc_id).unwrap();        
        
        let angle_to_camera = direction.y.atan2(direction.x); // Angle in radians

        // Extract the scale from the current transformation matrix
        let current_transform = croc.transform();

        // Add the scale to the transformation matrix
        let scale = Vector3::new(
            current_transform.x.x.abs(), // Scale on X-axis
            current_transform.y.y.abs(), // Scale on Y-axis
            current_transform.z.z.abs(), // Scale on Z-axis
        );
        let mut new_transform = Matrix4::<f32>::identity();        

        
        
        // ROTATION
        // new_transform =
        //     new_transform * Matrix4::from_angle_y(Rad(angle_to_camera));    

        // TRANSLATION
        new_transform = new_transform * Matrix4::from_translation(new_pos.into());        
        
        // SCALING
        new_transform = new_transform * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);
        

        dbg!(new_transform);
        // Set the new transformation matrix
        croc.set_transform(new_transform);
                
        true
    };

    // ############ LIGHTS ############
    let mut directional_light =
        renderer::light::DirectionalLight::new(&context, 0.7, Srgba::WHITE, &vec3(1.0, 1.0, -1.0));
    let ambient_light = renderer::light::AmbientLight::new(&context, 0.3, Srgba::WHITE);

    // ############ EVENT HANDLER ############
    let mut ev_handler = event_handler::EventHandler::new();

    let mut change: bool = true;
    event_loop.run(move |event, _, control_flow| match &event {
        wEvent::Event::MainEventsCleared => {
            window.request_redraw();
        }
        wEvent::Event::RedrawRequested(_) => {
            context.make_current().unwrap();
            let frame_input = frame_input_generator.generate(&context);

            camera.set_viewport(frame_input.viewport);
            change |= ev_handler.handle_events(
                &frame_input.events,
                &mut camera,
                &context,
                &mut entities,
                terrain_id,
            );

            entities
                .filter_to_entities_mut(|e| e.has_tag(TAG_HAS_ANIMATION))
                .iter_mut()
                .for_each(|e| {
                    e.animate(frame_input.accumulated_time as f32);
                });

            directional_light.generate_shadow_map(
                SHAHDOW_TEXTURE_SIZE,
                entities.filter_to_objects(|e| !e.has_tag(TAG_NO_LIGHT)),
            );

            change |= move_lizzo(croc_id, terrain_id, &mut entities, &camera);

            let all_objects = entities.get_objects();
            if change {
                // Get the screen render target to be able to render something on the screen
                frame_input
                    .screen()
                    // Clear the color and depth of the screen render target
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    .render(&camera, all_objects, &[&directional_light, &ambient_light]);

                context.swap_buffers().unwrap();
                control_flow.set_poll();
                window.request_redraw();
            }
            change = false;
        }
        wEvent::Event::DeviceEvent {
            event: device_event,
            ..
        } => match device_event {
            wEvent::DeviceEvent::MouseMotion { delta } => {
                camera_controller::look_around(&window, &mut camera, delta);
                change = true;
            }
            wEvent::DeviceEvent::Key(input) => {
                if input.virtual_keycode == Some(wEvent::VirtualKeyCode::C) {
                    window.set_cursor_visible(true);
                    change = true;
                } else if input.virtual_keycode == Some(wEvent::VirtualKeyCode::V) {
                    window.set_cursor_visible(false);
                    change = true;
                }
            }
            _ => {}
        },
        wEvent::Event::WindowEvent { event, .. } => {
            frame_input_generator.handle_winit_window_event(event);
            match event {
                wEvent::WindowEvent::Resized(physical_size) => {
                    context.resize(*physical_size);
                }
                wEvent::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    context.resize(**new_inner_size);
                }
                wEvent::WindowEvent::CloseRequested => {
                    context.make_current().unwrap(); // reckon this is always the current window, if so this is just an artefact from the example
                    control_flow.set_exit();
                }
                _ => (),
            }
        }
        _ => {}
    });
}
