use wasm_bindgen::prelude::*;

use crate::{core::Object3D, math::SphericalHarmonics3};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Light;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &JsValue, intensity: i32) -> Light;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Light, meta: &JsValue) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type Spotlight;

    #[wasm_bindgen(constructor)]
    pub fn new(
        color: &JsValue,
        intensity: i32,
        distance: i32,
        angle: f64,
        penumbra: i32,
        decay: i32,
    ) -> Spotlight;

    #[wasm_bindgen(method, getter)]
    pub fn power(this: &Spotlight) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_power(this: &Spotlight, power: f64);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type PointLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &JsValue, intensity: i32, distance: i32, decay: i32) -> PointLight;

    #[wasm_bindgen(method, getter)]
    pub fn power(this: &PointLight) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_power(this: &PointLight, power: f64);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type RectAreaLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &JsValue, intensity: i32, width: i32, height: i32) -> RectAreaLight;

    #[wasm_bindgen(method, getter)]
    pub fn power(this: &RectAreaLight) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_power(this: &RectAreaLight, power: f64);

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &RectAreaLight, meta: &JsValue) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type HemisphereLight;

    #[wasm_bindgen(constructor)]
    pub fn new(skyColor: &JsValue, groundColor: &JsValue, intensity: i32) -> HemisphereLight;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type LightProbe;

    #[wasm_bindgen(constructor)]
    pub fn new(sh: &SphericalHarmonics3, intensity: i32) -> LightProbe;

    #[wasm_bindgen(method, js_name = "fromJSON")]
    pub fn from_json(this: &LightProbe, json: &JsValue) -> LightProbe;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &LightProbe, meta: &JsValue) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = LightProbe)]
    pub type HemisphereLightProbe;

    #[wasm_bindgen(constructor)]
    pub fn new(skyColor: &JsValue, groundColor: &JsValue, intensity: i32) -> HemisphereLightProbe;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type DirectionalLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &JsValue, intensity: i32) -> DirectionalLight;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type AmbientLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &JsValue, intensity: i32) -> AmbientLight;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = LightProbe)]
    pub type AmbientLightProbe;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &JsValue, intensity: i32) -> AmbientLightProbe;

}
