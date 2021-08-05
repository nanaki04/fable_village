use amethyst::{
    shrev::{
        EventChannel,
    },
    ecs::{
        *,
    },
    input::{
        InputHandler,
        StringBindings,
    },
};

use crate::{
    events::{
        game_state_event::{
            Transition,
        },
    },
    bundles::{
        touch_input_bundle::{
            Touchable,
        },
    },
};

pub struct ButtonDispatchSystem;

impl<'s> System<'s> for ButtonDispatchSystem {
    type SystemData = (
        ReadStorage<'s, Touchable>,
        Write<'s, EventChannel<Transition>>,
    );

    fn run (&mut self, (
        touchables,
        mut event_channel,
    ): Self::SystemData) {
        for (touchable) in (&touchables).join() {
            if touchable.is_touch_ended() {
                match touchable.id.as_str() {
                    "loading_text" => {
                        event_channel.single_write(Transition::MainMenu);
                    },
                    _ => {
                    },
                }
            }
        }
    }
}
