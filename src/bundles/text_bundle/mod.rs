mod systems;
mod components;
mod bundle;
//mod resources;

pub use self::{
    bundle::{
        TextBundle,
    },
    systems::{
        text_render_system::{
            TextRenderSystem,
        },
    },
    components::{
        text::{
            Text,
        },
    },
};
pub use amethyst::{
    ui::{
        TtfFormat,
    },
};
