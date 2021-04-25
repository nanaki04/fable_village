use amethyst::{
    assets::{
        ProgressCounter,
    },
    ecs::{
        prelude::{
            *,
            Dispatcher,
            DispatcherBuilder,
        },
    },
    core::{
        ArcThreadPool,
    },
    GameData,
    State,
    StateData,
    StateEvent,
    Trans,
};
//use crate::{
//    systems::{
//        TouchInputSystem,
//    },
//};


use serde::Deserialize;
use amethyst_master_data::derive::MasterData;
use amethyst_master_data::*;

#[derive(Debug, Deserialize, Eq, PartialEq, MasterData, Clone)]
struct Text {
    id: u64,
    desc: String,
    en: String,
    nl: String,
    jp: String,
}


#[derive(Default)]
pub struct LoadingState {
    progress_counter: Option<ProgressCounter>,
    //dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(text) = Text::find(data.world, |text| text.id == 2) {
            println!("{:?}", text);
        }
//        let world = data.world;
//
//        let mut dispatcher_builder = DispatcherBuilder::new();
//        dispatcher_builder.add(TouchInputSystem::new(world), "touch_input_system", &[]);
//
//        let mut dispatcher = dispatcher_builder
//            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
//            .build()
//            ;
//
//        dispatcher.setup(world);
//
//        self.dispatcher = Some(dispatcher);
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world);

//        if let Some(dispatcher) = self.dispatcher.as_mut() {
//            dispatcher.dispatch(data.world);
//        }

        self.progress_counter
            .as_ref()
            .filter(|progress_counter| progress_counter.is_complete())
            .map(|_| Trans::None)
            .unwrap_or(Trans::None)
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        Trans::None
    }
}
