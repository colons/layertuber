use obs_wrapper::{
    module::{LoadContext, Module, ModuleContext},
    obs_register_module, obs_string,
    source::{CreatableSourceContext, GetNameSource, SourceContext, SourceType, Sourceable},
    string::ObsString,
};

struct PuppetSource;

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
