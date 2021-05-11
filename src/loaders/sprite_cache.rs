use amethyst::{
    assets::{
        Handle,
    },
    renderer::{
        SpriteSheet,
    },
};
use std::{
    collections::{
        HashMap,
    },
};

#[derive(Default)]
pub struct SpriteCache {
    handles: HashMap<String, Handle<SpriteSheet>>,
}

impl SpriteCache {
    pub fn insert(&mut self, id: &str, handle: Handle<SpriteSheet>) {
        self.handles.insert(String::from(id), handle);
    }

    pub fn fetch(&self, id: &str) -> Option<Handle<SpriteSheet>> {
        self.handles.get(id)
            .map(|sprite_sheet_handle| sprite_sheet_handle.clone())
    }
}
