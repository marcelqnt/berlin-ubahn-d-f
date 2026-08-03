use lotus_extra::bb_system::{VehicleInterface, basic::BackBone};
use lotus_script::{log, message::Message};

use crate::{Backbone, Modules};

#[derive(Default)]
pub struct DfInterface;

impl VehicleInterface<Modules, Backbone> for DfInterface {
    fn after_init(&mut self, _modules: &Modules, _backbone: &mut Backbone) {
        log::info!("Initialized");
    }

    fn tick_interface(&mut self, _modules: &Modules, backbone: &mut Backbone) {
        self.pneumatics_input(backbone);
    }

    fn interface_on_message(
        &mut self,
        _modules: &Modules,
        _backbone: &mut Backbone,
        _msg: &Message,
    ) {
    }
}

impl DfInterface {
    fn pneumatics_input(&mut self, backbone: &mut Backbone) {
        let fahrpult = &mut backbone.fahrpult;
        let pneumatic = &mut backbone.pneumatic;

        if fahrpult
            .key
            .state()
            .forward_on_changed(&mut pneumatic.compressor_armed)
        {
            log::info!(
                "compressor_armed changed: {:?}",
                pneumatic.compressor_armed.get_state()
            );
        }
    }
}
