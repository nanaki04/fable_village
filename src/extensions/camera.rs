use amethyst::{
    core::{
        transform::{
            Transform,
        },
        math::{
            Point3,
            Vector2,
        },
    },
    renderer::{
        Camera,
    },
};

pub trait CameraExt {
    fn dimensions(&self) -> (f32, f32);
    fn to_world_pos(
        &self,
        transform: &Transform,
        pos: (f64, f64),
        screen_res: (f64, f64),
    ) -> (f64, f64);
}

impl CameraExt for Camera {
    fn dimensions(&self) -> (f32, f32) {
        let width = 2.0 / self.matrix[(0, 0)];
        let height = -2.0 / self.matrix[(1, 1)];

        (width, height)
    }

    fn to_world_pos(
        &self,
        transform: &Transform,
        pos: (f64, f64),
        screen_res: (f64, f64),
    ) -> (f64, f64) {
        let (x, y) = pos;
        let (screen_width, screen_height) = screen_res;

        // MEMO: cam translation is not properly taken into account
        // so manual calculation is needed
        let local_point = self.screen_to_world_point(
            Point3::new(x as f32, y as f32, 0.0),
            Vector2::new(screen_width as f32, screen_height as f32),
            transform,
        );

        (local_point.x as f64, local_point.y as f64)
    }
}
