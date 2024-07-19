use std::{sync::Arc, vec};

use three_d::*;
mod event_handler;
mod map_generator;
use map_generator::*;



pub fn main() {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Dongo!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    
    // Get the graphics context from the window
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 90.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    );    

    const MAP_SIZE: (usize,usize) = (100,100);
    let map_generator = MapGenerator::new(MAP_SIZE);
    let map_model = map_generator.generate(ColorMode::Height,&context);        

    let models = Arc::new(vec![map_model]);
    
    // Start the main render loop
    window.render_loop(
        move |frame_input| // Begin a new frame with an updated frame input
    {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport);

        // Check for events
        event_handler::handle_events(&frame_input.events);                      

        let objects = Arc::clone(&models);
        // Get the screen render target to be able to render something on the screen
        frame_input.screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render(
                &camera, (*objects).iter(), &[]
            );            
        // Returns default frame output to end the frame
        FrameOutput::default()
    },    
    );
}