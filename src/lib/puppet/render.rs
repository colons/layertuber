use super::camera::ScaledOrbitControl;
use super::config::{LayerConfig, Rule};
use super::rig::{Rig, RigLayer};
use crate::tracker::{ControlMessage, TrackingReport};
use core::ops::Mul;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use three_d::window::{Window, WindowSettings};
use three_d::{
    degrees, vec3, Blend, Camera, ClearState, ColorMaterial, Context, CpuMesh, Event, FrameInput,
    FrameOutput, Gm, Key, Mat4, Mesh, RenderStates, Texture2D, Vec3,
};

struct RenderLayer {
    model: Gm<Mesh, ColorMaterial>,
    base_transformation: Mat4,
    configs: Vec<LayerConfig>,
}

impl RenderLayer {
    fn from_rig_layer(rig: &Rig, rig_layer: &RigLayer, context: &Context) -> RenderLayer {
        let aspect_ratio = (rig.height as f32) / (rig.width as f32);

        let translation = Mat4::from_translation(Vec3 {
            x: (((rig_layer.x as f32) - (rig.width as f32 / 2.0)
                + (rig_layer.texture.width as f32 / 2.0))
                / rig.width as f32)
                * 2.0,
            y: (((rig_layer.y as f32) - (rig.height as f32 / 2.0)
                + (rig_layer.texture.height as f32 / 2.0))
                / rig.height as f32)
                * -(2.0 * aspect_ratio),
            z: 0.0,
        });
        let scale = Mat4::from_nonuniform_scale(
            (rig_layer.texture.width as f32) / (rig.width as f32),
            ((rig_layer.texture.height as f32) / (rig.height as f32)) * aspect_ratio,
            1.0,
        );

        let mut offset: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        for config in &rig_layer.configs {
            if let Some(o) = config.offset {
                offset += o.into();
            }
        }

        RenderLayer {
            base_transformation: translation.mul(scale.mul(Mat4::from_translation(offset))),
            configs: rig_layer.configs.clone(),
            model: Gm::new(
                Mesh::new(context, &CpuMesh::square()),
                ColorMaterial {
                    texture: Some(Arc::new(Texture2D::new(context, &rig_layer.texture))),
                    is_transparent: true,
                    render_states: RenderStates {
                        blend: Blend::TRANSPARENCY,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ),
        }
    }

    fn from_rig(rig: &Rig, context: &Context) -> Vec<RenderLayer> {
        let mut render_layers = Vec::new();

        for rig_layer in &rig.layers {
            render_layers.push(RenderLayer::from_rig_layer(rig, rig_layer, context))
        }

        render_layers
    }

    fn currently_visible(&self, report: &TrackingReport) -> bool {
        for config in &self.configs {
            if !config.visible {
                return false;
            } else if let Some(rule) = config.invisible_when {
                if rule.apply(report) {
                    return false;
                }
            } else if let Some(rule) = config.visible_when {
                if !rule.apply(report) {
                    return false;
                }
            }
        }
        true
    }

    fn apply_transformation(&mut self, report: &TrackingReport) {
        let mut transformation = self.base_transformation;
        for config in &self.configs {
            transformation = config.transform(report).mul(transformation)
        }
        self.model.set_transformation(transformation);
    }
}

fn handle_input(frame_input: &FrameInput, control_tx: &Sender<ControlMessage>) {
    for event in &frame_input.events {
        match event {
            Event::KeyPress {
                kind: Key::B,
                modifiers: _,
                handled: _,
            } => control_tx.send(ControlMessage::Calibrate).unwrap(),
            Event::KeyPress {
                kind: Key::C,
                modifiers: _,
                handled: _,
            } => eprintln!("this should reset the camera"),
            Event::KeyPress {
                kind: Key::Space,
                modifiers: _,
                handled: _,
            } => eprintln!("this toggle gui elements, once they exist"),
            _ => (),
        }
    }
}

pub fn render(tracking_rx: Receiver<TrackingReport>, control_tx: Sender<ControlMessage>, rig: Rig) {
    let window = Window::new(WindowSettings {
        title: "layertuber".to_string(),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let mut orbit_control = ScaledOrbitControl::new(*camera.target(), 1.0, 3.0, 0.02);

    let mut render_layers: Vec<RenderLayer> = RenderLayer::from_rig(&rig, &context);

    window.render_loop(move |frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);

        orbit_control.handle_events(&mut camera, &frame_input.events);
        handle_input(&frame_input, &control_tx);

        let report = tracking_rx.recv().unwrap();
        let target = frame_input.screen();

        target.clear(ClearState::color_and_depth(0.0, 1.0, 0.0, 1.0, 1.0));

        for render_layer in &mut render_layers {
            if !render_layer.currently_visible(&report) {
                continue;
            }

            render_layer.apply_transformation(&report);
            target.render(&camera, &[&render_layer.model], &[]);
            target.clear(ClearState::depth(1.0));
        }

        FrameOutput::default()
    });
}
