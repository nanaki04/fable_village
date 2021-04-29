use amethyst::{
    ecs::{
        Component,
        NullStorage,
    },
};

#[derive(Default)]
pub struct MouseSimulatedTouch;

impl Component for MouseSimulatedTouch {
    type Storage = NullStorage<Self>;
}
