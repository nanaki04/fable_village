use amethyst::{
    winit::{
        TouchPhase,
    },
    ecs::{
        Component,
        DenseVecStorage,
    },
};
use crate::{
    bundles::{
        touch_input_bundle::{
            components::{
                Touch,
            },
        },
    },
};

#[derive(Debug)]
pub struct Touchable {
    pub id: String,
    pub touch_state: Option<TouchPhase>,
    pub touch_id: Option<u64>,
}

impl Touchable {
    pub fn new(id: String) -> Self {
        Self {
            id,
            touch_state: None,
            touch_id: None,
        }
    }

    pub fn update_touched_state(&mut self, touch: &Touch) {
        self.touch_state = Some(touch.status);
        self.touch_id = Some(touch.id);
    }

    pub fn clear_touched_state(&mut self) {
        self.touch_state = None;
        self.touch_id = None;
    }
}

impl Component for Touchable {
    type Storage = DenseVecStorage<Self>;
}
