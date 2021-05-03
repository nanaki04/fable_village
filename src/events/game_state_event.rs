use amethyst::{
    core::{
        ecs::{Read, SystemData, World},
        shrev::{ReaderId, EventChannel},
        EventReader,
    },
    derive::{
        EventReader,
    },
    winit::Event,
};

#[derive(Clone, Debug)]
pub struct Quit;

#[derive(Clone, Debug)]
pub struct Unit;

#[derive(Debug, EventReader, Clone)]
#[reader(GameStateEventReader)]
pub enum GameStateEvent {
    Quit(Quit),
    Placeholder(Unit),
}
