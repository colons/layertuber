use obs_wrapper::{
    module::{LoadContext, Module, ModuleContext},
    obs_register_module, obs_string,
    properties::{NumberProp, PathProp, PathType, Properties},
    source::*,
    string::ObsString,
};

struct PuppetSource;

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
        200
    }
}

impl GetHeightSource for PuppetSource {
    fn get_height(&mut self) -> u32 {
        200
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
            NumberProp::new_int().with_range(100..(2_usize).pow(16))
        );

        properties.add(
            obs_string!("height"),
            obs_string!("Render height (in pixels)"),
            NumberProp::new_int().with_range(100..(2_usize).pow(16))
        );

        properties
    }
}

impl Sourceable for PuppetSource {
    fn get_id() -> ObsString {
        obs_string!("layertuber_puppet")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }

    fn create(_create: &mut CreatableSourceContext<Self>, _source: SourceContext) -> Self {
        PuppetSource {}
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

struct LayertuberModule {
    context: ModuleContext,
}

impl Module for LayertuberModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        let source = load_context
            .create_source_builder::<PuppetSource>()
            .enable_get_name()
            .enable_video_render()
            .enable_activate()
            .enable_deactivate()
            .enable_get_width()
            .enable_get_height()
            .enable_get_properties()
            .build();
        load_context.register_source(source);
        true
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    fn description() -> ObsString {
        obs_string!("A way to use layertuber puppets as an source in OBS")
    }

    fn name() -> ObsString {
        obs_string!("layertuber")
    }

    fn author() -> ObsString {
        obs_string!("colons")
    }
}

obs_register_module!(LayertuberModule);
