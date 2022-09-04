use crate::puppet::rig::Rig;
use crate::tracker::TrackingReport;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use three_d::window::{Window, WindowSettings};
use three_d::{
    degrees, vec3, Camera, ClearState, ColorMaterial, CpuMesh, FrameInput, FrameOutput, Gm, Mat4,
    Mesh, Quaternion, Texture2D,
};

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

    let mut model = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            texture: Some(Arc::new(Texture2D::new(&context, &rig.layers[1].texture))),
            ..Default::default()
        },
    );

    window.render_loop(move |frame_input: FrameInput| {
        let report = rx.recv().unwrap();

        camera.set_viewport(frame_input.viewport);

        model.set_transformation(Mat4::from(Quaternion::new(
            report.head_rotation[3],
            report.head_rotation[0],
            report.head_rotation[1],
            report.head_rotation[2],
        )));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 1.0, 0.0, 1.0, 1.0))
            .render(&camera, &[&model], &[]);

        FrameOutput::default()
    });
}
