use common::*;
use dongo::*;
use dongo_object::*;
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

    let mut objects = DongoObjectManager::new();

    let model: three_d_asset::Model =
    three_d_asset::io::load_and_deserialize("assets/low-poly-tree/source/tree_2.obj").expect("Failed loading asset");
    
    //objects.add_object(0, Box::new(tree_gm), DongoObjectType::Map);

    let mut directional_light =
        renderer::light::DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(1.0, 0.0, -1.0));

    let ambient_light = renderer::light::AmbientLight::new(&context, 0.05, Srgba::WHITE);

    let mut ev_handler = event_handler::EventHandler::new();

    window.render_loop(
        move |frame_input|
    {
        camera.set_viewport(frame_input.viewport);
 
        ev_handler.handle_events(&frame_input.events, &mut camera, &context, &mut objects);

        let obj_vec = objects.get_vec(|o: &DongoObject| o.get_type() != &DongoObjectType::Selection);

        directional_light.generate_shadow_map(512, &obj_vec);

        let obj_vec = objects.get_vec(no_predicate);

        frame_input.screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera, obj_vec, &[&directional_light,&ambient_light]
            );
        FrameOutput::default()
    },
    );
}
