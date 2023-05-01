use wasm_bindgen::{prelude::*, JsValue};

use crate::math::Interpolant;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Interpolant)]
    pub type DiscreteInterpolant;

    #[wasm_bindgen(constructor)]
    pub fn new(
        parameterPositions: &JsValue,
        sampleValues: &JsValue,
        sampleSize: &JsValue,
        resultBuffer: &JsValue,
    ) -> DiscreteInterpolant;

    #[wasm_bindgen(method, js_name = "interpolate_")]
    pub fn interpolate(this: &DiscreteInterpolant, i1: &JsValue) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Interpolant)]
    pub type LinearInterpolant;

    #[wasm_bindgen(constructor)]
    pub fn new(
        parameterPositions: &JsValue,
        sampleValues: &JsValue,
        sampleSize: &JsValue,
        resultBuffer: &JsValue,
    ) -> LinearInterpolant;

    #[wasm_bindgen(method, js_name = "interpolate_")]
    pub fn interpolate(
        this: &LinearInterpolant,
        i1: &JsValue,
        t0: &JsValue,
        t: &JsValue,
        t1: &JsValue,
    ) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Interpolant)]
    pub type CubicInterpolant;

    #[wasm_bindgen(constructor)]
    pub fn new(
        parameterPositions: &JsValue,
        sampleValues: &JsValue,
        sampleSize: &JsValue,
        resultBuffer: &JsValue,
    ) -> CubicInterpolant;

    #[wasm_bindgen(method, js_name = "intervalChanged_")]
    pub fn interval_changed(this: &CubicInterpolant, i1: &JsValue, t0: &JsValue, t1: &JsValue);

    #[wasm_bindgen(method, js_name = "interpolate_")]
    pub fn interpolate(
        this: &CubicInterpolant,
        i1: &JsValue,
        t0: &JsValue,
        t: &JsValue,
        t1: &JsValue,
    ) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Interpolant)]
    pub type QuaternionLinearInterpolant;

    #[wasm_bindgen(constructor)]
    pub fn new(
        parameterPositions: &JsValue,
        sampleValues: &JsValue,
        sampleSize: &JsValue,
        resultBuffer: &JsValue,
    ) -> QuaternionLinearInterpolant;

    #[wasm_bindgen(method, js_name = "interpolate_")]
    pub fn interpolate(
        this: &QuaternionLinearInterpolant,
        i1: &JsValue,
        t0: &JsValue,
        t: &JsValue,
        t1: &JsValue,
    ) -> JsValue;
}
