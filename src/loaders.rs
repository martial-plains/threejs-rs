use wasm_bindgen::{prelude::*, JsValue};

use crate::{
    textures::{CompressedTexture, CubeTexture, DataTexture, Texture},
    Object3D,
};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Loader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> Loader;

    #[wasm_bindgen(method)]
    pub fn load(this: &Loader);

    #[wasm_bindgen(method, js_name = "loadAsync")]
    pub async fn load_async(this: &Loader, url: &JsValue, on_progress: &JsValue);

    #[wasm_bindgen(method)]
    pub async fn parse(this: &Loader);

    #[wasm_bindgen(method, js_name = "setCrossOrigin")]
    pub async fn set_cross_origin(this: &Loader);

    #[wasm_bindgen(method, js_name = "setWithCredentials")]
    pub async fn set_with_credentials(this: &Loader, value: &JsValue);

    #[wasm_bindgen(method, js_name = "setPath")]
    pub async fn set_path(this: &Loader, path: &JsValue);

    #[wasm_bindgen(method, js_name = "setResourcePath")]
    pub async fn set_resource_path(this: &Loader, resourcePath: &JsValue);

    #[wasm_bindgen(method, js_name = "setRequestHeader")]
    pub async fn set_request_header(this: &Loader, requestHeader: &JsValue);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type AnimationLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> AnimationLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &AnimationLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    );

    #[wasm_bindgen(method)]
    pub fn parse(this: &AnimationLoader, json: &JsValue) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type CompressedTextureLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> CompressedTextureLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &CompressedTextureLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    ) -> CompressedTexture;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type CubeTextureLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> CubeTextureLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &CubeTextureLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    ) -> CubeTexture;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type DataTextureLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> DataTextureLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &DataTextureLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    ) -> DataTexture;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type TextureLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> TextureLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &TextureLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    ) -> Texture;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type ObjectLoader;

    #[wasm_bindgen(constructor)]
    pub fn new() -> ObjectLoader;

    #[wasm_bindgen(constructor)]
    pub fn new_with(manager: &JsValue) -> ObjectLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &ObjectLoader,
        url: &JsValue,
        on_load: &mut dyn FnMut(JsValue),
        on_progress: &mut dyn FnMut(JsValue),
        on_error: &mut dyn FnMut(JsValue),
    );

    #[wasm_bindgen(method, js_name = "loadAsync")]
    pub async fn load_async(this: &ObjectLoader, url: &JsValue, on_progress: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn parse(this: &ObjectLoader, json: &JsValue, on_load: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseAsync")]
    pub async fn parse_async(this: &ObjectLoader, json: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseShapes")]
    pub fn parse_shapes(this: &ObjectLoader, json: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseSkeletons")]
    pub fn parse_skeletons(this: &ObjectLoader, json: &JsValue, object: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseGeometries")]
    pub fn parse_geometries(this: &ObjectLoader, json: &JsValue, object: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseMaterials")]
    pub fn parse_materials(this: &ObjectLoader, json: &JsValue, textures: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseAnimations")]
    pub fn parse_animations(this: &ObjectLoader, json: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseImages")]
    pub fn parse_images(this: &ObjectLoader, json: &JsValue, on_load: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseImagesAsync")]
    pub async fn parse_images_async(this: &ObjectLoader, json: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseTextures")]
    pub fn parse_textures(this: &ObjectLoader, json: &JsValue, images: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "parseObject")]
    pub fn parse_object(
        this: &ObjectLoader,
        data: &JsValue,
        geometries: &JsValue,
        materials: &JsValue,
        textures: &JsValue,
        animations: &JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = "bindSkeletons")]
    pub fn bind_skeletons(this: &ObjectLoader, object: &JsValue, skeleton: &JsValue) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type MaterialLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> MaterialLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &MaterialLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    );

    #[wasm_bindgen(method)]
    pub fn parse(this: &MaterialLoader, json: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setTextures")]
    pub fn set_textures(this: &MaterialLoader, json: &JsValue) -> MaterialLoader;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type BufferGeometryLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> BufferGeometryLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &BufferGeometryLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    );

    #[wasm_bindgen(method)]
    pub fn parse(this: &BufferGeometryLoader, json: &JsValue) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type LoadingManager;

    #[wasm_bindgen(constructor)]
    pub fn new(on_load: &JsValue, on_progress: &JsValue, on_error: &JsValue) -> LoadingManager;

    #[wasm_bindgen(method)]
    pub fn parse(this: &LoadingManager, json: &JsValue) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type ImageLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> ImageLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &ImageLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    ) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type ImageBitmapLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> ImageBitmapLoader;

    #[wasm_bindgen(method, js_name = "setOptions")]
    pub fn set_options(this: &ImageBitmapLoader, options: &JsValue) -> ImageBitmapLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &ImageBitmapLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    ) -> JsValue;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type FileLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> FileLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &FileLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = "setResponseType")]
    pub fn set_response_type(this: &FileLoader, value: &JsValue) -> FileLoader;

    #[wasm_bindgen(method, js_name = "setMimeType")]
    pub fn set_mime_type(this: &FileLoader, value: &JsValue) -> FileLoader;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Loader)]
    pub type AudioLoader;

    #[wasm_bindgen(constructor)]
    pub fn new(manager: &JsValue) -> AudioLoader;

    #[wasm_bindgen(method)]
    pub fn load(
        this: &AudioLoader,
        url: &JsValue,
        on_load: &JsValue,
        on_progress: &JsValue,
        on_error: &JsValue,
    ) -> JsValue;

}
