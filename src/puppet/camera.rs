use three_d::{Camera, Event, OrbitControl, Vec3};

/// three_d's base OrbitControl is way too sensitive, so here we slow it down
pub struct ScaledOrbitControl {
    orbit_control: OrbitControl,
    scale: f32,
}

impl ScaledOrbitControl {
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32, scale: f32) -> Self {
        Self {
            orbit_control: OrbitControl::new(target, min_distance, max_distance),
            scale,
        }
    }

    pub fn handle_events(&mut self, camera: &mut Camera, events: &[Event]) -> bool {
        let mut scaled_events = Vec::new();

        for event in events.iter().cloned() {
            if let Event::MouseMotion {
                button,
                delta,
                position,
                modifiers,
                handled,
            } = event
            {
                scaled_events.push(Event::MouseMotion {
                    delta: (delta.0 * (self.scale as f64), delta.1 * (self.scale as f64)),
                    button,
                    position,
                    modifiers,
                    handled,
                });
            } else {
                scaled_events.push(event);
            }
        }

        self.orbit_control.handle_events(camera, &mut scaled_events)
    }
}
