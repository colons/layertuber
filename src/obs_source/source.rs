use crate::puppet::Rig;
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

const SETTING_PATH: ObsString = obs_string!("path");
const SETTING_WIDTH: ObsString = obs_string!("width");
const SETTING_HEIGHT: ObsString = obs_string!("height");
const SETTING_CAMERA_INDEX: ObsString = obs_string!("camera_index");
const SETTING_SHOW_FEATURES: ObsString = obs_string!("show_features");

pub struct PuppetSource {
    tex: GraphicsTexture,
    path: Option<String>,
    camera_index: u8,
    show_features: bool,
    rig: Option<Rig>,
}

impl PuppetSource {
    fn reload_rig(&mut self) {
        // XXX this should happen asyncronously, but i need to learn things first
        self.rig = match &self.path {
            Some(p) => match Rig::open(Path::new(p.as_str())) {
                Ok(r) => Some(r),
                Err(e) => {
                    error!("failed to load rig: {}", e);
                    None
                }
            },
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
            rig: None,
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
        self.reload_rig();
    }
}

impl DeactivateSource for PuppetSource {
    fn deactivate(&mut self) {
        self.rig = None
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
