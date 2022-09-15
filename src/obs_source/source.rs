use crate::puppet::Rig;
use obs_wrapper::{
    data::DataObj,
    obs_string,
    properties::{BoolProp, NumberProp, PathProp, PathType, Properties},
    source::*,
    string::ObsString,
};
use std::borrow::Cow;
use std::path::Path;

pub struct PuppetSource {
    path: Option<String>,
    width: u32,
    height: u32,
    camera_index: u8,
    show_features: bool,
    rig: Option<Rig>,
}

impl PuppetSource {
    fn reload_rig(&mut self) {
        // this should happen asyncronously, but i need to learn things first
        self.rig = match &self.path {
            Some(p) => {
                match Rig::open(Path::new(p.as_str())) {
                    Ok(r) => Some(r),
                    Err(e) => {
                        eprintln!("failed to load rig: {}", e);
                        None
                    }
                }
            }
            None => {
                eprintln!("path not set");
                None
            }
        }
    }

    fn update_settings(&mut self, settings: &DataObj) {
        let path: Option<Cow<'_, str>> = settings.get(obs_string!("path"));
        self.path = path.map(|p| p.into_owned());

        if let Some(width) = settings.get(obs_string!("width")) {
            self.width = width
        }

        if let Some(height) = settings.get(obs_string!("height")) {
            self.height = height
        }

        if let Some(camera_index) = settings.get(obs_string!("camera_index")) {
            self.camera_index = camera_index
        }

        if let Some(show_features) = settings.get(obs_string!("show_features")) {
            self.show_features = show_features
        }

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
            path: None,
            width: 100,
            height: 100,
            camera_index: 0,
            show_features: false,
            rig: None,
        };
        source.update_settings(&create.settings);

        source
    }
}

impl GetPropertiesSource for PuppetSource {
    fn get_properties(&mut self) -> Properties {
        let mut properties = Properties::new();

        properties.add(
            obs_string!("path"),
            obs_string!("Puppet .ora file"),
            PathProp::new(PathType::File),
        );

        properties.add(
            obs_string!("width"),
            obs_string!("Render width (in pixels)"),
            NumberProp::new_int().with_range(100..(2_u32).pow(16)),
        );

        properties.add(
            obs_string!("height"),
            obs_string!("Render height (in pixels)"),
            NumberProp::new_int().with_range(100..(2_u32).pow(16)),
        );

        properties.add(
            obs_string!("camera_index"),
            obs_string!("Camera index"),
            NumberProp::new_int().with_range(0..64),
        );

        properties.add(
            obs_string!("show_features"),
            obs_string!("Show features"),
            BoolProp,
        );

        properties
    }
}

impl ActivateSource for PuppetSource {
    fn activate(&mut self) {
        eprintln!("activating...");
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
        self.width
    }
}

impl GetHeightSource for PuppetSource {
    fn get_height(&mut self) -> u32 {
        self.height
    }
}

impl UpdateSource for PuppetSource {
    fn update(&mut self, settings: &mut DataObj, _context: &mut GlobalContext) {
        self.update_settings(settings);
    }
}

impl VideoRenderSource for PuppetSource {
    fn video_render(&mut self, _context: &mut GlobalContext, _render: &mut VideoRenderContext) {
        // eprintln!("helo");
    }
}

impl GetNameSource for PuppetSource {
    fn get_name() -> ObsString {
        obs_string!("layertuber puppet")
    }
}
