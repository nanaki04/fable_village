use amethyst::{
    winit::{
        TouchPhase,
    },
    ecs::{
        Component,
        DenseVecStorage,
    },
};

#[derive(Debug)]
pub struct Touch {
    pub start: (f64, f64),
    pub pos: (f64, f64),
    pub prev: (f64, f64),
    pub status: TouchPhase,
    pub id: u64,
}

impl Touch {
    pub fn is_ended(&self) -> bool {
        self.status == TouchPhase::Ended || self.status == TouchPhase::Cancelled
    }
}

impl Component for Touch {
    type Storage = DenseVecStorage<Self>;
}
