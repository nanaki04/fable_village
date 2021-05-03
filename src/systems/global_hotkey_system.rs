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
            Quit,
        },
    },
};

pub struct GlobalHotkeySystem;

impl<'s> System<'s> for GlobalHotkeySystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, EventChannel<Quit>>,
    );

    fn run (&mut self, (
        input,
        mut event_channel,
    ): Self::SystemData) {
        if input.action_is_down("quit").unwrap_or(false) {
            event_channel.single_write(Quit);
        }
    }
}
