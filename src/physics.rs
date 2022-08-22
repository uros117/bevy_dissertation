use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum Colider {
    BoxColider(f32, f32),
    CircleColider(f32),
}


#[derive(Component, Debug)]
pub struct PhysicsObject {
    pub speed: Vec2,
    pub acc: Vec2,
    pub max_acc: Vec2,
    pub colider: Colider,
}

impl Default for PhysicsObject {
    fn default() -> Self {
        Self { 
            speed: Default::default(), 
            acc: Default::default(), 
            max_acc: Vec2::new(1.0, 1.0), 
            colider: Colider::BoxColider(1.0, 1.0) 
        }
    }
}




// physics_update
// fn physiscs_update(
//     mut physics_obj_query: Query<(Entity, &mut Transform, &mut PhysicsObject)>,
// ) {
//     let mut iterator = physics_obj_query.iter_combinations_mut::<2>();
//     while let Some(mut arr) = iterator.fetch_next()  {
//         let (l, m) = arr.split_at_mut(1);
//         if let Some(a) = l.get_mut(0) {
//             //(_ent_a, mut tr_a, mut po_a)
//             if let Some(b) = m.get_mut(0) {
//                 //(_ent_b, mut tr_b, mut po_b)
//                 //print!("{:?} ", a.2);
//                 let tr_a: &mut Transform = &mut a.1;
//                 let po_a: &mut PhysicsObject = &mut a.2;
//                 let tr_b: &mut Transform = &mut b.1;
//                 let po_b: &mut PhysicsObject = &mut b.2;
//                 resolve_colission(tr_a, po_a, tr_b, po_b);
//                 //println!("{:?} ", a.2);
//             }
//         }
//     }
// }