use three_d::*;


const CAMERA_MIN_DIST: f32 = 20.0;
const CAMERA_MAX_DIST: f32 = 1000.0;
const CAMERA_MOVE_SPEED: f32 = 2.0;
const CAMERA_SHIFT_FACTOR: f32 = 2.0;

pub fn handle_events(events: &Vec<Event>, camera: &mut Camera){
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
                if *kind == Key::W || *kind == Key::A || *kind == Key::S || *kind == Key::D {                                    
                    println!("move camera event: {:?}", std::time::Instant::now());                    
                    handle_camera_event(camera, ev.to_owned());
                }
            }
            Event::MouseWheel {delta: _, position: _, modifiers: _, handled: _} => {
                handle_camera_event(camera, ev.to_owned())
            }
            _ => (),
        }        
    }    
}

fn handle_camera_event(camera: &mut Camera, e :Event){
    match e {
        // zoom in and out
        Event::MouseWheel {delta, position: _, modifiers: _, handled: _} => {
            dbg!(delta);
            dbg!(camera.position());                                                    
            let mut pos_clone = camera.position().clone();
            let target_clone = camera.target().clone();
            let up_clone = camera.up().clone();

            pos_clone.z -= delta.1; // delta.1 is positive when scrolling "up" (zooming in)
            pos_clone.z = pos_clone.z.clamp(CAMERA_MIN_DIST, CAMERA_MAX_DIST);
                        
            camera.set_view(pos_clone, target_clone, up_clone);
        }
        Event::KeyPress {
            kind,
            modifiers,
            handled: _,
        } => {            
            if kind == Key::W || kind == Key::A || kind == Key::S || kind == Key::D {
                let mut direction = Vec3::new(0.0, 0.0, 0.0);
                if kind == Key::W{
                    direction.y += 1.0;
                }
                else if kind == Key::A{
                    direction.x -= 1.0;
                }
                else if kind == Key::S{
                    direction.y -= 1.0;
                }
                else if kind == Key::D{
                    direction.x += 1.0;
                }                
                let speed = if modifiers.shift { CAMERA_MOVE_SPEED * CAMERA_SHIFT_FACTOR } else { CAMERA_MOVE_SPEED };
                move_camera(camera, direction, speed);                
            }
            else { panic!("KeyPress is not a camera event") }   
        }
        _ => panic!("not a camera event")
    }
}

fn move_camera(camera: &mut Camera, direction: Vec3, speed: f32){
    let mut pos_clone = camera.position().clone();
    let mut target = pos_clone;
    target.z = 0.0;
    
    let up_clone = camera.up().clone();

    pos_clone += direction * speed;
    
    camera.set_view(pos_clone, target, up_clone);
}
