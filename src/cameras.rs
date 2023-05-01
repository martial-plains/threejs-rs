use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::core::Object3D;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Camera;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Camera;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type StereoCamera;

    #[wasm_bindgen(constructor)]
    pub fn new() -> StereoCamera;

    #[wasm_bindgen(method)]
    pub fn update(this: &StereoCamera, camera: &Camera) -> StereoCamera;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Camera)]
    pub type PerspectiveCamera;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PerspectiveCamera;

    #[wasm_bindgen(constructor)]
    pub fn new_with(fov: f64, aspect: f64, near: f64, far: f64) -> PerspectiveCamera;

    #[wasm_bindgen(method, js_name = "setFocalLength")]
    pub fn set_focal_length(this: &PerspectiveCamera, focalLength: i64);

    #[wasm_bindgen(method, js_name = "getFocalLength")]
    pub fn get_focal_length(this: &PerspectiveCamera) -> i64;

    #[wasm_bindgen(method, js_name = "getEffectiveFOV")]
    pub fn get_effective_fov(this: &PerspectiveCamera) -> i64;

    #[wasm_bindgen(method, js_name = "getFilmWidth")]
    pub fn get_film_width(this: &PerspectiveCamera) -> i64;

    #[wasm_bindgen(method, js_name = "getFilmHeight")]
    pub fn get_film_height(this: &PerspectiveCamera) -> i64;

    #[wasm_bindgen(method, js_name = "setViewOffset")]
    pub fn set_view_offset(
        this: &PerspectiveCamera,
        full_width: &JsValue,
        full_height: &JsValue,
        x: &JsValue,
        y: &JsValue,
        width: &JsValue,
        height: &JsValue,
    );

    #[wasm_bindgen(method, js_name = "clearViewOffset")]
    pub fn clear_view_offset(this: &PerspectiveCamera);

    #[wasm_bindgen(method, js_name = "updateProjectionMatrix")]
    pub fn update_projection_matrix(this: &PerspectiveCamera);

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &PerspectiveCamera, meta: &JsValue) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Camera)]
    pub type OrthographicCamera;

    #[wasm_bindgen(constructor)]
    pub fn new() -> OrthographicCamera;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
        near: f64,
        far: f64,
    ) -> OrthographicCamera;

    #[wasm_bindgen(method, js_name = "setViewOffset")]
    pub fn set_view_offset(
        this: &OrthographicCamera,
        full_width: &JsValue,
        full_height: &JsValue,
        x: &JsValue,
        y: &JsValue,
        width: &JsValue,
        height: &JsValue,
    );

    #[wasm_bindgen(method, js_name = "clearViewOffset")]
    pub fn clear_view_offset(this: &OrthographicCamera);

    #[wasm_bindgen(method, js_name = "updateProjectionMatrix")]
    pub fn update_projection_matrix(this: &OrthographicCamera);

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &OrthographicCamera, meta: &JsValue) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type CubeCamera;

    #[wasm_bindgen(constructor)]
    pub fn new(near: &JsValue, far: &JsValue, render_target: &JsValue) -> CubeCamera;

    #[wasm_bindgen(method)]
    pub fn update(this: &CubeCamera, renderer: &JsValue, scene: &JsValue);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type ArrayCamera;

    #[wasm_bindgen(constructor)]
    pub fn from_array(array: &JsValue) -> ArrayCamera;

}
