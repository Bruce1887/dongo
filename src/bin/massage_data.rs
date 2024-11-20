use dongo::data_massage_parlor::data_massage::*;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <data_name>", args[0]);
        std::process::exit(1);
    }
    let data_name = &args[1];
    println!("Massaging data...");
    
    let data_to_massage_path = format!("assets/{}/{}.obj", data_name, data_name);
    let massaged_data_path = format!("assets/{}/massaged_{}.obj", data_name, data_name);
    println!("DATA_NAME {data_name}\n");
    
    println!("Centering vertices...");
    center_obj_vertices(
        data_to_massage_path.as_str(),
        massaged_data_path.as_str(),
    )
    .unwrap();

    println!("Resizing vertices...");
    resize_obj_vertices(
        massaged_data_path.as_str(),
        massaged_data_path.as_str(),
    )
    .unwrap();

    println!("Rotating vertices...");
    rotate_obj_vertices(
        massaged_data_path.as_str(),
        massaged_data_path.as_str(),
        Axis::X,
        90.0
    )
    .unwrap();

    println!("\nData massaged: {massaged_data_path}");

    run(&data_name);
}

use three_d::*;

pub fn run(data_name: &str) {
    let window = Window::new(WindowSettings {
        title: "massage!".to_string(),
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
    let directional_2 = DirectionalLight::new(&context, 0.1, Srgba::WHITE, &vec3(1.0, 1.0, 1.0));
    let obj_path = format!("assets/{}/massaged_{}.obj", data_name, data_name);
    let mut loaded = three_d_asset::io::load(&[obj_path.as_str()]).unwrap();

    let model = loaded.deserialize(format!("{}.obj", data_name)).unwrap();

    let model_mat = three_d::Model::<PhysicalMaterial>::new(&context, &model).unwrap();

    let mut model_mat_2 = three_d::Model::<PhysicalMaterial>::new(&context, &model).unwrap();
    model_mat_2.iter_mut().for_each(|m| {
        m.material.render_states.cull = Cull::Front;
        m.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 10.0)));
    });

    // main loop
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);
        change |= control.handle_events(&mut camera, &mut frame_input.events);

        for event in &frame_input.events {
            if let &Event::KeyPress {
                kind,
                modifiers,
                handled: _,
            } = &event
            {
                if *kind == Key::W && modifiers.ctrl {
                    std::process::exit(0);
                }
            }
        } 

        let mut _empty_vec: Vec<&mut dyn three_d::Object> = vec![];
        let models_iter = model_mat.iter().chain(model_mat_2.iter());
        // draw
        if change {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(&camera, models_iter, &[&ambient, &directional, &directional_2]);
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}

