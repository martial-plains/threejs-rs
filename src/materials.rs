use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Material;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Material;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Material)]
    pub type MeshBasicMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new_with(parameters: JsValue) -> MeshBasicMaterial;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Material)]
    pub type LineBasicMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new_with(parameters: JsValue) -> LineBasicMaterial;

}
