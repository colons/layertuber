use crate::puppet::rig::Rig;
use crate::tracker::TrackingReport;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use three_d::window::{Window, WindowSettings};
use three_d::{
    degrees, vec3, Blend, Camera, ClearState, ColorMaterial, CpuMesh, FrameInput, FrameOutput, Gm,
    Mat4, Mesh, Quaternion, RenderStates, Texture2D,
};

struct RenderLayer {
    model: Gm<Mesh, ColorMaterial>,
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

    let mut render_layers: Vec<RenderLayer> = Vec::new();

    for rig_layer in &rig.layers {
        render_layers.push(RenderLayer {
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
        })
    }

    window.render_loop(move |frame_input: FrameInput| {
        let report = rx.recv().unwrap();
        let target = frame_input.screen();

        camera.set_viewport(frame_input.viewport);

        target.clear(ClearState::color_and_depth(0.0, 1.0, 0.0, 1.0, 1.0));

        for render_layer in &mut render_layers {
            render_layer
                .model
                .set_transformation(Mat4::from(Quaternion::new(
                    report.head_rotation[3],
                    report.head_rotation[0],
                    report.head_rotation[1],
                    report.head_rotation[2],
                )));
            target.render(&camera, &[&render_layer.model], &[]);
            target.clear(ClearState::depth(1.0));
        }

        FrameOutput::default()
    });
}
