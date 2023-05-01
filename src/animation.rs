use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::math::interpolants::{CubicInterpolant, DiscreteInterpolant, LinearInterpolant};

pub mod tracks;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type KeyframeTrack;

    #[wasm_bindgen(constructor)]
    pub fn new(
        name: &JsValue,
        times: &JsValue,
        values: &JsValue,
        interpolation: &JsValue,
    ) -> KeyframeTrack;

    #[wasm_bindgen(method, js_name = "InterpolantFactoryMethodDiscrete")]
    pub fn interpolant_factory_method_discrete(
        this: &KeyframeTrack,
        result: &JsValue,
    ) -> DiscreteInterpolant;

    #[wasm_bindgen(method, js_name = "InterpolantFactoryMethodLinear")]
    pub fn interpolant_factory_method_linear(
        this: &KeyframeTrack,
        result: &JsValue,
    ) -> LinearInterpolant;

    #[wasm_bindgen(method, js_name = "InterpolantFactoryMethodSmooth")]
    pub fn interpolant_factory_method_smooth(
        this: &KeyframeTrack,
        result: &JsValue,
    ) -> CubicInterpolant;

    #[wasm_bindgen(method, js_name = "setInterpolation")]
    pub fn set_interpolation(this: &KeyframeTrack, interpolation: i32) -> KeyframeTrack;

    #[wasm_bindgen(method, js_name = "setInterpolation")]
    pub fn get_interpolation(this: &KeyframeTrack) -> Option<i32>;

    #[wasm_bindgen(method, js_name = "getValueSize")]
    pub fn get_value_size(this: &KeyframeTrack) -> f64;

    #[wasm_bindgen(method)]
    pub fn shift(this: &KeyframeTrack, time_offset: f64) -> KeyframeTrack;

    #[wasm_bindgen(method)]
    pub fn scale(this: &KeyframeTrack, time_scale: f64) -> KeyframeTrack;

    #[wasm_bindgen(method)]
    pub fn trim(this: &KeyframeTrack, start_time: &JsValue, end_time: &JsValue) -> KeyframeTrack;

    #[wasm_bindgen(method)]
    pub fn validate(this: &KeyframeTrack) -> bool;

    #[wasm_bindgen(method)]
    pub fn optimize(this: &KeyframeTrack) -> KeyframeTrack;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type PropertyMixer;

    #[wasm_bindgen(constructor)]
    pub fn new(binding: &JsValue, typeName: &JsValue, valueSize: &JsValue) -> PropertyMixer;

    #[wasm_bindgen(method)]
    pub fn accumulate(this: &PropertyMixer, accuIndex: &JsValue, weight: &JsValue);

    #[wasm_bindgen(method, js_name = "accumulateAdditive")]
    pub fn accumulate_additive(this: &PropertyMixer, weight: &JsValue);

    #[wasm_bindgen(method)]
    pub fn apply(this: &PropertyMixer, accuIndex: &JsValue);

    #[wasm_bindgen(method, js_name = "saveOriginalState")]
    pub fn save_original_state(this: &PropertyMixer);

    #[wasm_bindgen(method, js_name = "restoreOriginalState")]
    pub fn restore_original_state(this: &PropertyMixer);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Composite;

    #[wasm_bindgen(constructor)]
    pub fn new(target_group: &JsValue, path: &JsValue, optional_parsed_path: &JsValue)
        -> Composite;

    #[wasm_bindgen(method, js_name = "getValue")]
    pub fn get_value(this: &Composite, array: &JsValue, offset: &JsValue);

    #[wasm_bindgen(method, js_name = "setValue")]
    pub fn set_value(this: &Composite, array: &JsValue, offset: &JsValue);

    #[wasm_bindgen(method)]
    pub fn bind(this: &Composite);

    #[wasm_bindgen(method)]
    pub fn unbind(this: &Composite);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type PropertyBinding;

    #[wasm_bindgen(constructor)]
    pub fn new(root_node: &JsValue, path: &JsValue, parsed_path: &JsValue) -> PropertyBinding;

    #[wasm_bindgen(method)]
    pub fn bind(this: &PropertyBinding);

    #[wasm_bindgen(method)]
    pub fn unbind(this: &PropertyBinding);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type AnimationObjectGroup;

    #[wasm_bindgen(constructor)]
    pub fn new() -> AnimationObjectGroup;

    #[wasm_bindgen(method)]
    pub fn add(this: &AnimationObjectGroup);

    #[wasm_bindgen(method)]
    pub fn remove(this: &AnimationObjectGroup);

    #[wasm_bindgen(method)]
    pub fn uncache(this: &AnimationObjectGroup);

    #[wasm_bindgen(method, js_name = "subscribe_")]
    pub fn subscribe(this: &AnimationObjectGroup);

    #[wasm_bindgen(method, js_name = "unsubscribe_")]
    pub fn unsubscribe(this: &AnimationObjectGroup);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type AnimationMixer;

    #[wasm_bindgen(constructor)]
    pub fn new(root: &JsValue) -> AnimationMixer;

    #[wasm_bindgen(method, js_name = "clipAction")]
    pub fn clip_action(
        this: &AnimationMixer,
        clip: &JsValue,
        optional_root: &JsValue,
        blend_mode: &JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = "existingAction")]
    pub fn existing_action(
        this: &AnimationMixer,
        clip: &JsValue,
        optional_root: &JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = "stopAllAction")]
    pub fn stop_all_action(this: &AnimationMixer) -> AnimationMixer;

    #[wasm_bindgen(method)]
    pub fn update(this: &AnimationMixer, delta_time: &JsValue) -> AnimationMixer;

    #[wasm_bindgen(method, js_name = "setTime")]
    pub fn set_time(this: &AnimationMixer, time_in_secs: &JsValue) -> AnimationMixer;

    #[wasm_bindgen(method, js_name = "getRoot")]
    pub fn get_root(this: &AnimationMixer, time_in_secs: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "uncacheClip")]
    pub fn uncache_clip(this: &AnimationMixer, clip: &JsValue);

    #[wasm_bindgen(method, js_name = "uncacheRoot")]
    pub fn uncache_root(this: &AnimationMixer, clip: &JsValue);

    #[wasm_bindgen(method, js_name = "uncacheAction")]
    pub fn uncache_action(this: &AnimationMixer, clip: &JsValue);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type AnimationClip;

    #[wasm_bindgen(constructor)]
    pub fn new(name: &JsValue) -> AnimationClip;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        name: &JsValue,
        duration: i32,
        tracks: &JsValue,
        blend_mode: i32,
    ) -> AnimationClip;

    #[wasm_bindgen(method, js_name = "resetDuration")]
    pub fn reset_duration(this: &AnimationClip) -> AnimationClip;

    #[wasm_bindgen(method)]
    pub fn trim(this: &AnimationClip) -> AnimationClip;

    #[wasm_bindgen(method)]
    pub fn validate(this: &AnimationClip) -> bool;

    #[wasm_bindgen(method)]
    pub fn optimize(this: &AnimationClip) -> AnimationClip;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &AnimationClip);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type AnimationAction;

    #[wasm_bindgen(constructor)]
    pub fn new(mixer: &JsValue, clip: &JsValue) -> AnimationAction;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        mixer: &JsValue,
        clip: i32,
        local_root: &JsValue,
        blend_mode: &JsValue,
    ) -> AnimationAction;

    #[wasm_bindgen(method)]
    pub fn play(this: &AnimationAction) -> AnimationAction;

    #[wasm_bindgen(method)]
    pub fn stop(this: &AnimationAction) -> AnimationAction;

    #[wasm_bindgen(method)]
    pub fn reset(this: &AnimationAction) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "isRunning")]
    pub fn is_running(this: &AnimationAction) -> bool;

    #[wasm_bindgen(method, js_name = "isScheduled")]
    pub fn is_scheduled(this: &AnimationAction) -> JsValue;

    #[wasm_bindgen(method, js_name = "startAt")]
    pub fn start_at(this: &AnimationAction, time: &JsValue) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "setLoop")]
    pub fn start_loop(
        this: &AnimationAction,
        mode: &JsValue,
        repetitions: &JsValue,
    ) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "setEffectiveWeight")]
    pub fn set_effective_weight(this: &AnimationAction, weight: &JsValue) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "getEffectiveWeight")]
    pub fn get_effective_weight(this: &AnimationAction) -> f64;

    #[wasm_bindgen(method, js_name = "fadeIn")]
    pub fn fade_in(this: &AnimationAction, duration: &JsValue) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "fadeOut")]
    pub fn fade_out(this: &AnimationAction, duration: &JsValue) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "crossFadeFrom")]
    pub fn cross_fade_from(
        this: &AnimationAction,
        fadeOutAction: &JsValue,
        duration: &JsValue,
        warp: &JsValue,
    ) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "crossFadeTo")]
    pub fn cross_fade_to(
        this: &AnimationAction,
        fadeOutAction: &JsValue,
        duration: &JsValue,
        warp: &JsValue,
    ) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "stopFading")]
    pub fn stop_fading(this: &AnimationAction);

    #[wasm_bindgen(method, js_name = "setEffectiveTimeScale")]
    pub fn set_effective_timeScale(this: &AnimationAction, timeScale: &JsValue) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "getEffectiveTimeScale")]
    pub fn get_effective_timeScale(this: &AnimationAction) -> f64;

    #[wasm_bindgen(method, js_name = "setDuration")]
    pub fn set_duration(this: &AnimationAction, duration: &JsValue) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "syncWith")]
    pub fn sync_with(this: &AnimationAction, action: &JsValue) -> AnimationAction;

    #[wasm_bindgen(method)]
    pub fn halt(this: &AnimationAction, duration: &JsValue) -> AnimationAction;

    #[wasm_bindgen(method)]
    pub fn warp(
        this: &AnimationAction,
        start_time_scale: &JsValue,
        end_time_scale: &JsValue,
        duration: &JsValue,
    ) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "stopWarping")]
    pub fn stop_warping(this: &AnimationAction) -> AnimationAction;

    #[wasm_bindgen(method, js_name = "getMixer")]
    pub fn get_mixer(this: &AnimationAction) -> JsValue;

    #[wasm_bindgen(method, js_name = "getClip")]
    pub fn get_clip(this: &AnimationAction) -> JsValue;

    #[wasm_bindgen(method, js_name = "getRoot")]
    pub fn get_root(this: &AnimationAction) -> JsValue;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &AnimationAction);

}
