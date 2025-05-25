use std::sync::{Arc, Mutex};

use gilrs::{Axis, Button, EventType, GamepadId, Gilrs};

use crate::vtl::{GlottisIdx, TractIdx, VTLParams};

pub struct ParamController {
    gilrs: Gilrs,
    params: Arc<Mutex<VTLParams>>,
    gamepad_id: GamepadId,
}

impl ParamController {
    pub fn new(params: Arc<Mutex<VTLParams>>) -> Self {
        let gilrs = Gilrs::new().unwrap();
        let gamepad_id = gilrs
            .gamepads()
            .find(|(_, gp)| gp.is_connected())
            .expect("No connected controller!")
            .0;
        ParamController {
            gilrs,
            params,
            gamepad_id,
        }
    }

    pub fn controller_loop(&mut self) {
        use EventType as E;
        use GlottisIdx as Gi;
        use TractIdx as Ti;
        let mut params = self.params.lock().unwrap();
        params.set_glottis_value(Gi::PR, 0.7);
        drop(params);
        loop {
            let event = match self.gilrs.next_event_blocking(None) {
                Some(e) => e,
                None => continue,
            };
            if event.id != self.gamepad_id {
                continue;
            }
            let mut params = self.params.lock().unwrap();
            match event.event {
                E::ButtonChanged(button, val, _) => match button {
                    Button::LeftTrigger => {
                        params.set_glottis_value(Gi::AS, val as f64);
                        params.set_glottis_value(Gi::XB, 0.2 + val as f64);
                        params.set_glottis_value(Gi::XT, 0.2 + val as f64);
                    }
                    Button::LeftTrigger2 => {
                        params.set_tract_value(Ti::VO, val as f64);
                    }
                    Button::RightTrigger => (),
                    Button::RightTrigger2 => {
                        params.set_tract_value(Ti::LD, 1.0 - val as f64);
                    }
                    _ => (),
                },
                E::AxisChanged(axis, val, _) => match axis {
                    Axis::LeftStickX => {
                        params.set_tract_value(
                            Ti::TCX,
                            stretch(normalise_axis(val), 0.2, 0.8) as f64,
                        );
                    }
                    Axis::LeftStickY => {
                        params.set_tract_value(
                            Ti::TCY,
                            stretch(normalise_axis(val), 0.0, 0.6) as f64,
                        );
                    }
                    Axis::RightStickX => {
                        params.set_tract_value(Ti::TBX, normalise_axis(val) as f64);
                        params.set_tract_value(Ti::TTX, normalise_axis(val) as f64);
                    }
                    Axis::RightStickY => {
                        params.set_tract_value(Ti::TBY, normalise_axis(val) as f64);
                        params.set_tract_value(Ti::TTY, normalise_axis(val) as f64);
                    }
                    _ => (),
                },
                E::Disconnected => break,
                _ => (),
            }
            dbg!(params.tract_state());
            dbg!(params.glottis_state());
        }
    }
}

fn stretch(val: f32, min: f32, max: f32) -> f32 {
    min + (max - min) * val
}

fn normalise_axis(val: f32) -> f32 {
    val / 2f32.sqrt() + 0.5
}
