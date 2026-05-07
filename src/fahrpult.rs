use lotus_extra::{
    bb_system::{
        basic::{
            BackBoneResetInputOutput, BackBoneResetType, ModuleInit, ModuleOnMessage, ModuleTick,
        },
        cockpit::{BBButton, Button, ButtonBehaviour},
    },
    input::InputEvent,
};

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

impl ModuleOnMessage<BBFahrpult> for Fahrpult {
    fn on_message(&self, backbone: &mut BBFahrpult, msg: &lotus_script::message::Message) -> bool {
        self.key.on_message(&mut backbone.key, msg)
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
