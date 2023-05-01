use js_sys::{Number, Uint8Array};
use wasm_bindgen::{prelude::*, JsValue};

use crate::{core::Object3D, AudioLoader};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type AudioListener;

    #[wasm_bindgen(constructor)]
    pub fn new() -> AudioListener;

    #[wasm_bindgen(method, js_name = "getInput")]
    pub fn get_input(this: &AudioListener) -> JsValue;

    #[wasm_bindgen(method, js_name = "removeFilter")]
    pub fn remove_filter(this: &AudioListener);

    #[wasm_bindgen(method, js_name = "getFilter")]
    pub fn get_filter(this: &AudioLoader) -> JsValue;

    #[wasm_bindgen(method, js_name = "setFilter")]
    pub fn set_filter(this: &AudioLoader, value: &JsValue) -> AudioListener;

    #[wasm_bindgen(method, js_name = "getMasterVolume")]
    pub fn get_master_volume(this: &AudioLoader) -> JsValue;

    #[wasm_bindgen(method, js_name = "setMasterVolume")]
    pub fn set_master_volume(this: &AudioLoader, value: &JsValue) -> AudioListener;

    #[wasm_bindgen(method, js_name = "updateMatrixWorld")]
    pub fn update_matrix_world(this: &AudioLoader, force: &JsValue);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Audio;

    #[wasm_bindgen(constructor)]
    pub fn new(listener: &JsValue) -> Audio;

    #[wasm_bindgen(method, js_name = "getOutput")]
    pub fn get_output(this: &Audio) -> JsValue;

    #[wasm_bindgen(method, js_name = "setNodeSource")]
    pub fn set_node_source(this: &Audio, audio_node: &JsValue) -> Audio;

    #[wasm_bindgen(method, js_name = "setMediaElementSource")]
    pub fn set_media_element_source(this: &Audio, media_element: &JsValue) -> Audio;

    #[wasm_bindgen(method, js_name = "setMediaStreamSource")]
    pub fn set_media_stream_source(this: &Audio, media_stream: &JsValue) -> Audio;

    #[wasm_bindgen(method, js_name = "setBuffer")]
    pub fn set_buffer(this: &Audio, audio_buffer: &JsValue) -> Audio;

    #[wasm_bindgen(method)]
    pub fn play(this: &Audio, delay: i32) -> Option<Audio>;

    #[wasm_bindgen(method)]
    pub fn pause(this: &Audio) -> Option<Audio>;

    #[wasm_bindgen(method)]
    pub fn stop(this: &Audio) -> Option<Audio>;

    #[wasm_bindgen(method)]
    pub fn connect(this: &Audio) -> Audio;

    #[wasm_bindgen(method)]
    pub fn disconnect(this: &Audio) -> Audio;

    #[wasm_bindgen(method, js_name = "getFilters")]
    pub fn get_filters(this: &Audio) -> JsValue;

    #[wasm_bindgen(method, js_name = "setFilters")]
    pub fn set_filters(this: &Audio, value: &JsValue) -> Audio;

    #[wasm_bindgen(method, js_name = "setDetune")]
    pub fn set_detune(this: &Audio, value: &JsValue) -> Option<Audio>;

    #[wasm_bindgen(method, js_name = "getDetune")]
    pub fn get_detune(this: &Audio) -> f64;

    #[wasm_bindgen(method, js_name = "getFilter")]
    pub fn get_filter(this: &Audio) -> JsValue;

    #[wasm_bindgen(method, js_name = "setFilter")]
    pub fn set_filter(this: &Audio, filter: &JsValue) -> Audio;

    #[wasm_bindgen(method, js_name = "setPlaybackRate")]
    pub fn set_playback_rate(this: &Audio, value: &JsValue) -> Option<Audio>;

    #[wasm_bindgen(method, js_name = "getPlaybackRate")]
    pub fn get_playback_rate(this: &Audio) -> f64;

    #[wasm_bindgen(method, js_name = "onEnded")]
    pub fn on_ended(this: &Audio);

    #[wasm_bindgen(method, js_name = "getLoop")]
    pub fn get_loop(this: &Audio) -> bool;

    #[wasm_bindgen(method, js_name = "setLoop")]
    pub fn set_loop(this: &Audio, value: bool) -> Option<Audio>;

    #[wasm_bindgen(method, js_name = "setLoopStart")]
    pub fn set_loop_start(this: &Audio, value: f64) -> Audio;

    #[wasm_bindgen(method, js_name = "setLoopEnd")]
    pub fn set_loop_end(this: &Audio, value: f64) -> Audio;

    #[wasm_bindgen(method, js_name = "getVolume")]
    pub fn get_volume(this: &Audio) -> JsValue;

    #[wasm_bindgen(method, js_name = "setVolume")]
    pub fn set_volume(this: &Audio, value: &JsValue) -> Audio;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Audio)]
    pub type PositionalAudio;

    #[wasm_bindgen(constructor)]
    pub fn new(listener: &JsValue) -> PositionalAudio;

    #[wasm_bindgen(method)]
    pub fn disconnect(this: &PositionalAudio);

    #[wasm_bindgen(method, js_name = "getOutput")]
    pub fn get_output(this: &PositionalAudio) -> JsValue;

    #[wasm_bindgen(method, js_name = "getRefDistance")]
    pub fn get_ref_distance(this: &PositionalAudio) -> JsValue;

    #[wasm_bindgen(method, js_name = "setRefDistance")]
    pub fn set_ref_distance(this: &PositionalAudio, value: &JsValue) -> PositionalAudio;

    #[wasm_bindgen(method, js_name = "getRolloffFactor")]
    pub fn get_rolloff_factor(this: &PositionalAudio) -> JsValue;

    #[wasm_bindgen(method, js_name = "setRolloffFactor")]
    pub fn set_rolloff_factor(this: &PositionalAudio, value: &JsValue) -> PositionalAudio;

    #[wasm_bindgen(method, js_name = "getDistanceModel")]
    pub fn get_distance_model(this: &PositionalAudio) -> JsValue;

    #[wasm_bindgen(method, js_name = "setDistanceModel")]
    pub fn setDistanceModel(this: &PositionalAudio, value: &JsValue) -> PositionalAudio;

    #[wasm_bindgen(method, js_name = "getMaxDistance")]
    pub fn get_max_distance(this: &PositionalAudio) -> JsValue;

    #[wasm_bindgen(method, js_name = "setMaxDistance")]
    pub fn set_max_distance(this: &PositionalAudio, value: &JsValue) -> PositionalAudio;

    #[wasm_bindgen(method, js_name = "setDirectionalCone")]
    pub fn set_directional_cone(
        this: &PositionalAudio,
        coneInnerAngle: &JsValue,
        coneOuterAngle: &JsValue,
        coneOuterGain: &JsValue,
    ) -> PositionalAudio;

    #[wasm_bindgen(method, js_name = "updateMatrixWorld")]
    pub fn update_matrix_world(this: &PositionalAudio, force: &JsValue);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type AudioAnalyser;

    #[wasm_bindgen(constructor)]
    pub fn new(audio: &JsValue, fftSize: Number) -> AudioAnalyser;

    #[wasm_bindgen(method, js_name = "getFrequencyData")]
    pub fn get_frequency_data(this: &AudioAnalyser) -> Uint8Array;

    #[wasm_bindgen(method, js_name = "getAverageFrequency")]
    pub fn get_average_frequency(this: &AudioAnalyser) -> f64;
}
