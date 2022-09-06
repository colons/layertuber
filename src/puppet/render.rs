use crate::puppet::config::{LayerConfig, Rule};
use crate::puppet::rig::{Rig, RigLayer};
use crate::tracker::{ControlMessage, QuatSource, Source, TrackingReport};
use core::ops::Mul;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use three_d::window::{Window, WindowSettings};
use three_d::{
    degrees, vec3, Blend, Camera, ClearState, ColorMaterial, Context, CpuMesh, Event, FrameInput,
    FrameOutput, Gm, Key, Mat4, Mesh, RenderStates, SquareMatrix, Texture2D, Vec3,
};

struct RenderLayer {
    model: Gm<Mesh, ColorMaterial>,
    base_transformation: Mat4,
    config: LayerConfig,
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

        RenderLayer {
            base_transformation: translation.mul(scale.mul(Mat4::identity())),
            config: rig_layer.config,
            model: Gm::new(
                Mesh::new(&context, &CpuMesh::square()),
                ColorMaterial {
                    texture: Some(Arc::new(Texture2D::new(&context, &rig_layer.texture))),
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
        if !self.config.visible {
            return false;
        }

        match self.config.invisible_when {
            None => (),
            Some(rule) => {
                if rule.apply(&report) {
                    return false;
                }
            }
        }

        match self.config.visible_when {
            None => (),
            Some(rule) => {
                if !rule.apply(&report) {
                    return false;
                }
            }
        }
        true
    }

    fn rotation(&mut self, report: &TrackingReport) -> Mat4 {
        // XXX this should respect configured rules
        Mat4::from(QuatSource::HeadRotation.value(report))
    }

    fn apply_transformation(&mut self, report: &TrackingReport) -> () {
        let transformation = self.rotation(report).mul(self.base_transformation);
        self.model.set_transformation(transformation);
    }
}

fn handle_input(frame_input: &FrameInput, control_tx: &Sender<ControlMessage>) {
    for event in &frame_input.events {
        match event {
            Event::KeyPress {
                kind,
                modifiers: _,
                handled: _,
            } => match kind {
                Key::C => {
                    control_tx.send(ControlMessage::Calibrate).unwrap();
                }
                _ => (),
            },
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

    let mut render_layers: Vec<RenderLayer> = RenderLayer::from_rig(&rig, &context);

    window.render_loop(move |frame_input: FrameInput| {
        handle_input(&frame_input, &control_tx);

        let report = tracking_rx.recv().unwrap();
        let target = frame_input.screen();

        camera.set_viewport(frame_input.viewport);

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
