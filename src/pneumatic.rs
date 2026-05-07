use lotus_extra::bb_system::{
    basic::{
        BBSimple, BackBone, BackBoneReset, BackBoneResetInputOutput, BackBoneResetType, ModuleInit,
        ModuleOnMessage, ModuleTick,
    },
    pneumatics::{BBPneumaticSystem, PneumaticSystem, ValveAutomatic},
};

use lotus_script::{message, prelude::*};

use crate::Wagenteil;

const KOMPRESSORWAGEN: Wagenteil = Wagenteil::S;

const V_DURCHGEHENDE_LEITUNGEN: f32 = 0.06;
const V_HAUPTLUFTBEHAELTER: f32 = 1.0;

const V_RATE_COMPRESSOR: f32 = 0.2; //  0.05;

const A_PNEU_A_VENTIL_HAUPTLUFTBEHAELTER_NACH_FUELLLEITUNG: f32 = 0.0001;

const P_KOMPRESSOR_START: f32 = 700000.0;
const P_KOMPRESSOR_STOP: f32 = 800000.0;
const P_FUELLEITUNG_NOMINAL: f32 = 600000.0;

pub struct Pneumatic {
    system: PneumaticSystem,
    // wagenteil: Wagenteil,
    tank_indices: PneumaticTankIndices,
    connection_indices: PneumaticConnectionIndices,
    compressor_index: Option<usize>,
}

pub struct PneumaticTankIndices {
    fuellleitung: usize,
    // bremsleitung: usize,
    hauptluftbehaelter: Option<usize>,
}

pub struct PneumaticConnectionIndices {
    ventil_hauptluftbehaelter_nach_fuellleitung: Option<usize>,
}

impl Pneumatic {
    pub fn new(wagenteil: Wagenteil) -> Self {
        let mut system = PneumaticSystem::default();

        let tanks = PneumaticTankIndices {
            fuellleitung: system.add_tank_get_index(V_DURCHGEHENDE_LEITUNGEN),
            // bremsleitung: system.add_tank_get_index(V_DURCHGEHENDE_LEITUNGEN),
            hauptluftbehaelter: if wagenteil == KOMPRESSORWAGEN {
                Some(system.add_tank_get_index(V_HAUPTLUFTBEHAELTER))
            } else {
                None
            },
        };

        let connections = PneumaticConnectionIndices {
            ventil_hauptluftbehaelter_nach_fuellleitung: if wagenteil == KOMPRESSORWAGEN {
                Some(system.add_connection_get_index(
                    [tanks.hauptluftbehaelter.unwrap(), tanks.fuellleitung],
                    A_PNEU_A_VENTIL_HAUPTLUFTBEHAELTER_NACH_FUELLLEITUNG,
                    ValveAutomatic::Regulator {
                        nominal_pressure: P_FUELLEITUNG_NOMINAL,
                        increase_tolerance: 0.01,
                    },
                ))
            } else {
                None
            },
        };

        let compressor_index = if wagenteil == KOMPRESSORWAGEN {
            Some(system.add_compressor_get_index(
                None,
                Some(tanks.hauptluftbehaelter.unwrap()),
                V_RATE_COMPRESSOR,
            ))
        } else {
            None
        };

        system.add_coupling_get_index(
            message::Coupling::Rear,
            A_PNEU_A_VENTIL_HAUPTLUFTBEHAELTER_NACH_FUELLLEITUNG,
            tanks.fuellleitung,
        );

        system.add_coupling_get_index(
            message::Coupling::Front,
            A_PNEU_A_VENTIL_HAUPTLUFTBEHAELTER_NACH_FUELLLEITUNG,
            tanks.fuellleitung,
        );

        Self {
            system,
            // wagenteil: Wagenteil::Unknown,
            tank_indices: tanks,
            connection_indices: connections,
            compressor_index,
        }
    }
}

impl ModuleTick<BBPneumatic> for Pneumatic {
    fn tick(&self, bb: &mut BBPneumatic) {
        self.system.tick(&mut bb.system);

        // log::info!("compressor_armed: {:?}", bb.compressor_armed.get_state());

        if let Some(compressor_index) = self.compressor_index {
            if bb.compressor_armed.get_state() {
                let compressor_speed = bb.system.compressor_speed(compressor_index);

                if compressor_speed < 0.05
                    && bb
                        .system
                        .tank_pressure(self.tank_indices.hauptluftbehaelter.unwrap())
                        < P_KOMPRESSOR_START
                {
                    bb.system.set_compressor_speed(compressor_index, 1.0);
                } else if compressor_speed > 0.05
                    && bb
                        .system
                        .tank_pressure(self.tank_indices.hauptluftbehaelter.unwrap())
                        > P_KOMPRESSOR_STOP
                {
                    bb.system.set_compressor_speed(compressor_index, 0.0);
                }
            } else if bb.compressor_armed.changed() {
                bb.system.set_compressor_speed(compressor_index, 0.0);
            }
        }

        // Werte reinschreiben in: Pneu_p_Fuellleitung_Pa und Pneu_p_Bremsleitung_Pa

        f32::set_var(
            "Pneu_p_Fuellleitung_Pa",
            bb.system.tank_pressure(self.tank_indices.fuellleitung),
        );
        if let Some(hauptluftbehaelter_index) = self.tank_indices.hauptluftbehaelter {
            f32::set_var(
                "Pneu_p_Bremsleitung_Pa",
                bb.system.tank_pressure(hauptluftbehaelter_index),
            );
        }
    }
}

impl ModuleInit<BBPneumatic> for Pneumatic {
    fn init(&self, bb: &mut BBPneumatic) {
        self.system.init(&mut bb.system);
    }
}

impl ModuleOnMessage<BBPneumatic> for Pneumatic {
    fn on_message(&self, bb: &mut BBPneumatic, msg: &lotus_script::message::Message) -> bool {
        self.system.on_message(&mut bb.system, msg)
    }
}

#[derive(Default)]
pub struct BBPneumatic {
    system: BBPneumaticSystem,
    pub compressor_armed: BBSimple<bool>,
}

impl BackBoneResetInputOutput for BBPneumatic {
    fn reset(&mut self, reset_type: BackBoneResetType) {
        if reset_type == BackBoneResetType::Input {
            self.compressor_armed.reset();
        }
    }
}
