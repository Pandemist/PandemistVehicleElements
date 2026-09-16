use lotus_extra::{messages::pis::Announcement, vehicle::CockpitSide};
use lotus_script::{
    content::ContentId,
    message::{message_type, Message},
};
use serde::{Deserialize, Serialize};

use crate::api::{sound::SoundWithStartAndEnd, variable::Variable};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnnouncementDriver {
    pub value: Vec<ContentId>,
    pub cab: Option<CockpitSide>,
}

message_type!(AnnouncementDriver, "Std_ELA", "AnnouncementDriver");

pub enum AnnouncementDestionation {
    DriverCab(Option<CockpitSide>),
    Cabin,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpeakerTarget {
    Int,
    Ext,
    IntExt,
    Extr,
    Extl,
}

pub struct AnnouncementManagerBuilder {
    power: bool,

    destination: AnnouncementDestionation,

    int_speaker_sounds: SoundWithStartAndEnd,
    ext_speaker_sounds: SoundWithStartAndEnd,
    ext_r_speaker_sounds: SoundWithStartAndEnd,
    ext_l_speaker_sounds: SoundWithStartAndEnd,

    int_speaker: Variable<ContentId>,
    ext_speaker: Variable<ContentId>,
    ext_r_speaker: Variable<ContentId>,
    ext_l_speaker: Variable<ContentId>,

    speaker_target: SpeakerTarget,
    annoucement_target: SpeakerTarget,

    announcement_vol: Variable<f32>,
    int_speaker_vol: Variable<f32>,
    ext_speaker_vol: Variable<f32>,

    announcement_vol_var: f32,
    int_speaker_vol_var: f32,
    ext_speaker_vol_var: f32,
}

impl AnnouncementManagerBuilder {
    pub fn add_ext_speaker(mut self, ext_speaker_name: &str) -> Self {
        self.ext_speaker = Variable::new(ext_speaker_name);
        self
    }

    pub fn add_ext_speaker_l_r(
        mut self,
        ext_r_speaker_name: &str,
        ext_l_speaker_name: &str,
    ) -> Self {
        self.ext_r_speaker = Variable::new(ext_r_speaker_name);
        self.ext_l_speaker = Variable::new(ext_l_speaker_name);
        self
    }

    pub fn add_announcement_vol(mut self, announcement_vol_name: &str) -> Self {
        self.announcement_vol = Variable::new(announcement_vol_name);
        self
    }

    pub fn add_int_speaker_vol(mut self, int_speaker_vol_name: &str) -> Self {
        self.int_speaker_vol = Variable::new(int_speaker_vol_name);
        self
    }

    pub fn add_ext_speaker_vol(mut self, ext_speaker_vol_name: &str) -> Self {
        self.ext_speaker_vol = Variable::new(ext_speaker_vol_name);
        self
    }

    pub fn add_int_speaker_sounds(
        mut self,
        int_speaker_start_name: Option<&str>,
        int_speaker_loop_name: Option<&str>,
        int_speaker_end_name: Option<&str>,
    ) -> Self {
        self.int_speaker_sounds = SoundWithStartAndEnd::new(
            int_speaker_start_name,
            int_speaker_loop_name,
            int_speaker_end_name,
        );
        self
    }

    pub fn add_ext_speaker_sounds(
        mut self,
        ext_speaker_start_name: Option<&str>,
        ext_speaker_loop_name: Option<&str>,
        ext_speaker_end_name: Option<&str>,
    ) -> Self {
        self.ext_speaker_sounds = SoundWithStartAndEnd::new(
            ext_speaker_start_name,
            ext_speaker_loop_name,
            ext_speaker_end_name,
        );
        self
    }

    pub fn add_ext_r_speaker_sounds(
        mut self,
        ext_r_speaker_start_name: Option<&str>,
        ext_r_speaker_loop_name: Option<&str>,
        ext_r_speaker_end_name: Option<&str>,
    ) -> Self {
        self.ext_r_speaker_sounds = SoundWithStartAndEnd::new(
            ext_r_speaker_start_name,
            ext_r_speaker_loop_name,
            ext_r_speaker_end_name,
        );
        self
    }

    pub fn add_ext_l_speaker_sounds(
        mut self,
        ext_l_speaker_start_name: Option<&str>,
        ext_l_speaker_loop_name: Option<&str>,
        ext_l_speaker_end_name: Option<&str>,
    ) -> Self {
        self.ext_l_speaker_sounds = SoundWithStartAndEnd::new(
            ext_l_speaker_start_name,
            ext_l_speaker_loop_name,
            ext_l_speaker_end_name,
        );
        self
    }

