use ambisonic::rodio::Source;
use ambisonic::{Ambisonic, AmbisonicBuilder, SoundController};
use math::vec::vec3f;

pub struct Audio {
    scene: Ambisonic,
    sounds: Vec<Sound>,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            scene: AmbisonicBuilder::new().build(),
            sounds: Vec::new(),
        }
    }

    pub fn add_sound(&mut self, source: impl Source<Item = f32> + Send + 'static, position: vec3f) -> SoundId {
        let controller = self.scene.play_at(source, position.to_array());

        let index = self.sounds.len();
        self.sounds.push(Sound { controller, is_paused: false });

        SoundId { index }
    }

    pub fn sound_mut(&mut self, id: SoundId) -> Option<&mut Sound> {
        self.sounds.get_mut(id.index)
    }

    pub fn pause(&mut self) {
        for sound in &mut self.sounds {
            sound.controller.pause();
            sound.is_paused = true;
        }
    }

    pub fn resume(&mut self) {
        for sound in &mut self.sounds {
            sound.controller.resume();
            sound.is_paused = false;
        }
    }
}

pub struct SoundId {
    index: usize,
}

pub struct Sound {
    controller: SoundController,
    is_paused: bool,
}
