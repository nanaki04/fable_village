use amethyst::{
    core::{
        transform::{
            Transform,
        },
    },
    ecs::{
        *,
    },
    winit::{
        WindowEvent,
        Event,
        TouchPhase,
        Touch as WinitTouch,
    },
    window::{
        ScreenDimensions,
    },
    renderer::{
        Camera,
    },
    shrev::{
        EventChannel,
        ReaderId
    },
};
use crate::{
    bundles::{
        touch_input_bundle::{
            Touch,
            components::{
                MouseSimulatedTouch,
            },
        },
    },
    extensions::{
        camera::{
            CameraExt,
        },
    },
};

#[derive(Debug)]
pub struct TouchInputSystem {
    reader: ReaderId<Event>,
}

impl TouchInputSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader = world.fetch_mut::<EventChannel<Event>>().register_reader();

        Self { 
            reader,
        }
    }

}

impl<'s> System<'s> for TouchInputSystem {
    type SystemData = (
        Read<'s, EventChannel<Event>>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, MouseSimulatedTouch>,
        WriteStorage<'s, Touch>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
    );

    fn run (&mut self, (
        input,
        screen_dimensions,
        mouse_simulated_touches,
        mut touches,
        cameras,
        transforms,
        entities
    ): Self::SystemData) {
        let camera = (&cameras, &transforms).join()
            .next()
            ;

        let (new_touch_info, touch_info) : (Vec<WinitTouch>, Vec<WinitTouch>) = input
            .read(&mut self.reader)
            .filter_map(find_touch)
            .partition(|touch| touch.phase == TouchPhase::Started)
            ;

        for (e, mut touch, _) in (&*entities, &mut touches, !&mouse_simulated_touches).join() {
            let winit_touch = touch_info
                .iter()
                .find(|&&t| touch.id == t.id)
                ;

            if let Some(t) = winit_touch {
                let world_pos = camera
                    .map(|(cam, transform)| cam.to_world_pos(
                        transform,
                        (t.location.x, t.location.y),
                        (screen_dimensions.width() as f64, screen_dimensions.height() as f64),
                    ))
                    .unwrap_or((t.location.x, t.location.y))
                    ;

                touch.prev = touch.pos;
                touch.pos = world_pos;
                touch.status = t.phase;
            }

            if touch.is_ended() { // MEMO: even when deleted here, the entity lasts for the rest of the frame to handle the Ended or Cancelled status
                entities.delete(e).expect("Failed to delete touch");
            }
        }

        for new_touch in new_touch_info {
            let world_pos = camera
                .map(|(cam, transform)| cam.to_world_pos(
                    transform,
                    (new_touch.location.x, new_touch.location.y),
                    (screen_dimensions.width() as f64, screen_dimensions.height() as f64),
                ))
                .unwrap_or((new_touch.location.x, new_touch.location.y))
                ;

            let touch = Touch {
                start: world_pos,
                pos: world_pos,
                prev: world_pos,
                status: new_touch.phase,
                id: new_touch.id,
            };

            entities.build_entity()
                .with(touch, &mut touches)
                .build()
                ;
        }
    }
}

fn find_touch(event: &Event) -> Option<WinitTouch> {
    match event {
        Event::WindowEvent { event: WindowEvent::Touch(touch), .. } => Some(*touch),
        _ => None,
    }
}