    pub fn build(self) -> AnnouncementManager {
        AnnouncementManager {
            power: self.power,

            destination: self.destination,

            int_speaker_sounds: self.int_speaker_sounds,
            ext_speaker_sounds: self.ext_speaker_sounds,
            ext_r_speaker_sounds: self.ext_r_speaker_sounds,
            ext_l_speaker_sounds: self.ext_l_speaker_sounds,

            int_speaker: self.int_speaker,
            ext_speaker: self.ext_speaker,
            ext_r_speaker: self.ext_r_speaker,
            ext_l_speaker: self.ext_l_speaker,

            speaker_target: self.speaker_target,
            annoucement_target: self.annoucement_target,

            announcement_vol: self.announcement_vol,
            int_speaker_vol: self.int_speaker_vol,
            ext_speaker_vol: self.ext_speaker_vol,

            announcement_vol_var: self.announcement_vol_var,
            int_speaker_vol_var: self.int_speaker_vol_var,
            ext_speaker_vol_var: self.ext_speaker_vol_var,
        }
    }
}

pub struct AnnouncementManager {
    pub power: bool,

    destination: AnnouncementDestionation,

    int_speaker_sounds: SoundWithStartAndEnd,
    ext_speaker_sounds: SoundWithStartAndEnd,
    ext_r_speaker_sounds: SoundWithStartAndEnd,
    ext_l_speaker_sounds: SoundWithStartAndEnd,

    int_speaker: Variable<ContentId>,
    ext_speaker: Variable<ContentId>,
    ext_r_speaker: Variable<ContentId>,
    ext_l_speaker: Variable<ContentId>,

    pub speaker_target: SpeakerTarget,
    pub annoucement_target: SpeakerTarget,

    announcement_vol: Variable<f32>,
    int_speaker_vol: Variable<f32>,
    ext_speaker_vol: Variable<f32>,

    pub announcement_vol_var: f32,
    pub int_speaker_vol_var: f32,
    pub ext_speaker_vol_var: f32,
}

impl AnnouncementManager {
    pub fn builder(
        destination: AnnouncementDestionation,
        int_speaker_name: &str,
    ) -> AnnouncementManagerBuilder {
        AnnouncementManagerBuilder {
            power: false,

            destination,

            int_speaker_sounds: SoundWithStartAndEnd::new(None, None, None),
            ext_speaker_sounds: SoundWithStartAndEnd::new(None, None, None),
            ext_r_speaker_sounds: SoundWithStartAndEnd::new(None, None, None),
            ext_l_speaker_sounds: SoundWithStartAndEnd::new(None, None, None),

            int_speaker: Variable::new(int_speaker_name),
            ext_speaker: Variable::new(""),
            ext_r_speaker: Variable::new(""),
            ext_l_speaker: Variable::new(""),

            speaker_target: SpeakerTarget::Int,
            annoucement_target: SpeakerTarget::Int,

            announcement_vol: Variable::new(""),
            int_speaker_vol: Variable::new(""),
            ext_speaker_vol: Variable::new(""),

            announcement_vol_var: 1.0,
            int_speaker_vol_var: 1.0,
            ext_speaker_vol_var: 1.0,
        }
    }

    pub fn tick(&mut self, speaker_target: bool) {
        let int_speaker = speaker_target
            && matches!(
                self.speaker_target,
                SpeakerTarget::Int | SpeakerTarget::IntExt
            );
        let ext_speaker = speaker_target
            && matches!(
                self.speaker_target,
                SpeakerTarget::Int
                    | SpeakerTarget::IntExt
                    | SpeakerTarget::Extr
                    | SpeakerTarget::Extl
            );

        self.announcement_vol.set(self.announcement_vol_var);
        self.ext_speaker_vol.set(self.ext_speaker_vol_var);
        self.int_speaker_vol.set(self.int_speaker_vol_var);

        self.int_speaker_sounds.tick(int_speaker);
        self.ext_speaker_sounds.tick(ext_speaker);
    }

    pub fn on_message(&mut self, msg: Message) {
        msg.handle::<Announcement>(|m| {
            // Todo verarbeitung

            Ok(())
        })
        .expect("Announcement: message handle failed");

        msg.handle::<AnnouncementDriver>(|m| {
            // Todo verarbeitung

            Ok(())
        })
        .expect("AnnouncementDriver: message handle failed");
    }
}
