use obs_wrapper::{
    data::DataObj,
    obs_string,
    properties::{NumberProp, PathProp, PathType, Properties},
    source::*,
    string::ObsString,
};
use std::borrow::Cow;

pub struct PuppetSource {
    width: u32,
    height: u32,
    path: Option<String>,
}

impl PuppetSource {
    fn update_settings(&mut self, settings: &DataObj) {
        if let Some(width) = settings.get(obs_string!("width")) {
            self.width = width
        }

        if let Some(height) = settings.get(obs_string!("height")) {
            self.height = height
        }

        let path: Option<Cow<'_, str>> = settings.get(obs_string!("path"));
        self.path = path.map(|p| p.into_owned())
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
            width: 100,
            height: 100,
            path: None,
        };
        source.update_settings(&create.settings);

        source
    }
}

impl GetPropertiesSource for PuppetSource {
    fn get_properties(&mut self) -> Properties {
        let mut properties = Properties::new();

        properties.add(
            obs_string!("puppet"),
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

        properties
    }
}

impl ActivateSource for PuppetSource {
    fn activate(&mut self) {
        eprintln!("activating")
    }
}

impl DeactivateSource for PuppetSource {
    fn deactivate(&mut self) {
        eprintln!("deactivating")
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
