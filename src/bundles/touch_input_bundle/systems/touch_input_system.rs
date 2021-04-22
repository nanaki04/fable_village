use amethyst::{
    shrev::{
        EventChannel,
        ReaderId
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
        ReadStorage<'s, MouseSimulatedTouch>,
        WriteStorage<'s, Touch>,
        Entities<'s>,
    );

    fn run (&mut self, (
        input,
        mouse_simulated_touches,
        mut touches,
        entities
    ): Self::SystemData) {
        let (new_touch_info, touch_info) : (Vec<WinitTouch>, Vec<WinitTouch>) = input
            .read(&mut self.reader)
            .filter_map(find_touch)
            .partition(|touch| touch.phase == TouchPhase::Started);

        for (e, mut touch, _) in (&*entities, &mut touches, !&mouse_simulated_touches).join() {
            if touch.is_ended() {
                entities.delete(e).expect("Failed to delete touch");
                continue;
            }

            let winit_touch = touch_info
                .iter()
                .find(|&&t| touch.id == t.id);

            if let Some(t) = winit_touch {
                touch.prev = touch.pos;
                touch.pos = (t.location.x, t.location.y);
                touch.status = t.phase;
            }
        }

        for new_touch in new_touch_info {
            let touch = Touch {
                start: (new_touch.location.x, new_touch.location.y),
                pos: (new_touch.location.x, new_touch.location.y),
                prev: (new_touch.location.x, new_touch.location.y),
                status: new_touch.phase,
                id: new_touch.id,
            };

            entities.build_entity()
                .with(touch, &mut touches)
                .build();
        }
    }
}

fn find_touch(event: &Event) -> Option<WinitTouch> {
    match event {
        Event::WindowEvent { event: WindowEvent::Touch(touch), .. } => Some(*touch),
        _ => None,
    }
}
