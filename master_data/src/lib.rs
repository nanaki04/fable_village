use specs::{
    World,
};
use std::{
    marker::{
        Sized,
    },
};
pub use amethyst_master_data_derive as derive;

pub trait MasterData {
    fn preload(world: &mut World);
    fn all(world: &mut World) -> Vec<Self> where Self: Sized;
    fn find<F>(world: &mut World, predicate: F) -> Option<Self> where
        F: Fn(&Self) -> bool,
        Self: Sized;
    fn filter<F>(world: &mut World, predicate: F) -> Vec<Self> where
        F: Fn(&Self) -> bool,
        Self: Sized;
}
