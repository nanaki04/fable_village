use amethyst::{
    shrev::{
        EventChannel,
        ReaderId,
    },
    GameData,
    State,
    StateData,
    Trans,
};
use crate::{
    loaders::{
        sprite_loader::{
            SpriteLoader,
        },
    },
    events::{
        GameStateEvent,
    },
};

#[derive(Default)]
pub struct MainMenuState;

impl<'a, 'b> State<GameData<'a, 'b>, GameStateEvent> for MainMenuState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("swapped to main menu state");
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, GameStateEvent> {
        data.data.update(data.world);

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: GameStateEvent,
    ) -> Trans<GameData<'a, 'b>, GameStateEvent> {
        println!("event procced: {:?}", event);
        match event {
            GameStateEvent::Quit(_) => Trans::Quit,
            _ => Trans::None,
        }
    }
}
