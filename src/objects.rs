use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    core::{BufferGeometry, Object3D},
    materials::LineBasicMaterial,
    MeshBasicMaterial,
};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Mesh;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Mesh;

    #[wasm_bindgen(constructor)]
    pub fn new_with(geometry: &BufferGeometry, material: &MeshBasicMaterial) -> Mesh;

    #[wasm_bindgen(method, js_name = "updateMorphTargets")]
    pub fn update_morph_targets(this: &Mesh);

    #[wasm_bindgen(method, js_name = "getVertexPosition")]
    pub fn get_vertex_position(this: &Mesh, index: &JsValue, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn raycast(this: &Mesh, raycaster: &JsValue, intersects: &JsValue);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type LOD;

    #[wasm_bindgen(constructor)]
    pub fn new() -> LOD;

    #[wasm_bindgen(method, js_name = "addLevel")]
    pub fn add_level(this: &LOD, object: &JsValue, distance: i32, hysteresis: i32) -> LOD;

    #[wasm_bindgen(method, js_name = "getCurrentLevel")]
    pub fn get_current_level(this: &LOD) -> i32;

    #[wasm_bindgen(method, js_name = "getObjectForDistance")]
    pub fn get_object_for_distance(this: &LOD, distance: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn raycast(this: &LOD, raycaster: &JsValue, intersects: &JsValue);

    #[wasm_bindgen(method)]
    pub fn update(this: &LOD, camera: &JsValue);

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &LOD, meta: &JsValue);

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Sprite;

    #[wasm_bindgen(constructor)]
    pub fn new(material: &JsValue) -> Sprite;

    #[wasm_bindgen(method)]
    pub fn raycast(this: &Sprite, raycaster: &JsValue, intersects: &JsValue);

    #[wasm_bindgen(method, js_name = "transformVertex")]
    pub fn transform_vertex(
        this: &Sprite,
        vertexPosition: &JsValue,
        mvPosition: &JsValue,
        center: &JsValue,
        scale: &JsValue,
        sin: &JsValue,
        cos: &JsValue,
    );
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Mesh)]
    pub type SkinnedMesh;

    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &JsValue, material: &JsValue) -> SkinnedMesh;

    #[wasm_bindgen(method, js_name = "computeBoundingBox")]
    pub fn compute_bounding_box(this: &SkinnedMesh);

    #[wasm_bindgen(method, js_name = "computeBoundingSphere")]
    pub fn compute_bounding_sphere(this: &SkinnedMesh);

    #[wasm_bindgen(method)]
    pub fn bind(this: &SkinnedMesh, skeleton: &JsValue, bindMatrix: &JsValue);

    #[wasm_bindgen(method)]
    pub fn pose(this: &SkinnedMesh);

    #[wasm_bindgen(method, js_name = "normalizeSkinWeights")]
    pub fn normalize_skin_weights(this: &SkinnedMesh);

    #[wasm_bindgen(method, js_name = "updateMatrixWorld")]
    pub fn update_matrix_world(this: &SkinnedMesh, force: &JsValue);

    #[wasm_bindgen(method, js_name = "applyBoneTransform")]
    pub fn apply_bone_transform(this: &SkinnedMesh, index: &JsValue, vector: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "boneTransform")]
    pub fn bone_transform(this: &SkinnedMesh, index: &JsValue, vector: &JsValue) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Skeleton;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Skeleton;

    #[wasm_bindgen(constructor)]
    pub fn new_with(bones: &JsValue, boneInverses: &JsValue) -> Skeleton;

    #[wasm_bindgen(method)]
    pub fn init(this: &Skeleton);

    #[wasm_bindgen(method, js_name = "calculateInverses")]
    pub fn calculate_inverses(this: &Skeleton);

    #[wasm_bindgen(method)]
    pub fn pose(this: &Skeleton);

    #[wasm_bindgen(method)]
    pub fn update(this: &Skeleton);

    #[wasm_bindgen(method, js_name = "computeBoneTexture")]
    pub fn compute_bone_texture(this: &Skeleton) -> Skeleton;

    #[wasm_bindgen(method, js_name = "getBoneByName")]
    pub fn get_bone_by_name(this: &Skeleton, name: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "fromJSON")]
    pub fn from_json(this: &Skeleton, json: &JsValue, bones: &JsValue) -> Skeleton;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Skeleton) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Bone;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Bone;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Mesh)]
    pub type InstancedMesh;

    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &JsValue, material: &JsValue, count: &JsValue) -> InstancedMesh;

    #[wasm_bindgen(method, js_name = "computeBoundingBox")]
    pub fn compute_bounding_box(this: &InstancedMesh);

    #[wasm_bindgen(method, js_name = "computeBoundingSphere")]
    pub fn compute_bounding_sphere(this: &InstancedMesh);

    #[wasm_bindgen(method, js_name = "getColorAt")]
    pub fn get_color_at(this: &InstancedMesh, index: &JsValue, color: &JsValue);

    #[wasm_bindgen(method, js_name = "getMatrixAt")]
    pub fn get_matrix_at(this: &InstancedMesh, index: &JsValue, color: &JsValue);

    #[wasm_bindgen(method)]
    pub fn raycast(this: &InstancedMesh, raycaster: &JsValue, intersects: &JsValue);

    #[wasm_bindgen(method, js_name = "setColorAt")]
    pub fn set_color_at(this: &InstancedMesh, index: &JsValue, color: &JsValue);

    #[wasm_bindgen(method, js_name = "setMatrixAt")]
    pub fn set_matrix_at(this: &InstancedMesh, index: &JsValue, matrix: &JsValue);

    #[wasm_bindgen(method, js_name = "updateMorphTargets")]
    pub fn update_morph_targets(this: &InstancedMesh);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Line;

    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &BufferGeometry, material: &LineBasicMaterial) -> Line;

    #[wasm_bindgen(method, js_name = "computeLineDistances")]
    pub fn compute_line_distances(this: &Line);

    #[wasm_bindgen(method)]
    pub fn raycast(this: &Line, raycaster: &JsValue, intersects: &JsValue);

    #[wasm_bindgen(method, js_name = "updateMorphTargets")]
    pub fn update_morph_targets(this: &Line);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Line)]
    pub type LineSegments;

    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &JsValue, material: &JsValue) -> LineSegments;

    #[wasm_bindgen(method, js_name = "computeLineDistances")]
    pub fn compute_line_distances(this: &LineSegments) -> LineSegments;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Line)]
    pub type LineLoop;

    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &JsValue, material: &JsValue) -> LineLoop;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Points;

    #[wasm_bindgen(constructor)]
    pub fn new(geometry: &BufferGeometry, material: &LineBasicMaterial) -> Points;

    #[wasm_bindgen(method)]
    pub fn raycast(this: &Points, raycaster: &JsValue, intersects: &JsValue);

    #[wasm_bindgen(method, js_name = "updateMorphTargets")]
    pub fn update_morph_targets(this: &Points);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = Object3D)]
    pub type Group;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Group;

}
