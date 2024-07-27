use crate::{common::*, dongo_object::*, mouse_selection::resize_selection};
use three_d::*;

#[derive(Default)]
pub struct EventHandler {
    wasd_down: (bool, bool, bool, bool), // camera movement
    qe_down: (bool, bool),               // camera rotation
    shift_down: bool,
    ctrl_down: bool,
    alt_down: bool,
    dragging_state: DraggingState,
    // cmd_down: bool, // command is the ctrl key on windows and linux
}

enum DraggingState {
    Dragging(Vec3),
    NotDragging,
}
impl Default for DraggingState {
    fn default() -> Self {
        DraggingState::NotDragging
    }
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            ..std::default::Default::default()
        }
    }

    pub fn handle_events(
        &mut self,
        events: &Vec<Event>,
        camera: &mut Camera,
        context: &Context,
        objects: &mut DongoObjectManager,
    ) {
        for ev in events {
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

                    if *kind == Key::ArrowUp {
                        crate::camera_controller::zoom_camera(camera, &(28.0, 28.0))
                    }

                    if *kind == Key::ArrowDown {
                        crate::camera_controller::zoom_camera(camera, &(-28.0, -28.0))
                    }

                    if *kind == Key::Num0 {
                        // reset camera position and stuff. for debug
                        camera.set_view(CAM_START_POS, CAM_START_TARGET, CAM_START_UP);
                    }

                    if *kind == Key::X {
                        for m in objects.get_models() {
                            let m = m as &dyn DongoObjectTraits;
                            dbg!(m, m.get_aabb_center());
                            
                        }
                        for m in objects.get_objects() {
                            let m = m as &dyn DongoObjectTraits;
                            dbg!(m,m.get_aabb_center());
                        }
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
                } => crate::camera_controller::zoom_camera(camera, delta),
                Event::MousePress {
                    button,
                    position,
                    modifiers: _,
                    handled: _,
                } => {
                    if *button == MouseButton::Left {
                        if let Some(start_pick) = pick(
                            context,
                            &camera,
                            *position,
                            objects
                                .get_objects_vec(|o: &DongoObject| o.get_type() == &DongoObjectType::Map),
                        ) {
                            //pick_mesh.set_transformation(Mat4::from_translation(pick));
                            self.dragging_state = DraggingState::Dragging(start_pick);
                        }
                    }
                }
                Event::MouseRelease {
                    button,
                    position:_,
                    modifiers: _,
                    handled: _,
                } => {
                    if *button == MouseButton::Left {
                        if let DraggingState::Dragging(_) = self.dragging_state {
                            drop(objects.take_obj(SELECTION_ID));
                            //resize_selection(objects, start, *position, context)
                            self.dragging_state = DraggingState::NotDragging;
                        }
                    }
                }
                Event::MouseMotion {
                    button: _,
                    delta: _,
                    position,
                    modifiers: _,
                    handled: _,
                } => {
                    if let DraggingState::Dragging(start) = self.dragging_state {
                        if let Some(end_pick) = pick(
                            context,
                            &camera,
                            *position,
                            objects
                                .get_objects_vec(|o: &DongoObject| o.get_type() == &DongoObjectType::Map),
                        ) {
                            resize_selection(objects, start, end_pick, context)
                        }
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
            let mut rotation_direction = 0.0;
            if self.qe_down.0 {
                rotation_direction += 1.0;
            }
            if self.qe_down.1 {
                rotation_direction -= 1.0;
            }
            if rotation_direction != 0.0 {
                crate::camera_controller::rotate_camera(camera, rotation_direction);
            }
        }
    }

    fn check_keys_down(&mut self, ev: &Event) {
        let value: bool;
        let key: Key;
        if let Event::KeyPress {
            kind,
            modifiers: _,
            handled: _,
        } = ev
        {
            key = *kind;
            value = true;
        } else if let Event::KeyRelease {
            kind,
            modifiers: _,
            handled: _,
        } = ev
        {
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
