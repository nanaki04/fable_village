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
    },
    renderer::{
        ImageFormat,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
    },
};

fn load_texture(world: &World, file: &str) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        file,
        ImageFormat::default(),
        (),
        &texture_storage,
    )
}

fn load_spritesheet(
    world: &World,
    file: &str,
    extension: &str,
) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    let texture_handle = load_texture(
        world,
        format!("{file}.{extension}", file = file, extension = extension).as_str(),
    );
    loader.load(
        format!("{}.ron", file).as_str(),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_storage,
    )
}

fn load_background_texture(
    world: &World,
    file: &str,
    extension: &str,
) -> Handle<SpriteSheet> {
    let path = format!("textures/backgrounds/{}", file);
    load_spritesheet(
        world,
        path.as_str(),
        extension,
    )
}

pub fn load_background_renderer(
    world: &World,
    file: &str,
    extension: &str,
) -> SpriteRender {
    let sprite_sheet_handle = load_background_texture(world, file, extension);
    SpriteRender::new(sprite_sheet_handle, 0)
}
