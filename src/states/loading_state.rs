use amethyst::{
    assets::{
        ProgressCounter,
    },
    ecs::{
        Component,
        DenseVecStorage,
        prelude::{
            *,
            Dispatcher,
            DispatcherBuilder,
        },
    },
    core::{
        ArcThreadPool,
        transform::{
            Transform,
        },
    },
    renderer::{
        Camera,
    },
    window::{
        ScreenDimensions,
    },
    shrev::{
        EventChannel,
        ReaderId,
    },
    GameData,
    State,
    StateData,
    StateEvent,
    TransEvent,
    Trans,
};
use crate::{
    components::{
        plane::{
            Plane,
        },
    },
    bundles::{
        touch_input_bundle::{
            Touchable,
        },
    },
    loaders::{
        texture_loader::{
            load_background_renderer,
        },
    },
    events::{
        GameStateEvent,
    },
};

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
    //reader: Option<ReaderId<GameStateEvent>>,
    //dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> State<GameData<'a, 'b>, GameStateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(text) = Text::find(data.world, |text| text.id == 3) {
            println!("{:?}", text);
        }

        let world = data.world;
        let (screen_width, screen_height) = {
            let screen_dimensions = world.read_resource::<ScreenDimensions>();
            (screen_dimensions.width(), screen_dimensions.height())
        };

        let mut cam_transform = Transform::default();
        cam_transform.set_translation_xyz(
            screen_width / 2.0,
            screen_height / 2.0,
            1.0
        );
        world
            .create_entity()
            .with(Camera::standard_2d(screen_width, screen_height))
            .with(cam_transform)
            .build()
            ;

        let mut plane_transform = Transform::default();
        plane_transform.set_translation_xyz(25.0, 25.0, 0.0);
        world
            .create_entity()
            .with(plane_transform)
            .with(Plane::new(25.0, 25.))
            .with(Touchable::new("test_plane".to_string()))
            .build()
            ;

        let mut background_transform = Transform::default();
        background_transform.set_translation_xyz(
            screen_width / 2.0,
            screen_height / 2.0,
            0.0
        );
        let background_renderer = load_background_renderer(&world, "bg001", "png");
        world
            .create_entity()
            .with(background_transform)
            .with(background_renderer)
            .build()
            ;

        //self.reader = Some(world.fetch_mut::<EventChannel<GameStateEvent>>()
            //.register_reader());

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
    ) -> Trans<GameData<'a, 'b>, GameStateEvent> {
        data.data.update(data.world);

//        if let Some(dispatcher) = self.dispatcher.as_mut() {
//            dispatcher.dispatch(data.world);
//        }

        self.progress_counter
            .as_ref()
            .filter(|progress_counter| progress_counter.is_complete())
            .map(|_| Trans::None)
            .unwrap_or(Trans::None)

//        match trans {
//            Trans::None => {
//                let channel = data.world.read_resource::<EventChannel<GameStateEvent>>();
//                channel.read(self.reader.as_mut().unwrap())
//                    .inspect(|e| println!("e: {:?}", e))
//                    .map(|e| match e {
//                        GameStateEvent::Quit(_) => Trans::Quit,
//                        _ => Trans::None,
//                    })
//                    .find(|trans| !matches!(trans, Trans::None))
//                    .unwrap_or(Trans::None)
//            },
//            trans => trans,
//        }
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: GameStateEvent,
    ) -> Trans<GameData<'a, 'b>, GameStateEvent> {
        println!("event procced: {:?}", event);
        match event {
            GameStateEvent::Quit(_) => Trans::Quit,
            _ => Trans::None,
        }
    }
}
