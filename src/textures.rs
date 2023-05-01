use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Texture;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Texture;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        image: &JsValue,
        mapping: i32,
        wrap_s: i32,
        wrap_t: i32,
        mag_filter: i32,
        min_filter: i32,
        format: i32,
        r#type: i32,
        anisotropy: i32,
        encoding: i32,
    ) -> Texture;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Texture) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn image(this: &Texture) -> JsValue;

    #[wasm_bindgen(method, setter)]
    pub fn set_image(this: &Texture, value: &JsValue);

    #[wasm_bindgen(method, js_name = "updateMatrix")]
    pub fn update_matrix(this: &Texture);

    #[wasm_bindgen(method, js_name = "transformUv")]
    pub fn transform_uv(this: &Texture, uv: &JsValue) -> JsValue;

    #[wasm_bindgen(method, setter, js_name = "needsUpdate")]
    pub fn needs_update(this: &Texture, value: &JsValue);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type VideoTexture;

    #[wasm_bindgen(constructor)]
    pub fn new(
        video: &JsValue,
        mapping: &JsValue,
        wrap_s: &JsValue,
        wrap_t: &JsValue,
        mag_filter: &JsValue,
        min_filter: &JsValue,
        format: &JsValue,
        r#type: &JsValue,
        anisotropy: &JsValue,
    ) -> VideoTexture;

    #[wasm_bindgen(method)]
    pub fn update(this: &VideoTexture);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type FramebufferTexture;

    #[wasm_bindgen(constructor)]
    pub fn new(width: &JsValue, height: &JsValue, format: &JsValue) -> FramebufferTexture;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type Source;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Source;

    #[wasm_bindgen(constructor)]
    pub fn new_with_data(data: &JsValue) -> Source;

    #[wasm_bindgen(method, setter, js_name = "needsUpdate")]
    pub fn set_needs_update(this: &Source, value: &JsValue);

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Source, meta: &JsValue);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type DataTexture;

    #[wasm_bindgen(constructor)]
    pub fn new(
        data: &JsValue,
        width: i32,
        height: i32,
        format: &JsValue,
        r#type: &JsValue,
        mapping: &JsValue,
        wrap_s: &JsValue,
        wrap_t: &JsValue,
        mag_filter: i32,
        min_filter: i32,
        anisotropy: &JsValue,
        encoding: &JsValue,
    ) -> DataTexture;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type DataArrayTexture;

    #[wasm_bindgen(constructor)]
    pub fn new() -> DataArrayTexture;

    #[wasm_bindgen(constructor)]
    pub fn new_with(data: &JsValue, width: i32, height: i32, depth: i32) -> DataArrayTexture;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type Data3DTexture;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Data3DTexture;

    #[wasm_bindgen(constructor)]
    pub fn new_with(data: &JsValue, width: i32, height: i32, depth: i32) -> Data3DTexture;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type CompressedTexture;

    #[wasm_bindgen(constructor)]
    pub fn new(
        mipmaps: &JsValue,
        width: &JsValue,
        height: &JsValue,
        format: &JsValue,
        r#type: &JsValue,
        mapping: &JsValue,
        wrap_s: &JsValue,
        wrap_t: &JsValue,
        mag_filter: i32,
        min_filter: i32,
        anisotropy: &JsValue,
        encoding: &JsValue,
    ) -> CompressedTexture;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type CompressedArrayTexture;

    #[wasm_bindgen(constructor)]
    pub fn new(
        mipmaps: &JsValue,
        width: &JsValue,
        height: &JsValue,
        depth: &JsValue,
        format: &JsValue,
        r#type: &JsValue,
    ) -> CompressedArrayTexture;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type CubeTexture;

    #[wasm_bindgen(constructor)]
    pub fn new() -> CubeTexture;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        images: &JsValue,
        mapping: &JsValue,
        wrap_s: &JsValue,
        wrap_t: &JsValue,
        mag_filter: &JsValue,
        min_filter: &JsValue,
        format: &JsValue,
        r#type: &JsValue,
        anisotropy: &JsValue,
        encoding: &JsValue,
    ) -> CubeTexture;

    #[wasm_bindgen(method, getter)]
    pub fn image(this: &CubeTexture) -> JsValue;

    #[wasm_bindgen(method, setter)]
    pub fn set_image(this: &CubeTexture, value: &JsValue);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type CanvasTexture;

    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: &JsValue,
        mapping: &JsValue,
        wrap_s: &JsValue,
        wrap_t: &JsValue,
        mag_filter: i32,
        min_filter: i32,
        format: &JsValue,
        r#type: &JsValue,
        anisotropy: &JsValue,
    ) -> CanvasTexture;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Texture)]
    pub type DepthTexture;

    #[wasm_bindgen(constructor)]
    pub fn new(
        width: &JsValue,
        height: &JsValue,
        r#type: &JsValue,
        mapping: &JsValue,
        wrap_s: &JsValue,
        wrap_t: &JsValue,
        mag_filter: i32,
        min_filter: i32,
        anisotropy: &JsValue,
        format: &JsValue,
    ) -> DepthTexture;
}
