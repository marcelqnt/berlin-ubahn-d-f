use lotus_extra::{
    bb_system::{
        basic::{
            BackBoneResetInputOutput, BackBoneResetType, ModuleInit, ModuleOnAction, ModuleTick,
        },
        cockpit::{BBButton, Button, ButtonBehaviour},
    },
    input::InputEvent,
};
use lotus_script::action::ActionEvent;

pub struct Fahrpult {
    key: Button,
}

impl Default for Fahrpult {
    fn default() -> Self {
        Self {
            key: Button::new(ButtonBehaviour::OnOff)
                .with_input(InputEvent::new("InsertKey_Reverser", 0))
                .with_visibility_change("Sw_Schluessel")
                .with_sound_press("snd_key_in")
                .with_sound_release("snd_key_out"),
        }
    }
}

impl ModuleTick<BBFahrpult> for Fahrpult {
    fn tick(&self, _: &mut BBFahrpult) {}
}

impl ModuleInit<BBFahrpult> for Fahrpult {
    fn init(&self, backbone: &mut BBFahrpult) {
        self.key.init(&mut backbone.key);
    }
}

impl ModuleOnAction<BBFahrpult> for Fahrpult {
    fn on_action(&self, backbone: &mut BBFahrpult, action: &ActionEvent) -> bool {
        self.key.on_action(&mut backbone.key, action)
    }
}

#[derive(Default)]
pub struct BBFahrpult {
    pub key: BBButton,
}

impl BackBoneResetInputOutput for BBFahrpult {
    fn reset(&mut self, reset_type: BackBoneResetType) {
        self.key.reset(reset_type);
    }
}
