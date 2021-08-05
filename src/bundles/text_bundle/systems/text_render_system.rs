use amethyst::{
    core::{
        transform::{
            Transform,
        },
        math::{
            Point3,
            Vector2,
        },
    },
    ecs::{
        *,
    },
    ui::{
        FontAsset,
    },
    assets::{
        AssetStorage,
    },
};
use crate::{
    bundles::{
        text_bundle::{
            Text,
        },
    },
};

#[derive(Debug)]
pub struct TextRenderSystem;

impl TextRenderSystem {
    pub fn new() -> Self {
        Self
    }
}

impl<'s> System<'s> for TextRenderSystem {
    type SystemData = (
        ReadStorage<'s, Text>,
        ReadStorage<'s, Transform>,
        //ReadStorage<'s, SpriteRender>,
        //Write<'s, AssetStorage<Texture>>,
        //Read<'s, AssetStorage<SpriteSheet>>,
        Read<'s, AssetStorage<FontAsset>>,
    );

    fn run (&mut self, (
        texts,
        transforms,
        fonts,
    ): Self::SystemData) {
    }
}
