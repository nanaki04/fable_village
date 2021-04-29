use amethyst::{
    ecs::{
        *,
    },
};
use crate::{
    bundles::{
        touch_input_bundle::{
            Touch,
            Touchable,
            LogLevel,
        },
    },
};

#[derive(Debug)]
pub struct TouchInputDebugSystem {
    log_level: LogLevel,
}

impl TouchInputDebugSystem {
    pub fn new(log_level: LogLevel) -> Self {
        Self { log_level }
    }
}

impl<'s> System<'s> for TouchInputDebugSystem {
    type SystemData = (
        ReadStorage<'s, Touch>,
        ReadStorage<'s, Touchable>,
    );

    fn run (&mut self, (touches, touchables): Self::SystemData) {
        if matches!(self.log_level, LogLevel::Touches | LogLevel::Full) {
            for (touch) in (&touches).join() {
                println!("touch: {:?}", touch);
            }
        }

        if matches!(self.log_level, LogLevel::Touchables | LogLevel::Full) {
            for (touchable) in (&touchables)
                .join()
                .filter(|touchable| touchable.touch_id.is_some()) {

                println!("touchable: {:?}", touchable);
            }
        }
    }
}
