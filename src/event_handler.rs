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
                        crate::camera_controller::zoom_camera(camera, &(28.0,28.0))
                    }
                    
                    if *kind == Key::ArrowDown{
                        crate::camera_controller::zoom_camera(camera, &(-28.0,-28.0))
                    }

                    if *kind == Key::Num0{ // reset camera position and stuff. for debug
                        camera.set_view(CAM_START_POS, CAM_START_TARGET, CAM_START_UP);
                    }

                }
                Event::KeyRelease {
                    kind,
                    modifiers: _,
                    handled: _,
                } => {
                    self.check_keys_down(ev);

                    if *kind == Key::Q  {
                        crate::camera_controller::rotate_camera(camera);
                    }
                }
                Event::MouseWheel {
                    delta,
                    position: _,
                    modifiers: _,
                    handled: _,
                } => crate::camera_controller::zoom_camera(camera, delta),
                Event::MousePress { button , position, modifiers, handled } => {

                    if *button == MouseButton::Left {
                        println!("MousePress: button: {:?}, position: {:?}, modifiers: {:?}, handled: {:?}", button, position, modifiers, handled);
                        dbg!(position);
                        let world_pos = camera.position_at_pixel(*position);
                        dbg!("World position: {:?}", world_pos);
                        dbg!(camera.view_direction_at_pixel(*position));
                    }

                }
                _ => (),
            }
        }        
        
        // check if any of the wasd keys are down, if so move the camera
        if self.wasd_down.0 || self.wasd_down.1 || self.wasd_down.2 || self.wasd_down.3 {
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

            crate::camera_controller::move_camera(camera, direction, speed);
        }
        if self.qe_down.0 || self.qe_down.1 {
            //self.rotate_camera(camera);
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
}
