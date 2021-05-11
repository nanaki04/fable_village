use amethyst::{
    core::{
        ecs::{
            prelude::{
                World,
                WorldExt,
            },
        },
    },
    assets::{
        AssetStorage,
        Loader,
        Handle,
        ProgressCounter,
        Progress,
    },
    renderer::{
        ImageFormat,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
    },
};
use crate::{
    loaders::{
        sprite_cache::{
            SpriteCache,
        },
    },
};

pub struct SpriteLoader<'a> {
    id: String,
    image_path: String,
    config_path: String,
    image_extension: String,
    config_extension: String,
    progress_counter: Option<&'a mut ProgressCounter>,
}

impl Default for SpriteLoader<'_> {
    fn default() -> Self {
        SpriteLoader {
            id: String::new(),
            image_path: String::new(),
            config_path: String::new(),
            image_extension: String::from("png"),
            config_extension: String::from("ron"),
            progress_counter: None,
        }
    }
}

impl<'a> SpriteLoader<'a> {
    pub fn new(path: &str) -> Self {
        SpriteLoader::default()
            .with_id(path)
            .with_path(path)
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = String::from(id);
        self
    }

    fn with_path(self, path: &str) -> Self {
        self.with_config_path(path)
            .with_image_path(path)
    }

    fn with_config_path(mut self, path: &str) -> Self {
        self.config_path = String::from(path);
        self
    }

    fn with_image_path(mut self, path: &str) -> Self {
        self.image_path = String::from(path);
        self
    }

    pub fn with_progress(mut self, progress_counter: &'a mut ProgressCounter) -> Self {
        self.progress_counter = Some(progress_counter);
        self
    }

    pub fn fetch(&mut self, world: &mut World) -> Handle<SpriteSheet> {
        let handle = {
            let cache = world.entry()
                .or_insert_with(|| SpriteCache::default());

            cache.fetch(&self.id)
        };

        handle
            .unwrap_or_else(|| {
                {
                    self.load(world);
                }
                self.fetch(world)
            })
    }

    pub fn as_renderer(&mut self, world: &mut World, sprite_sheet_index: usize) -> SpriteRender {
        let handle = self.fetch(world);
        SpriteRender::new(handle, sprite_sheet_index)
    }

    pub fn load(&mut self, world: &mut World) {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            let texture_path = format!(
                "{path}.{extension}",
                path = self.image_path,
                extension = self.image_extension
            );

            loader.load(
                texture_path.as_str(),
                ImageFormat::default(),
                (),
                &texture_storage
            )
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            let spritesheet_path = format!(
                "{path}.{extension}",
                path = self.config_path,
                extension = self.config_extension,
            );

            match self.progress_counter.take() {
                Some(progress_counter) => loader.load(
                    spritesheet_path.as_str(),
                    SpriteSheetFormat(texture_handle),
                    progress_counter,
                    &sprite_sheet_storage,
                ),
                None => loader.load(
                    spritesheet_path.as_str(),
                    SpriteSheetFormat(texture_handle),
                    (),
                    &sprite_sheet_storage,
                ),
            }
        };

        let mut cache = world.entry()
            .or_insert_with(|| SpriteCache::default());

        cache.insert(&self.id, sprite_sheet_handle);
    }
}
