use lotus_extra::bb_system::{
    basic::{ModuleInit, ModuleOnMessage, ModuleTick},
    pneumatics::BBPneumaticSystem,
};
use lotus_script::prelude::*;

use crate::pneumatic::Pneumatic;

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
    wagen_teil: Wagenteil,
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
        self.modules.tick(&mut self.backbone);

        set_var("veh_number", "1");
    }

    fn on_message(&mut self, msg: lotus_script::message::Message) {
        log::info!("Message: {:?}", msg);

        msg.handle(|c: vehicle::TrainConfigurationChanged| {
            let is_k = c.reversed_to_train;
            self.wagen_teil = if is_k { Wagenteil::K } else { Wagenteil::S };
            set_var("IsK", is_k);
            set_var("IsS", !is_k);
            set_var("veh_number", if is_k { "2679" } else { "2678" });
            self.modules
                .set_wagenteil(if is_k { Wagenteil::K } else { Wagenteil::S });
            Ok(())
        })
        .unwrap();

        self.modules.on_message(&mut self.backbone, &msg);
    }
}

#[derive(Default)]
struct Modules {
    pneumatic: Pneumatic,
}

impl Modules {
    fn set_wagenteil(&mut self, wagenteil: Wagenteil) {
        self.pneumatic.set_wagenteil(wagenteil);
    }
}

impl ModuleTick<Backbone> for Modules {
    fn tick(&self, bb: &mut Backbone) {
        self.pneumatic.tick(&mut bb.pneumatic);
    }
}

impl ModuleInit<Backbone> for Modules {
    fn init(&self, bb: &mut Backbone) {
        self.pneumatic.init(&mut bb.pneumatic);
    }
}

impl ModuleOnMessage<Backbone> for Modules {
    fn on_message(&self, backbone: &mut Backbone, msg: &lotus_script::message::Message) {
        self.pneumatic.on_message(&mut backbone.pneumatic, msg);
    }
}

#[derive(Default)]
struct Backbone {
    pneumatic: BBPneumaticSystem,
}
