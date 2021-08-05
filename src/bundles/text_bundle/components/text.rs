use amethyst::{
    ui::{
        FontHandle,
    },
    assets::{
        Format,
        Handle,
    },
    ecs::{
        Component,
        DenseVecStorage,
    },
    renderer::{
        types::{
            DefaultBackend,
        },
    },
};

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub font_size: f32,
    pub color: [f32; 4],
    pub font: Option<FontHandle>,
}

impl Text {
    pub fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 12.0,
            color: [1.0, 1.0, 1.0, 1.0],
            font: None::<FontHandle>,
        }
    }

    pub fn new(text: &str) -> Self {
        Self::default()
            .with_text(text)
    }

//    pub fn create_entity(mut self, world: &mut world) -> EntityBuilder {
//        let factory = world.write_resource::<Factory<DefaultBackend>>(); // TODO generics
//        let queue = world.read_resource::<QueueId>();
//        let glyph_brush = world.write_resource::<GlyphBrush>();
//
//        world
//            .create_entity()
//            .with(self)
//    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = String::from(text);
        self
    }

    pub fn with_font(mut self, font: FontHandle) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }
}

impl Component for Text {
    type Storage = DenseVecStorage<Self>;
}
