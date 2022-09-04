use crate::puppet::rig::{Rig, RigLayer};
use crate::tracker::TrackingReport;
use core::ops::Mul;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use three_d::window::{Window, WindowSettings};
use three_d::{
    degrees, vec3, Blend, Camera, ClearState, ColorMaterial, Context, CpuMesh, FrameInput,
    FrameOutput, Gm, Mat4, Mesh, Quaternion, RenderStates, SquareMatrix, Texture2D, Vec3,
};

struct RenderLayer {
    model: Gm<Mesh, ColorMaterial>,
    base_transformation: Mat4,
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

    fn rotation(&mut self, report: &TrackingReport) -> Mat4 {
        Mat4::from(Quaternion::new(
            // this ordering is weird because scipy deals in quats scalar-last, but three_d's are scalar-first
            report.head_rotation[3],
            // invert these to make the puppet behave like a mirror
            -report.head_rotation[0], // pitch
            -report.head_rotation[1], // yaw
            -report.head_rotation[2], // roll
        ))
    }

    fn apply_transformation(&mut self, report: &TrackingReport) -> () {
        let transformation = self.rotation(report).mul(self.base_transformation);
        self.model.set_transformation(transformation);
    }
}

pub fn render(rx: Receiver<TrackingReport>, rig: Rig) {
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
        let report = rx.recv().unwrap();
        let target = frame_input.screen();

        camera.set_viewport(frame_input.viewport);

        target.clear(ClearState::color_and_depth(0.0, 1.0, 0.0, 1.0, 1.0));

        for render_layer in &mut render_layers {
            render_layer.apply_transformation(&report);
            target.render(&camera, &[&render_layer.model], &[]);
            target.clear(ClearState::depth(1.0));
        }

        FrameOutput::default()
    });
}
