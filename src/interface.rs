use lotus_extra::bb_system::basic::{BackBone, BackBoneForwarding};
use lotus_script::log;

use crate::{Backbone, Modules};

impl Modules {
    pub fn tick_interface(&mut self, backbone: &mut Backbone) {
        self.pneumatics_input(backbone);
    }

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
