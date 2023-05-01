use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::Element;

use crate::Scene;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = WebGLRenderTarget)]
    pub type WebGL3DRenderTarget;

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGL3DRenderTarget;

    #[wasm_bindgen(constructor)]
    pub fn new_with(width: i32, height: i32, depth: i32) -> WebGL3DRenderTarget;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = WebGLRenderTarget)]
    pub type WebGLMultipleRenderTargets;

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGLMultipleRenderTargets;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        width: i32,
        height: i32,
        count: i32,
        options: &JsValue,
    ) -> WebGLMultipleRenderTargets;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = WebGLRenderTarget)]
    pub type WebGLCubeRenderTarget;

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGLCubeRenderTarget;

    #[wasm_bindgen(constructor)]
    pub fn new_with(size: i32, options: &JsValue) -> WebGLCubeRenderTarget;

    #[wasm_bindgen(method, js_name = "fromEquirectangularTexture")]
    pub fn from_equirectangular_texture(
        this: &WebGLCubeRenderTarget,
        renderer: &JsValue,
        texture: &JsValue,
    );

    #[wasm_bindgen(method)]
    pub fn clear(
        this: &WebGLCubeRenderTarget,
        renderer: &JsValue,
        color: &JsValue,
        depth: &JsValue,
        stencil: &JsValue,
    );

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = WebGLRenderTarget)]
    pub type WebGLArrayRenderTarget;

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGLArrayRenderTarget;

    #[wasm_bindgen(constructor)]
    pub fn new_with(width: i32, height: i32, depth: i32) -> WebGLArrayRenderTarget;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type WebGLRenderer;

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGLRenderer;

    #[wasm_bindgen(constructor)]
    pub fn new_with(parameters: &JsValue) -> WebGLRenderer;

    #[wasm_bindgen(method, js_name = "setSize")]
    pub fn set_size(this: &WebGLRenderer, width: &JsValue, height: &JsValue, updateStyle: bool);

    #[wasm_bindgen(method, getter)]
    pub fn domElement(this: &WebGLRenderer) -> Element;

    #[wasm_bindgen(method)]
    pub fn render(this: &WebGLRenderer, scene: &Scene, camera: &JsValue);

    #[wasm_bindgen(method, getter, js_name = "physicallyCorrectLights")]
    pub fn physically_correct_lights(this: &WebGLRenderTarget) -> bool;

    #[wasm_bindgen(method, setter, js_name = "physicallyCorrectLights")]
    pub fn set_physically_correct_lights(this: &WebGLRenderTarget, value: bool) -> bool;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type WebGLRenderTarget;

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGLRenderTarget;

    #[wasm_bindgen(constructor)]
    pub fn new_with(width: i32, height: i32, options: &JsValue) -> WebGLRenderTarget;

    #[wasm_bindgen(method, js_name = "setSize")]
    pub fn set_size(this: &WebGLRenderTarget, width: &JsValue, height: &JsValue, depth: i32);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = WebGLRenderer)]
    pub type WebGL1Renderer;

}
