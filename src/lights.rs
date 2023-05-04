use js_sys::{Number, Object};
use wasm_bindgen::prelude::*;

use crate::{core::Object3D, math::SphericalHarmonics3, Color, Vector3};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Light;

    #[wasm_bindgen(constructor)]
    pub fn new(color: i32, intensity: f32) -> Light;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Light, meta: &Object) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &Light) -> Color;

    #[wasm_bindgen(method, setter)]
    pub fn set_color(this: &Light, value: &Color);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type Spotlight;

    #[wasm_bindgen(constructor)]
    pub fn new(
        color: i32,
        intensity: f32,
        distance: f32,
        angle: f32,
        penumbra: f32,
        decay: f32,
    ) -> Spotlight;

    #[wasm_bindgen(method, getter)]
    pub fn power(this: &Spotlight) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_power(this: &Spotlight, power: f32);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type PointLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: i32, intensity: f32, distance: Number, decay: f32) -> PointLight;

    #[wasm_bindgen(method, getter)]
    pub fn power(this: &PointLight) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_power(this: &PointLight, power: f32);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type RectAreaLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: i32, intensity: f32, width: f32, height: f32) -> RectAreaLight;

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
    pub fn new(skyColor: i32, groundColor: i32, intensity: f32) -> HemisphereLight;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &HemisphereLight) -> Vector3;

    #[wasm_bindgen(method, setter)]
    pub fn set_position(this: &HemisphereLight, value: &Vector3);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type LightProbe;

    #[wasm_bindgen(constructor)]
    pub fn new(sh: &SphericalHarmonics3, intensity: f32) -> LightProbe;

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
    pub fn new(skyColor: &Color, groundColor: &Color, intensity: f32) -> HemisphereLightProbe;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type DirectionalLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: i32, intensity: f32) -> DirectionalLight;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Light)]
    pub type AmbientLight;

    #[wasm_bindgen(constructor)]
    pub fn new(color: i32, intensity: f32) -> AmbientLight;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = LightProbe)]
    pub type AmbientLightProbe;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &Color, intensity: f32) -> AmbientLightProbe;

}
