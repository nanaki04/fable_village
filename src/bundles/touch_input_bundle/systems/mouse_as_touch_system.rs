use amethyst::{
    shrev::{
        EventChannel,
        ReaderId
    },
    ecs::{
        *,
    },
    winit::{
        WindowEvent,
        Event,
        TouchPhase,
    },
};
use crate::{
    bundles::{
        touch_input_bundle::{
            Touch,
            components::{
                MouseSimulatedTouch,
            },
            resources::{
                MousePosition,
            },
        },
    },
};

pub use amethyst::{
    winit::{
        MouseButton,
        ElementState,
    },
};

#[derive(Debug)]
pub struct MouseAsTouchSystem {
    reader: ReaderId<Event>,
    mouse_buttons: Vec<MouseButton>,
}

impl MouseAsTouchSystem {
    pub fn new(world: &mut World, mouse_buttons: Vec<MouseButton>) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader = world.fetch_mut::<EventChannel<Event>>().register_reader();

        let mouse_position = MousePosition {
            x: 0.0,
            y: 0.0,
        };
        world.insert(mouse_position);

        Self { 
            reader,
            mouse_buttons,
        }
    }

}

impl<'s> System<'s> for MouseAsTouchSystem {
    type SystemData = (
        Read<'s, EventChannel<Event>>,
        Write<'s, MousePosition>,
        WriteStorage<'s, Touch>,
        WriteStorage<'s, MouseSimulatedTouch>,
        Entities<'s>,
    );

    fn run (
        &mut self,
        (
            input,
            mut mouse_pos,
            mut touches,
            mut mouse_simulated_touches,
            entities,
        ): Self::SystemData,
    ) {
        let mouse_events : Vec<&WindowEvent> = input
            .read(&mut self.reader)
            .filter_map(find_mouse_event)
            .collect();

        let last_move_event = mouse_events
            .iter()
            .rev()
            .find(|e| matches!(e, WindowEvent::CursorMoved { .. }));

        if let Some(WindowEvent::CursorMoved { position, .. }) = last_move_event {
            mouse_pos.x = position.x;
            mouse_pos.y = position.y;
        }

        let has_cancel_event = mouse_events
            .iter()
            .find(|e| matches!(e, WindowEvent::CursorLeft { .. }))
            .is_some();

        let pressed_buttons : Vec<u64> = mouse_events
            .iter()
            .filter_map(|&e| find_pressed_button(e))
            .filter(|button| self.mouse_buttons.contains(&button))
            .map(button_to_id)
            .collect();

        let released_buttons : Vec<u64> = mouse_events
            .iter()
            .filter_map(|&e| find_released_button(e))
            .filter(|button| self.mouse_buttons.contains(&button))
            .map(button_to_id)
            .collect();

        for (e, mut touch, _) in (&*entities, &mut touches, &mouse_simulated_touches).join() {
            if touch.is_ended() {
                entities.delete(e).expect("Failed to delete mouse simulated touch");
                continue;
            }

            if has_cancel_event {
                touch.status = TouchPhase::Cancelled;
                continue;
            }

            if released_buttons.contains(&touch.id) {
                touch.status = TouchPhase::Ended;
                continue;
            }

            touch.prev = touch.pos;
            touch.pos = (mouse_pos.x, mouse_pos.y);
            touch.status = TouchPhase::Moved;
        }

        for new_touch_id in pressed_buttons {
            let touch = Touch {
                start: (mouse_pos.x, mouse_pos.y),
                pos: (mouse_pos.x, mouse_pos.y),
                prev: (mouse_pos.x, mouse_pos.y),
                status: TouchPhase::Started,
                id: new_touch_id,
            };

            entities.build_entity()
                .with(touch, &mut touches)
                .with(MouseSimulatedTouch, &mut mouse_simulated_touches)
                .build();
        }
    }
}

fn find_mouse_event(event: &Event) -> Option<&WindowEvent> {
    match event {
        Event::WindowEvent { event: e @ WindowEvent::CursorMoved { .. }, .. } => Some(e),
        Event::WindowEvent { event: e @ WindowEvent::MouseInput { .. }, .. } => Some(e),
        // TODO check behaviour related leaving and entering the window
        //Event::WindowEvent { event: e @ WindowEvent::CursorEntered { .. }, .. } => Some(*e),
        Event::WindowEvent { event: e @ WindowEvent::CursorLeft { .. }, .. } => Some(e),
        _ => None,
    }
}

fn find_pressed_button(event: &WindowEvent) -> Option<MouseButton> {
    match event {
        WindowEvent::MouseInput { state: ElementState::Pressed, button, .. } => Some(*button),
        _ => None,
    }
}

fn find_released_button(event: &WindowEvent) -> Option<MouseButton> {
    match event {
        WindowEvent::MouseInput { state: ElementState::Released, button, .. } => Some(*button),
        _ => None,
    }
}

fn button_to_id(button : MouseButton) -> u64 {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(other) => other as u64,
    }
}

pub fn id_to_mouse_button(id: u64) -> MouseButton {
    match id {
        0 => MouseButton::Left,
        1 => MouseButton::Right,
        2 => MouseButton::Middle,
        other => MouseButton::Other(other as u8),
    }
}
