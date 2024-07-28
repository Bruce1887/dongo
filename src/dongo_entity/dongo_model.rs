use three_d::*;
use crate::*;

pub struct DongoModel {
    pub(crate) id: u16,
    pub(crate) model: Model<PhysicalMaterial>,
    pub(crate) o_type: DongoObjectType,
}
impl DongoModel {
    pub fn foo(&mut self) {
        self.model.iter_mut().for_each(|m| {
            // dbg!(m.geometry.transformation());
            dbg!(m.transformation());
            let transform = m.transformation().clone();
            m.set_transformation(transform*Mat4::from_translation(vec3(0.0, 0.0, 10.0)));
        }
        );
    }
}
impl DongoObjectTraits for DongoModel{
    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_type(&self) -> &DongoObjectType {
        &self.o_type
    }

    fn get_aabb_center(&self) -> Vec3 {
        dbg!(self as &dyn DongoObjectTraits);
        let min_vec = self.model.iter().fold(Vector3::new(f32::MAX, f32::MAX, f32::MAX), |acc, part| {
            let min = part.aabb().min();
            Vector3::new(acc.x.min(min.x), acc.y.min(min.y), acc.z.min(min.z))
        });
        dbg!(min_vec);

        let max_vec = self.model.iter().fold(Vector3::new(f32::MIN, f32::MIN, f32::MIN), |acc, part| {
            let max = part.aabb().max();
            Vector3::new(acc.x.max(max.x), acc.y.max(max.y), acc.z.max(max.z))
        });
        dbg!(max_vec);

        let num_mps = self.model.len();
        let mut center_vec = Vector3::zero();
        for idx in 0..num_mps {
            center_vec += self.model[idx].aabb().center();
        }
        dbg!(center_vec);
        
        center_vec / num_mps as f32
    }
}