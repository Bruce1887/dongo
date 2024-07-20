use common::{
    CAM_START_FOV, CAM_START_POS, CAM_START_TARGET, CAM_START_UP, CAM_START_Z_FAR, CAM_START_Z_NEAR, MAP_MAX_HEIGHT,
};
use dongo::*;
//use std::{sync::Arc, vec};

use map_generator::*;
use three_d::*;

pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Dongo!".to_string(),
        max_size: Some((1280, 720)),
        // borderless: true,
        ..Default::default()
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
    // let mut models: Arc<Vec<Gm<Mesh, ColorMaterial>>> = Arc::new(vec![]);

    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    
    let map_generator = MapGenerator::read_from_file(common::MAPFILE_PATH).unwrap();
    // let map_generator = MapGenerator::new((256, 256));
    let map_obj = map_generator.generate(&context);    
    //Arc::get_mut(&mut models).unwrap().push(map_model);
    
    objects.push(Box::new(map_obj));

    let mut cube_trimesh = CpuMesh::cube();
    cube_trimesh.colors = Some(Vec::from([Srgba::RED; 36]));

    cube_trimesh
        .transform(&Mat4::from_translation(vec3(0.0, 0.0, MAP_MAX_HEIGHT as f32 + 1.0)))
        .expect("Failed to transform cube");

    let cube_obj = Gm::new(Mesh::new(&context, &cube_trimesh), PhysicalMaterial::default());
    
    // Arc::get_mut(&mut models).unwrap().push(cube_model);
    objects.push(Box::new(cube_obj));

    let mut directional_light =
        renderer::light::DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(1.0, 0.0, -1.0));
    directional_light.generate_shadow_map(32, objects.iter().map(|obj| &**obj).collect::<Vec<&dyn Object>>());

    let ambient_light = renderer::light::AmbientLight::new(&context, 0.05, Srgba::WHITE);

    let mut ev_handler = event_handler::EventHandler::new();
    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
    {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        // Check for events
        ev_handler.handle_events(&frame_input.events, &mut camera);
        let obj_vec = objects.iter().map(|obj| &**obj).collect::<Vec<&dyn Object>>();

        directional_light.generate_shadow_map(128, &obj_vec);

        //let objects = Arc::clone(&models); // do i have to clone here? 

        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, obj_vec, &[&directional_light,&ambient_light]
            );
        // Returns default frame output to end the frame
        FrameOutput::default()
    },
    );
}