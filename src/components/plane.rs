use amethyst::{
    core::{
        transform::{
            Transform,
        },
    },
    ecs::{
        Component,
        DenseVecStorage,
    },
};

pub struct Plane {
    width: f64,
    height: f64,
}

impl Plane {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn point_in_rect(&self, transform: &Transform, point: (f64, f64)) -> bool {
        let x_min = (transform.translation().x as f64) - self.width / 2.0;
        let x_max = x_min + self.width;
        let y_min = (transform.translation().y as f64) - self.height / 2.0;
        let y_max = y_min + self.height;
        let (x, y) = point;

        x_min <= x && x_max >= x && y_min <= y && y_max >= y
    }
}

impl Component for Plane {
    type Storage = DenseVecStorage<Self>;
}
