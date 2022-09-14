use self::obs_source::PuppetSource;
use obs_wrapper::{
    module::{LoadContext, Module, ModuleContext},
    obs_register_module, obs_string,
    string::ObsString,
};
use std::sync::mpsc::{channel, sync_channel};
use std::thread;
pub use options::Options;

mod obs_source;
pub mod options;
pub mod puppet;
pub mod tracker;

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

pub fn run_cli() {
    let options = options::Options::from_arguments();

    let (report_tx, report_rx) = sync_channel(0);
    let (control_tx, control_rx) = channel();

    let tracker_options = options.clone();
    thread::spawn(move || {
        let tracker =
            tracker::run_tracker(control_rx, &tracker_options).expect("could not start tracker");
        for report in tracker {
            report_tx.send(report).unwrap()
        }
    });

    puppet::run_puppet(options.path.as_path(), report_rx, control_tx);
}
