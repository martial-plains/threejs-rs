use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::core::Object3D;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Fog;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &JsValue) -> Fog;

    #[wasm_bindgen(constructor)]
    pub fn new_with_near_and_far(color: &JsValue, near: i32, far: i32) -> Fog;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Fog) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type FogExp2;

    #[wasm_bindgen(constructor)]
    pub fn new(color: &JsValue) -> FogExp2;

    #[wasm_bindgen(constructor)]
    pub fn new_with_density(color: &JsValue, density: f64) -> FogExp2;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &FogExp2) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Scene;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Scene, meta: JsValue) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn autoUpdate(this: &Scene) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_autoUpdate(this: &Scene, value: bool);

}
