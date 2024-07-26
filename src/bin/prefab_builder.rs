const DATA_NAME: &str = "low-poly-pinetree";

fn main() {
    let data_to_massage_path = format!("assets/{}/{}.obj", DATA_NAME,DATA_NAME);
    let massaged_data_path = format!("assets/{}/massaged_{}.obj", DATA_NAME,DATA_NAME);
    
    dongo::data_massage_parlor::data_massage::center_obj_vertices(data_to_massage_path.as_str(), massaged_data_path.as_str()).unwrap();
    dongo::data_massage_parlor::data_massage::resize_obj_vertices(data_to_massage_path.as_str(), massaged_data_path.as_str()).unwrap();
    run();
}

use three_d::*;

pub fn run() {
    let window = Window::new(WindowSettings {
        title: "prefab_builder!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(4.0, 4.0, 20.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 10000.0);

    let ambient = AmbientLight::new(&context, 0.4, Srgba::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Srgba::WHITE, &vec3(-1.0, -1.0, -1.0));

    let obj_path = format!("assets/{}/massaged_{}.obj", DATA_NAME,DATA_NAME);
    let mut loaded = three_d_asset::io::load(&[obj_path.as_str()]).unwrap();

    let model = loaded.deserialize(format!("{}.obj",DATA_NAME)).unwrap();

    let mut material_model = three_d::Model::<PhysicalMaterial>::new(&context, &model).unwrap();
    material_model
        .iter_mut()
        .for_each(|m| m.material.render_states.cull = Cull::None);

    // main loop
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);
        change |= control.handle_events(&mut camera, &mut frame_input.events);

        for event in &frame_input.events {
            if let &Event::KeyPress { kind , modifiers, handled: _ } = &event {
                if *kind == Key::W && modifiers.ctrl {
                    std::process::exit(0);
                }
            }
        }

        // draw
        if change {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(
                    &camera,
                    &material_model,
                    &[&ambient, &directional],
                );
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}
