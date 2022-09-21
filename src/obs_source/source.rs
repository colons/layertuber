use crate::{puppet::Rig, tracker::ControlMessage};
use log::{error, info};
use obs_wrapper::{
    data::DataObj,
    graphics::{GraphicsColorFormat, GraphicsTexture},
    obs_string,
    properties::{BoolProp, NumberProp, PathProp, PathType, Properties},
    source::*,
    string::ObsString,
};
use std::borrow::Cow;
use std::path::Path;
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::thread;

const SETTING_PATH: ObsString = obs_string!("path");
const SETTING_WIDTH: ObsString = obs_string!("width");
const SETTING_HEIGHT: ObsString = obs_string!("height");
const SETTING_CAMERA_INDEX: ObsString = obs_string!("camera_index");
const SETTING_SHOW_FEATURES: ObsString = obs_string!("show_features");

struct RenderThread {
    thread: thread::Thread,
    control_tx: Sender<ControlMessage>,
    frame_rx: Receiver<Box<[u8]>>,
}

pub struct PuppetSource {
    tex: GraphicsTexture,
    path: Option<String>,
    camera_index: u8,
    show_features: bool,
    render_thread: Option<RenderThread>,
}

fn render_thread(rig_path: &Path) -> RenderThread {
    let rig_path = rig_path.to_owned();

    let (frame_tx, frame_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    RenderThread {
        control_tx,
        frame_rx,
        thread: thread::spawn(move || {
            let rig = match Rig::open(rig_path.as_path()) {
                Ok(r) => r,
                Err(e) => {
                    error!("error loading rig: {}", e);
                    panic!("this should show an error message in OBS somehow");
                }
            };
            dbg!(&rig);
        })
        .thread()
        .to_owned(),
    }
}

impl PuppetSource {
    fn kill_render_thread(&mut self) {
        // this might need to send a signal to the render thread to end? the docs don't say what
        // happens when a Thread object is dropped, only JoinHandle
        self.render_thread = None;
    }

    fn spawn_render_thread(&mut self) {
        self.render_thread = match &self.path {
            Some(p) => Some(render_thread(Path::new(p.as_str()))),
            None => {
                info!("path not set");
                None
            }
        }
    }

    fn update_settings(&mut self, settings: &DataObj) {
        let path: Option<Cow<'_, str>> = settings.get(SETTING_PATH);
        self.path = path.map(|p| p.into_owned());

        if let Some(width) = settings.get(SETTING_WIDTH) {
            if let Some(height) = settings.get(SETTING_HEIGHT) {
                self.tex = GraphicsTexture::new(width, height, GraphicsColorFormat::RGBA);
            }
        }

        if let Some(camera_index) = settings.get(SETTING_CAMERA_INDEX) {
            self.camera_index = camera_index
        }

        if let Some(show_features) = settings.get(SETTING_SHOW_FEATURES) {
            self.show_features = show_features
        }

        self.render();
    }

    fn render(&mut self) {
        let mut pixels = Vec::new();

        for row in 0..self.tex.height() {
            for col in 0..self.tex.width() {
                pixels.extend_from_slice(&[
                    (row as f32 / self.tex.height() as f32 * 256.0) as u8,
                    (col as f32 / self.tex.width() as f32 * 256.0) as u8,
                    14,
                    255,
                ])
            }
        }

        self.tex
            .set_image(pixels.as_slice(), self.tex.width() * 4, false);
    }
}

impl Sourceable for PuppetSource {
    fn get_id() -> ObsString {
        obs_string!("layertuber_puppet")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }

    fn create(create: &mut CreatableSourceContext<Self>, _source: SourceContext) -> Self {
        let mut source = PuppetSource {
            tex: GraphicsTexture::new(100, 100, GraphicsColorFormat::RGBA),
            path: None,
            camera_index: 0,
            show_features: false,
            render_thread: None,
        };
        source.update_settings(&create.settings);

        create.register_hotkey(
            obs_string!("calibrate"),
            obs_string!("Reset puppet position"),
            |key, _data| {
                if key.pressed {
                    // XXX implement this
                    info!("calibration requested")
                }
            },
        );

        source
    }
}

impl GetPropertiesSource for PuppetSource {
    fn get_properties(&mut self) -> Properties {
        let mut properties = Properties::new();

        properties.add(
            SETTING_PATH,
            obs_string!("Puppet .ora file"),
            PathProp::new(PathType::File),
        );

        properties.add(
            SETTING_WIDTH,
            obs_string!("Render width (in pixels)"),
            NumberProp::new_int().with_range(100..(2_u32).pow(16)),
        );

        properties.add(
            SETTING_HEIGHT,
            obs_string!("Render height (in pixels)"),
            NumberProp::new_int().with_range(100..(2_u32).pow(16)),
        );

        properties.add(
            SETTING_CAMERA_INDEX,
            obs_string!("Camera index"),
            NumberProp::new_int().with_range(0..64),
        );

        properties.add(
            SETTING_SHOW_FEATURES,
            obs_string!("Show features"),
            BoolProp,
        );

        properties
    }
}

impl ActivateSource for PuppetSource {
    fn activate(&mut self) {
        info!("activating...");
        self.spawn_render_thread();
    }
}

impl DeactivateSource for PuppetSource {
    fn deactivate(&mut self) {
        self.kill_render_thread();
    }
}

impl GetWidthSource for PuppetSource {
    fn get_width(&mut self) -> u32 {
        self.tex.width()
    }
}

impl GetHeightSource for PuppetSource {
    fn get_height(&mut self) -> u32 {
        self.tex.height()
    }
}

impl UpdateSource for PuppetSource {
    fn update(&mut self, settings: &mut DataObj, _context: &mut GlobalContext) {
        self.update_settings(settings);
    }
}

impl VideoRenderSource for PuppetSource {
    fn video_render(&mut self, _context: &mut GlobalContext, _render: &mut VideoRenderContext) {
        self.tex
            .draw(0, 0, self.tex.width(), self.tex.height(), false);
    }
}

impl GetNameSource for PuppetSource {
    fn get_name() -> ObsString {
        obs_string!("layertuber puppet")
    }
}
