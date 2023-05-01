use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::QuaternionLinearInterpolant;

use super::KeyframeTrack;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = KeyframeTrack)]
    pub type VectorKeyframeTrack;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = KeyframeTrack)]
    pub type StringKeyframeTrack;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = KeyframeTrack)]
    pub type QuaternionKeyframeTrack;

    #[wasm_bindgen(method, js_name = "InterpolantFactoryMethodLinear")]
    pub fn InterpolantFactoryMethodLinear(
        this: &QuaternionKeyframeTrack,
        result: &JsValue,
    ) -> QuaternionLinearInterpolant;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = KeyframeTrack)]
    pub type NumberKeyframeTrack;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = KeyframeTrack)]
    pub type ColorKeyframeTrack;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = KeyframeTrack)]
    pub type BooleanKeyframeTrack;
}
