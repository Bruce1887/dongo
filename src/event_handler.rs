use crate::common::*;
use three_d::*;

#[derive(Default)]
pub struct EventHandler {
    wasd_down: (bool, bool, bool, bool), // camera movement
    qe_down: (bool, bool), // camera rotation 
    shift_down: bool,
    ctrl_down: bool,
    alt_down: bool,
    // cmd_down: bool, // command is the ctrl key on windows and linux
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            ..std::default::Default::default()
        }
    }

    pub fn handle_events(&mut self, events: &Vec<Event>, camera: &mut Camera) {
        for ev in events {
            //dbg!(ev);
            match ev {
                Event::ModifiersChange { modifiers } => {
                    // update modifier fields in struct
                    self.shift_down = modifiers.shift;
                    self.ctrl_down = modifiers.ctrl;
                    self.alt_down = modifiers.alt;
                }
                Event::KeyPress {
                    kind,
                    modifiers,
                    handled: _,
                } => {
                    self.check_keys_down(ev);

                    if *kind == Key::W && modifiers.ctrl {
                        println!("Ctrl + W pressed. Exiting application...");
                        std::process::exit(0);
                    }

                    if *kind == Key::ArrowUp{
                        Self::zoom_camera(camera, &(28.0,28.0))
                    }
                    
                    if *kind == Key::ArrowDown{
                        Self::zoom_camera(camera, &(-28.0,-28.0))
                    }

                }
                Event::KeyRelease {
                    kind: _,
                    modifiers: _,
                    handled: _,
                } => {
                    self.check_keys_down(ev);
                }
                Event::MouseWheel {
                    delta,
                    position: _,
                    modifiers: _,
                    handled: _,
                } => Self::zoom_camera(camera, delta),
                _ => (),
            }
        }        
        
        if self.wasd_down.0 || self.wasd_down.1 || self.wasd_down.2 || self.wasd_down.3 {
            self.move_camera(camera);
        }
        if self.qe_down.0 || self.qe_down.1 {
            self.rotate_camera(camera);
        }
    }    

    fn check_keys_down(&mut self, ev: &Event) {
        let value: bool;
        let key: Key;
        if let Event::KeyPress { kind, modifiers: _, handled: _ } = ev {
            key = *kind;
            value = true;
        } else if let Event::KeyRelease { kind, modifiers: _, handled: _ } = ev {
            key = *kind;
            value = false;
        } else {
            panic!("Event is not a key event (KeyPress or KeyRelease)");
        }

        // camera movement
        match key {
            Key::W => self.wasd_down.0 = value,
            Key::A => self.wasd_down.1 = value,
            Key::S => self.wasd_down.2 = value,
            Key::D => self.wasd_down.3 = value,
            Key::Q => self.qe_down.0 = value,
            Key::E => self.qe_down.1 = value,
            _ => (),
        }
    }

    fn zoom_camera(camera: &mut Camera, delta: &(f32, f32)) {
        let mut pos_clone = camera.position().clone();
        let target_clone = camera.target().clone();
        let up_clone = camera.up().clone();

        pos_clone.z -= delta.1; // delta.1 is positive when scrolling "up" (zooming in)
        pos_clone.z = pos_clone.z.clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);

        camera.set_view(pos_clone, target_clone, up_clone);
    }

    fn rotate_camera(&self, camera: &mut Camera) {
        let target = camera.target().clone();
        let distance = Vec3::distance(target, camera.position().clone());        
    
        dbg!(distance);
        let direction = if self.qe_down.0 {
            1.0
        } else {
            -1.0
        };
        
        let angle_rad : f32 = 0.017; // approximately 1 degree (0.0174533)
        let new_x = distance  * angle_rad.sin() + camera.position().x;
        let new_y = distance  * angle_rad.cos() + camera.position().y;
        
        let new_pos = Vec3::new(new_x, new_y, camera.position().z);
    
        dbg!(camera.position());
        dbg!(new_pos);
        
        let up = camera.up().clone();
    
        camera.set_view(new_pos, target, up);
    }
    fn move_camera(&self, camera: &mut Camera) {
        // I think it is good practice if these are set in order
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        if self.wasd_down.0 {
            direction.y += 1.0;
        }
        if self.wasd_down.1 {
            direction.x -= 1.0;
        }
        if self.wasd_down.2 {
            direction.y -= 1.0;
        }
        if self.wasd_down.3 {
            direction.x += 1.0;
        }
        let speed = if self.shift_down {
            CAMERA_MOVE_SPEED * CAMERA_SHIFT_FACTOR
        } else {
            CAMERA_MOVE_SPEED
        };

        let mut pos_clone = camera.position().clone();
        pos_clone += direction * speed;

        let mut target = pos_clone;
        target.z = 0.0;
        target += CAM_START_TARGET;

        let up_clone = camera.up().clone();

        camera.set_view(pos_clone, target, up_clone);
    }
}
