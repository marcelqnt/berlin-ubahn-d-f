use lotus_extra::bb_modules;
use lotus_extra::bb_system::{BBVehicle, TickExtra};
use lotus_script::{
    Script, message::Message, prelude::*, script, vehicle::spawned_inverted_to_train,
};

use crate::{
    fahrpult::{BBFahrpult, Fahrpult},
    interface::DfInterface,
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

struct DfExtras {
    wagenteil: Wagenteil,
}

impl Default for DfExtras {
    fn default() -> Self {
        Self {
            wagenteil: determine_wagenteil(),
        }
    }
}

impl TickExtra for DfExtras {}

fn determine_wagenteil() -> Wagenteil {
    let wagenteil = if spawned_inverted_to_train() {
        Wagenteil::K
    } else {
        Wagenteil::S
    };

    apply_wagenteil_vars(wagenteil);

    wagenteil
}

fn apply_wagenteil_vars(wagenteil: Wagenteil) {
    let is_k = wagenteil.is_k();

    set_var("IsK", is_k);
    set_var("IsS", !is_k);

    let veh_number = if is_k { "2679" } else { "2678" };
    set_var::<String>("veh_number", veh_number.into());
    set_var::<String>("VehNr_String", veh_number.into());
}

type BbVehicle = BBVehicle<Modules, Backbone, DfExtras, DfInterface>;

pub struct MyScript(BbVehicle);

impl Default for MyScript {
    fn default() -> Self {
        let extras = DfExtras::default();
        Self(BBVehicle::new(
            Modules::new(extras.wagenteil),
            Backbone::default(),
            extras,
            DfInterface::default(),
        ))
    }
}

impl Script for MyScript {
    fn init(&mut self) {
        self.0.init();
    }

    fn tick(&mut self) {
        self.0.tick();
    }

    fn on_message(&mut self, msg: Message) {
        self.0.on_message(msg);
    }
}

script!(MyScript);

pub struct Modules {
    pneumatic: Pneumatic,
    fahrpult: Fahrpult,
}

impl Modules {
    fn new(wagenteil: Wagenteil) -> Self {
        Self {
            pneumatic: Pneumatic::new(wagenteil),
            fahrpult: Fahrpult::default(),
        }
    }
}

bb_modules! {
    Modules => Backbone {
        tick {
            pneumatic => pneumatic;
            fahrpult => fahrpult;
        }
        init {
            pneumatic => pneumatic;
            fahrpult => fahrpult;
        }
        on_action {
            fahrpult => fahrpult;
        }
        on_message {
            pneumatic => pneumatic;
        }
        reset {
            pneumatic, fahrpult,
        }
    }
}

#[derive(Default)]
pub struct Backbone {
    pub pneumatic: BBPneumatic,
    pub fahrpult: BBFahrpult,
}
