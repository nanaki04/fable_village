use crate::{
    bundles::{
        touch_input_bundle::{
            TouchInputSystem,
            MouseAsTouchSystem,
            TouchableSystem,
            TouchInputDebugSystem,
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

#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Silent,
    Touchables,
    Touches,
    Full,
}

pub struct TouchInputBundle {
    mouse_buttons: Vec<MouseButton>,
    track_touchables: bool,
    log_level: LogLevel,
}

impl TouchInputBundle {
    pub fn new() -> Self {
        TouchInputBundle {
            mouse_buttons: vec![],
            track_touchables: false,
            log_level: LogLevel::Silent,
        }
    }

    pub fn with_touchables(mut self) -> Self {
        self.track_touchables = true;
        self
    }

    pub fn with_mouse_simulation(mut self, button: MouseButton) -> Self {
        self.mouse_buttons.push(button);
        self
    }

    pub fn with_logging(mut self, log_level: LogLevel) -> Self {
        self.log_level = log_level;
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

        if self.track_touchables {
            builder.add(
                TouchableSystem,
                "touchable_system",
                &["touch_input_system"],
            );
        }

        if self.log_level != LogLevel::Silent {
            builder.add(
                TouchInputDebugSystem::new(self.log_level),
                "touch_input_debug_system",
                &["touch_input_system"],
            );
        }

        Ok(())
    }
}
