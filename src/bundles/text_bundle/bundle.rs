use crate::{
    bundles::{
        text_bundle::{
            TextRenderSystem,
        },
    },
};
use amethyst::{
    core::{
        ecs::{
            prelude::{
                DispatcherBuilder,
                World,
            },
        },
        SystemBundle,
    },
    Error,
};

pub struct TextBundle;

impl TextBundle {
    pub fn new() -> Self {
        Self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for TextBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {

        builder.add(
            TextRenderSystem::new(),
            "text_render_system",
            &[],
        );

        Ok(())
    }
}
