use crate::obs_source::PuppetSource;
pub use crate::options::Options;
use obs_wrapper::{
    log::Logger,
    module::{LoadContext, Module, ModuleContext},
    obs_register_module, obs_string,
    string::ObsString,
};

pub mod cli;
pub mod options;
pub mod puppet;
pub mod tracker;

mod obs_source;

struct LayertuberModule {
    context: ModuleContext,
}

impl Module for LayertuberModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        if let Ok(()) = Logger::new().init() {
            log::set_max_level(log::LevelFilter::Info)
        }

        let source = load_context
            .create_source_builder::<PuppetSource>()
            .enable_get_name()
            .enable_video_render()
            .enable_activate()
            .enable_deactivate()
            .enable_get_width()
            .enable_get_height()
            .enable_get_properties()
            .enable_update()
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
