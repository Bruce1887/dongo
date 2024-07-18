use three_d::*;

/*
const CAMERA_MIN_DIST: f32 = 0.1;
const CAMERA_MAX_DIST: f32 = 100.0;

pub fn zoom_camera(mut cam: Camera, delta: f32) {
    let mut zoom_target = cam.position().clone();
    zoom_target.y = 0.0;
    cam.zoom_towards(&zoom_target, delta, CAMERA_MIN_DIST, CAMERA_MAX_DIST)
}
*/

pub fn handle_events(events: &Vec<Event>) {
    for ev in events {
        //dbg!(ev);
        match ev {
            Event::KeyPress {
                kind,
                modifiers,
                handled,
            } => {
                if *kind == Key::W && modifiers.ctrl && !*handled {
                    println!("Ctrl + W pressed. Exiting application...");
                    std::process::exit(0);
                }
            }
            Event::MouseWheel {
                delta,
                position,
                modifiers,
                handled,
            } => {
                dbg!(delta, position, modifiers, handled);
                
                // maybe_cam.unwrap().map(|mut cam| {
                //     let mut zoom_target = cam.position().clone();
                //     zoom_target.y = 0.0;
                //     cam.zoom_towards(&zoom_target, delta.0, CAMERA_MIN_DIST, CAMERA_MAX_DIST)
                // });
            }
            _ => (),
        }
    }
}
