use lotus_extra::bb_system::{
    basic::{ModuleInit, ModuleTick},
    pneumatics::{BBPneumaticSystem, PneumaticSystem, ValveAutomatic},
};

use lotus_script::{message, prelude::*};

use crate::Wagenteil;

const KOMPRESSORWAGEN: Wagenteil = Wagenteil::S;

const V_DURCHGEHENDE_LEITUNGEN: f32 = 0.06;
const V_HAUPTLUFTBEHAELTER: f32 = 1.0;

const V_RATE_COMPRESSOR: f32 = 0.05;

const A_PNEU_A_VENTIL_HAUPTLUFTBEHAELTER_NACH_FUELLLEITUNG: f32 = 0.0001;

const P_KOMPRESSOR_START: f32 = 700000.0;
const P_KOMPRESSOR_STOP: f32 = 800000.0;
const P_FUELLEITUNG_NOMINAL: f32 = 600000.0;

pub struct Pneumatic {
    system: PneumaticSystem,
    wagenteil: Wagenteil,

    tanks: PneumaticTankIndices,
    connections: PneumaticConnectionIndices,
}

pub struct PneumaticTankIndices {
    fuellleitung: usize,
    // bremsleitung: usize,
    hauptluftbehaelter: usize,
}

pub struct PneumaticConnectionIndices {
    ventil_hauptluftbehaelter_nach_fuellleitung: usize,
}

impl Default for Pneumatic {
    fn default() -> Self {
        let mut system = PneumaticSystem::default();

        let tanks = PneumaticTankIndices {
            fuellleitung: system.add_tank_get_index(V_DURCHGEHENDE_LEITUNGEN),
            // bremsleitung: system.add_tank_get_index(V_DURCHGEHENDE_LEITUNGEN),
            hauptluftbehaelter: system.add_tank_get_index(V_HAUPTLUFTBEHAELTER),
        };

        let connections = PneumaticConnectionIndices {
            ventil_hauptluftbehaelter_nach_fuellleitung: system.add_connection_get_index(
                [tanks.hauptluftbehaelter, tanks.fuellleitung],
                A_PNEU_A_VENTIL_HAUPTLUFTBEHAELTER_NACH_FUELLLEITUNG,
                ValveAutomatic::Manual,
            ),
        };

        system.add_compressor_get_index(None, Some(tanks.hauptluftbehaelter), V_RATE_COMPRESSOR);

        system.add_coupling_get_index(
            message::Coupling::Rear,
            A_PNEU_A_VENTIL_HAUPTLUFTBEHAELTER_NACH_FUELLLEITUNG,
            tanks.fuellleitung,
        );

        Self {
            system,
            wagenteil: Wagenteil::Unknown,
            tanks,
            connections,
        }
    }
}

impl ModuleTick<BBPneumaticSystem> for Pneumatic {
    fn tick(&self, bb: &mut BBPneumaticSystem) {
        self.system.tick(bb);

        if self.wagenteil == KOMPRESSORWAGEN {
            let compressor_speed = bb.compressor_speed(0);

            if compressor_speed < 0.05
                && bb.tank_pressure(self.tanks.hauptluftbehaelter) < P_KOMPRESSOR_START
            {
                bb.set_compressor_speed(0, 1.0);
            } else if compressor_speed > 0.05
                && bb.tank_pressure(self.tanks.hauptluftbehaelter) > P_KOMPRESSOR_STOP
            {
                bb.set_compressor_speed(0, 0.0);
            }
        }

        // Werte reinschreiben in: Pneu_p_Fuellleitung_Pa und Pneu_p_Bremsleitung_Pa

        f32::set_var(
            "Pneu_p_Fuellleitung_Pa",
            bb.tank_pressure(self.tanks.fuellleitung),
        );
        f32::set_var(
            "Pneu_p_Bremsleitung_Pa",
            bb.tank_pressure(self.tanks.hauptluftbehaelter),
        );
    }
}

impl ModuleInit<BBPneumaticSystem> for Pneumatic {
    fn init(&self, bb: &mut BBPneumaticSystem) {
        self.system.init(bb);
    }
}

impl Pneumatic {
    pub fn set_wagenteil(&mut self, wagenteil: Wagenteil) {
        self.wagenteil = wagenteil;

        if self.wagenteil == KOMPRESSORWAGEN {
            self.system.set_valve_type(
                self.connections.ventil_hauptluftbehaelter_nach_fuellleitung,
                ValveAutomatic::Regulator {
                    nominal_pressure: P_FUELLEITUNG_NOMINAL,
                    increase_tolerance: 0.01,
                },
            );
        }
    }

    pub fn on_message(&self, bb: &mut BBPneumaticSystem, msg: lotus_script::message::Message) {
        self.system.on_message(bb, msg);
    }
}
