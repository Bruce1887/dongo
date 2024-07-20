use crate::common::*;
use three_d::*;

#[derive(Default)]
pub struct EventHandler {
    wasd_down: (bool, bool, bool, bool),
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
                    self.modifiers_updated(modifiers);
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
        if self.check_wasd() {
            self.move_camera(camera);
        }
    }

    fn modifiers_updated(&mut self, modifiers: &Modifiers) {
        self.shift_down = modifiers.shift;
        self.ctrl_down = modifiers.ctrl;
        self.alt_down = modifiers.alt;
        // self.cmd_down = modifiers.command; // command is the ctrl key on windows and linux
    }

    fn check_keys_down(&mut self, ev: &Event) {
        let value: bool;
        let key: Key;
        match ev {
            Event::KeyPress {
                kind,
                modifiers: _,
                handled: _,
            } => {
                key = *kind;
                value = true;
            }
            Event::KeyRelease {
                kind,
                modifiers: _,
                handled: _,
            } => {
                key = *kind;
                value = false;
            }
            _ => panic!("Event is not a key event (KeyPress or KeyRelease)"),
        };

        // camera movement
        match key {
            Key::W => self.wasd_down.0 = value,
            Key::A => self.wasd_down.1 = value,
            Key::S => self.wasd_down.2 = value,
            Key::D => self.wasd_down.3 = value,
            _ => (),
        }
    }

    fn check_wasd(&self) -> bool {
        self.wasd_down.0 || self.wasd_down.1 || self.wasd_down.2 || self.wasd_down.3
    }

    fn zoom_camera(camera: &mut Camera, delta: &(f32, f32)) {
        let mut pos_clone = camera.position().clone();
        let target_clone = camera.target().clone();
        let up_clone = camera.up().clone();

        pos_clone.z -= delta.1; // delta.1 is positive when scrolling "up" (zooming in)
        pos_clone.z = pos_clone.z.clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);

        camera.set_view(pos_clone, target_clone, up_clone);
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

        let up_clone = camera.up().clone();

        camera.set_view(pos_clone, target, up_clone);
    }
}
