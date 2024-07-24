use three_d::*;

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Cull test!".to_string(),
        min_size: (10, 10),
        max_size: Some((1280, 720)),
        borderless: false,
        surface_settings: Default::default(),
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl();
    
    context.set_cull(Cull::FrontAndBack);

    dbg!(&context);

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 20.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut cube_trimesh = CpuMesh::cube();
    cube_trimesh.colors = Some(Vec::from([Srgba::RED; 36]));
    let cube_obj = Gm::new(Mesh::new(&context, &cube_trimesh), PhysicalMaterial::default());

    
    // Create a CPU-side mesh consisting of a single colored triangle
    let positions = vec![
        vec3(5.0, -5.0, 5.0),  // bottom right
        vec3(-5.0, -5.0, 5.0), // bottom left
        vec3(0.0, 5.0, 5.0),   // top
    ];
    dbg!(vec3(0.5, -0.5, 0.0).cross(vec3(-0.5, -0.5, 0.0)));

    let colors = vec![
        Srgba::BLUE,   // bottom right
        Srgba::BLUE, // bottom left
        Srgba::BLUE,  // top
    ];
    let mut triangle_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };
    triangle_mesh.compute_normals();
    
    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    let blue_triangle = Gm::new(Mesh::new(&context, &triangle_mesh), PhysicalMaterial::default());

    let mut directional_light =
    renderer::light::DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(1.0, 0.0, -1.0));    
    let ambient_light = renderer::light::AmbientLight::new(&context, 0.05, Srgba::WHITE);
    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
    {
        dbg!(&context);

        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        directional_light.generate_shadow_map(128, &cube_obj);
        //unsafe {context.cull_face(Cull::FrontAndBack as u32);}
        for ev in &frame_input.events{          
            if let Event::KeyPress { kind, modifiers, handled: _ } =  ev {
                if *kind == Key::W && modifiers.ctrl {
                    println!("Ctrl + W pressed. Exiting application...");
                    std::process::exit(0);   
                }
                match *kind {
                    Key::W => {
                        let mut pos = camera.position().clone();
                        pos.z += 3.0;
                        let target = camera.target().clone();
                        let up = camera.up().clone();
                        camera.set_view(pos, target, up)
                    },
                    Key::S => {
                        let mut pos = camera.position().clone();
                        pos.z += -3.0;
                        let target = camera.target().clone();
                        let up = camera.up().clone();
                        camera.set_view(pos, target, up)
                    },
                    _ =>(),
                }
            }
        };
        

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera, &[&cube_obj,&blue_triangle], &[&directional_light,&ambient_light]
            );
        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}