use crate::{
    bundles::{
        touch_input_bundle::{
            TouchInputSystem,
            MouseAsTouchSystem,
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
    winit::{
        MouseButton,
    },
    Error,
};

pub struct TouchInputBundle {
    mouse_buttons: Vec<MouseButton>,
}

impl TouchInputBundle {
    pub fn new() -> Self {
        TouchInputBundle { mouse_buttons: vec![] }
    }

    pub fn with_mouse_simulation(mut self, button: MouseButton) -> Self {
        self.mouse_buttons.push(button);
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for TouchInputBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {

        if self.mouse_buttons.len() > 0 {
            builder.add(
                MouseAsTouchSystem::new(world, self.mouse_buttons.clone()),
                "mouse_as_touch_system",
                &[],
            );

            builder.add(
                TouchInputSystem::new(world),
                "touch_input_system",
                &["mouse_as_touch_system"],
            );
        } else {
            builder.add(
                TouchInputSystem::new(world),
                "touch_input_system",
                &[],
            );
        }

        Ok(())
    }
}
