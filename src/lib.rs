use lotus_extra::bb_system::{
    basic::{
        BackBoneResetInputOutput, BackBoneResetType, ModuleInit, ModuleOnMessage, ModuleTick,
        handle_message,
    },
    pneumatics::BBPneumaticSystem,
};
use lotus_script::{action::ActionEvent, prelude::*, vehicle::spawned_inverted_to_train};

use crate::{
    fahrpult::{BBFahrpult, Fahrpult},
    pneumatic::{BBPneumatic, Pneumatic},
};

mod fahrpult;
mod interface;
mod pneumatic;

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub enum Wagenteil {
    #[default]
    Unknown,
    S,
    K,
}

impl Wagenteil {
    pub fn is_s(&self) -> bool {
        *self == Wagenteil::S
    }

    pub fn is_k(&self) -> bool {
        *self == Wagenteil::K
    }
}

#[derive(Default)]
pub struct MyScript {
    modules: Modules,
    backbone: Backbone,
}

script!(MyScript);

impl Script for MyScript {
    fn init(&mut self) {
        self.modules.init(&mut self.backbone);

        log::info!("Initialized");
    }

    fn tick(&mut self) {
        self.backbone.reset_inputs();

        self.modules.tick_interface(&mut self.backbone);

        self.backbone.reset_outputs();

        self.modules.tick(&mut self.backbone);

        set_var("veh_number", "1");
    }

    fn on_message(&mut self, msg: lotus_script::message::Message) {
        // if msg.source().coupling.is_none() {
        //     log::info!("Message: {:?}", msg);
        // }

        // handle_message(&msg, |a: ActionEvent| -> bool {
        //     log::info!("ActionState: {:?}", a);
        //     true
        // });

        self.modules.on_message(&mut self.backbone, &msg);
        self.modules
            .fahrpult
            .on_message(&mut self.backbone.fahrpult, &msg);
    }
}

struct Modules {
    pneumatic: Pneumatic,
    fahrpult: Fahrpult,
}

impl Default for Modules {
    fn default() -> Self {
        Self {
            pneumatic: Pneumatic::new(Self::set_and_return_wagenteil()),
            fahrpult: Fahrpult::default(),
        }
    }
}

impl Modules {
    fn set_and_return_wagenteil() -> Wagenteil {
        let is_k = spawned_inverted_to_train();

        set_var("IsK", is_k);
        set_var("IsS", !is_k);
        set_var("veh_number", if is_k { "2679" } else { "2678" });

        if is_k { Wagenteil::K } else { Wagenteil::S }
    }
}

impl ModuleTick<Backbone> for Modules {
    fn tick(&self, bb: &mut Backbone) {
        self.pneumatic.tick(&mut bb.pneumatic);
        self.fahrpult.tick(&mut bb.fahrpult);
    }
}

impl ModuleInit<Backbone> for Modules {
    fn init(&self, bb: &mut Backbone) {
        self.pneumatic.init(&mut bb.pneumatic);
        self.fahrpult.init(&mut bb.fahrpult);
    }
}

impl ModuleOnMessage<Backbone> for Modules {
    fn on_message(&self, backbone: &mut Backbone, msg: &lotus_script::message::Message) -> bool {
        self.pneumatic.on_message(&mut backbone.pneumatic, msg)
    }
}

#[derive(Default)]
struct Backbone {
    pneumatic: BBPneumatic,
    fahrpult: BBFahrpult,
}

impl BackBoneResetInputOutput for Backbone {
    fn reset(&mut self, reset_type: BackBoneResetType) {
        self.pneumatic.reset(reset_type);
        self.fahrpult.reset(reset_type);
    }
}
