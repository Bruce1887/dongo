use three_d::*;
struct DongoMetaData{
    _id: Option<u16>,
    _desc: Option<String>,
}

enum DongoEntity{
    Object(Box<dyn Object>,DongoMetaData),
    Model(Model<PhysicalMaterial>,DongoMetaData), 
    //ColorModel(Model<ColorMaterial>),
}

impl DongoEntity {
    pub fn from_obj_file(
        context: &Context,
        obj_filename: &str,
    ) -> DongoEntity {
        let path = format!("assets/{}/massaged_{}.obj", obj_filename,obj_filename);
        let mut loaded = three_d_asset::io::load(&[path]).unwrap();
        let model = loaded.deserialize(format!("{}.obj",obj_filename)).unwrap();

        let mut model_mat = three_d::Model::<PhysicalMaterial>::new(&context, &model).unwrap();

        // set cull to back
        model_mat.iter_mut().for_each(|part| {
            part.material.render_states.cull = Cull::Back;  
        });

        let meta = DongoMetaData{
            _id: None,
            _desc: None,
        };

        DongoEntity::Model(model_mat, meta)
    } 

    /*
    fn get_objects(&self) -> Vec<&dyn Object> {
        match self {
            DongoEntity::Object(o, _) => vec![o.as_ref()],
            DongoEntity::Model(m, _) => m.iter().map(|part| part as &dyn Object).collect(),
            DongoEntity::ColorModel(m) => m.iter().map(|part| part as &dyn Object).collect(),
        }
    }
    */
    
}

struct EntitiesHandler {
    e_vec: Vec<DongoEntity>,
}
impl EntitiesHandler {
    fn get_objects(&self) -> Vec<&dyn Object> {
        let mut objects: Vec<&dyn Object> = Vec::new();

        self.e_vec.iter().for_each(|e| {
            match e {
                DongoEntity::Object(o, _) => objects.push(o.as_ref()),
                DongoEntity::Model(m, _) => m.iter().for_each(|part| objects.push(part)),
                // DongoEntity::ColorModel(m) => m.iter().for_each(|part| objects.push(part)),
            }
        });   

        objects
    }
}

fn main(){
        // Create a window (a canvas on web)
        let window = Window::new(WindowSettings {
            title: "test Entity!".to_string(),
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
            vec3(0.0, 0.0, 20.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            1000.0,
        );

        let mut entities = EntitiesHandler{
            e_vec: Vec::new(),
        };

        let mut cube_trimesh = CpuMesh::cube();
        cube_trimesh.colors = Some(Vec::from([Srgba::RED; 36]));
        let cube_gm = Gm::new(
            Mesh::new(&context, &cube_trimesh),
            ColorMaterial::default(),
        );
        entities.e_vec.push(DongoEntity::Object(Box::new(cube_gm), DongoMetaData{_id: None, _desc: None}));

    
        let tree_de = DongoEntity::from_obj_file(&context, "low-poly-pinetree");
        entities.e_vec.push(tree_de);
        

        let mut directional_light =
            renderer::light::DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(2.0, 0.0, -1.0));
        
        window.render_loop(
            move |frame_input| // Begin a new frame with an updated frame input
        {
            // Ensure the viewport matches the current window viewport which changes if the window is resized
            camera.set_viewport(frame_input.viewport);            

            directional_light.generate_shadow_map(1024, &entities.get_objects());

            frame_input.screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(
                    &camera, entities.get_objects(), &[]
                );
            FrameOutput::default()
        },
        );
    }
    