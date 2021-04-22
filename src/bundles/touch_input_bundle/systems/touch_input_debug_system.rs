use amethyst::{
    ecs::{
        *,
    },
};
use crate::{
    bundles::{
        touch_input_bundle::{
            Touch,
        },
    },
};

#[derive(Debug)]
pub struct TouchInputDebugSystem;

impl<'s> System<'s> for TouchInputDebugSystem {
    type SystemData = ReadStorage<'s, Touch>;

    fn run (&mut self, touches: Self::SystemData) {
        for (touch) in (&touches).join() {
            println!("touch: {:?}", touch);
        }
    }
}
