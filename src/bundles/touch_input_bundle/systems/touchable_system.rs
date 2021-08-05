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
    ui::{
        UiTransform,
    },
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
        Touch as WinitTouch,
    },
    window::{
        ScreenDimensions,
    },
    renderer::{
        Camera,
    },
};
use crate::{
    components::{
        plane::{
            Plane,
        },
    },
    bundles::{
        touch_input_bundle::{
            Touch,
            Touchable,
        },
    },
};

#[derive(Debug)]
pub struct TouchableSystem;

impl<'s> System<'s> for TouchableSystem {
    type SystemData = (
        ReadStorage<'s, Touch>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, UiTransform>,
        ReadStorage<'s, Plane>,
        WriteStorage<'s, Touchable>,
    );

    fn run (&mut self, (
        touches,
        transforms,
        ui_transforms,
        planes,
        mut touchables,
    ): Self::SystemData) {
        let touches : Vec<&Touch> = (&touches)
            .join()
            .collect()
            ;

        for (transform, plane, mut touchable) in (&transforms, &planes, &mut touchables).join() {
            match touchable.touch_state {
                Some(TouchPhase::Ended) | Some(TouchPhase::Cancelled) | None => {
                    let touch = touches
                        .iter()
                        .filter(|touch| matches!(touch.status, TouchPhase::Started))
                        .find(|touch| plane.point_in_rect(transform, touch.pos))
                        .map(|&touch| touch)
                        ;

                    if let Some(touch) = touch {
                        touchable.update_touched_state(touch);
                    } else {
                        touchable.clear_touched_state();
                    }
                },
                Some(TouchPhase::Started) | Some(TouchPhase::Moved) => {
                    let touch = touches
                        .iter()
                        .filter(|touch| touchable.touch_id
                            .map(|touch_id| touch_id == touch.id)
                            .unwrap_or(false)
                        )
                        .find(|touch| plane.point_in_rect(transform, touch.pos))
                        .map(|&touch| touch)
                        ;

                    if let Some(touch) = touch {
                        touchable.update_touched_state(touch);
                    } else {
                        touchable.touch_state = Some(TouchPhase::Cancelled);
                    }
                },
            }
        }

        for (ui_transform, mut touchable) in (&ui_transforms, &mut touchables).join() {
            match touchable.touch_state {
                Some(TouchPhase::Ended) | Some(TouchPhase::Cancelled) | None => {
                    let touch = touches
                        .iter()
                        .filter(|touch| matches!(touch.status, TouchPhase::Started))
                        .find(|touch| {
                            let (x, y) = touch.pos;
                            ui_transform.position_inside(x as f32, y as f32)
                        })
                        .map(|&touch| touch)
                        ;

                    if let Some(touch) = touch {
                        touchable.update_touched_state(touch);
                    } else {
                        touchable.clear_touched_state();
                    }
                },
                Some(TouchPhase::Started) | Some(TouchPhase::Moved) => {
                    let touch = touches
                        .iter()
                        .filter(|touch| touchable.touch_id
                            .map(|touch_id| touch_id == touch.id)
                            .unwrap_or(false)
                        )
                        .find(|touch| {
                            let (x, y) = touch.pos;
                            ui_transform.position_inside(x as f32, y as f32)
                        })
                        .map(|&touch| touch)
                        ;

                    if let Some(touch) = touch {
                        touchable.update_touched_state(touch);
                    } else {
                        touchable.touch_state = Some(TouchPhase::Cancelled);
                    }
                },
            }
        }
    }
}
