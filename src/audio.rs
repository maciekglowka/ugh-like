use fyrox_sound::{
    buffer::{DataSource, SoundBufferResource, SoundBufferResourceExtension},
    context::SoundContext,
    engine::SoundEngine,
    pool::Handle,
    source::{SoundSource, SoundSourceBuilder, Status}
};
use std::collections::HashMap;


#[derive(Default)]
pub struct AudioContext {
    inner: Option<SoundContext>,
    sounds: HashMap<&'static str, Handle<SoundSource>>
}
impl AudioContext {
    pub fn play(&mut self, sound: &str) {
        let Some(handle) = self.sounds.get(sound) else { return };
        if let Some(context) = self.inner.as_mut() {
            let mut state = context.state();
            let source = state.source_mut(*handle);
            let _ = source.stop();
            source.play();
        }
    }
}

pub fn get_audio_context() -> AudioContext {
    let Ok(engine) = SoundEngine::new() else {
        return AudioContext {
            inner: None,
            sounds: HashMap::new()
        }
    };
    let context = SoundContext::new();
    engine.state().add_context(context.clone());

    let mut data = HashMap::new();
    data.insert("hit", include_bytes!("../assets/hit.wav").to_vec());
    data.insert("load", include_bytes!("../assets/load.wav").to_vec());
    data.insert("unload", include_bytes!("../assets/unload.wav").to_vec());
    data.insert("resign", include_bytes!("../assets/resign.wav").to_vec());

    let mut sounds = HashMap::new();

    for (k, v) in data.iter() {
        let buffer = SoundBufferResource::new_generic(
                DataSource::from_memory(v.to_vec())
            )
            .expect(&format!("Can't build audio buffer for {}!", k));
        let source = SoundSourceBuilder::new()
            .with_buffer(buffer)
            .build()
            .expect(&format!("Can't build audio source for {}!", k));
        let handle = context.state().add_source(source);
        sounds.insert(*k, handle);
    }

    AudioContext { inner: Some(context), sounds }
}