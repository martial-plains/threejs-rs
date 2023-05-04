use js_sys::{Array, Function, Object};
use wasm_bindgen::prelude::*;

use crate::{
    math::{Euler, Vector3},
    Matrix4, Quaternion,
};

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type InterleavedBuffer;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, stride: &JsValue) -> InterleavedBuffer;

    #[wasm_bindgen(method, js_name = "onUploadCallback")]
    pub fn on_upload_callback(this: &InterleavedBuffer);

    #[wasm_bindgen(method, setter, js_name = "needsUpdate")]
    pub fn set_needs_update(this: &InterleavedBuffer, value: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setUsage")]
    pub fn set_usage(this: &InterleavedBuffer, value: &JsValue) -> InterleavedBuffer;

    #[wasm_bindgen(method, js_name = "copyAt")]
    pub fn copy_at(
        this: &InterleavedBuffer,
        index1: i32,
        attribute: &InterleavedBuffer,
        index2: i32,
    ) -> InterleavedBuffer;

    #[wasm_bindgen(method)]
    pub fn set(this: &InterleavedBuffer, value: &JsValue, offset: i32) -> InterleavedBuffer;

    #[wasm_bindgen(method, js_name = "onUpload")]
    pub fn on_upload(this: &InterleavedBuffer, callback: &JsValue) -> InterleavedBuffer;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type BufferGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new() -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "getIndex")]
    pub fn get_index(this: &BufferGeometry) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "setIndex")]
    pub fn set_index(this: &BufferGeometry, index: &BufferAttribute) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "getAttribute")]
    pub fn get_attribute(this: &BufferGeometry, name: &str) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "setAttribute")]
    pub fn set_attribute(
        this: &BufferGeometry,
        name: &str,
        attribute: &BufferAttribute,
    ) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "deleteAttribute")]
    pub fn delete_attribute(this: &BufferGeometry, name: &str) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "hasAttribute")]
    pub fn has_attribute(this: &BufferGeometry, name: &str) -> bool;

    #[wasm_bindgen(method, js_name = "addGroup")]
    pub fn add_group(this: &BufferGeometry, start: i32, count: i32, material_index: i32);

    #[wasm_bindgen(method, js_name = "clearGroups")]
    pub fn clear_groups(this: &BufferGeometry);

    #[wasm_bindgen(method, js_name = "setDrawRange")]
    pub fn set_draw_range(this: &BufferGeometry, start: i32, count: i32);

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix4(this: &BufferGeometry, matrix: &Matrix4) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "applyQuaternion")]
    pub fn apply_quaternion(this: &BufferGeometry, q: &Quaternion) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "rotateX")]
    pub fn rotate_x(this: &BufferGeometry, angle: f32) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "rotateY")]
    pub fn rotate_y(this: &BufferGeometry, angle: f32) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "rotateZ")]
    pub fn rotate_z(this: &BufferGeometry, angle: f32) -> BufferGeometry;

    #[wasm_bindgen(method)]
    pub fn translate(this: &BufferGeometry, x: f32, y: f32, z: f32) -> BufferGeometry;

    #[wasm_bindgen(method)]
    pub fn scale(this: &BufferGeometry, x: f32, y: f32, z: f32) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "lookAt")]
    pub fn look_at(this: &BufferGeometry, vector: &Vector3) -> BufferGeometry;

    #[wasm_bindgen(method)]
    pub fn center(this: &BufferGeometry) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "setFromPoints")]
    pub fn set_from_points(this: &BufferGeometry, points: &Array) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "computeBoundingBox")]
    pub fn compute_bounding_box(this: &BufferGeometry);

    #[wasm_bindgen(method, js_name = "computeBoundingSphere")]
    pub fn compute_bounding_sphere(this: &BufferGeometry);

    #[wasm_bindgen(method, js_name = "computeTangents")]
    pub fn compute_tangents(this: &BufferGeometry);

    #[wasm_bindgen(method, js_name = "computeVertexNormals")]
    pub fn compute_vertex_normals(this: &BufferGeometry);

    #[wasm_bindgen(method)]
    pub fn merge(this: &BufferGeometry) -> BufferGeometry;

    #[wasm_bindgen(method, js_name = "normalizeNormals")]
    pub fn normalize_normals(this: &BufferGeometry);

    #[wasm_bindgen(method, js_name = "toNonIndexed")]
    pub fn to_non_indexed(this: &BufferGeometry);

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &BufferGeometry);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Object3D;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Object3D;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Object3D) -> Vector3;

    #[wasm_bindgen(method, getter)]
    pub fn rotation(this: &Object3D) -> Euler;

    #[wasm_bindgen(method, js_name = "onBeforeRender")]
    pub fn on_before_render(this: &Object3D);

    #[wasm_bindgen(method, js_name = "onAfterRender")]
    pub fn on_after_render(this: &Object3D);

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix_4(this: &Object3D, matrix: &Matrix4);

    #[wasm_bindgen(method, js_name = "applyQuaternion")]
    pub fn apply_quaternion(this: &Object3D, q: &Quaternion) -> Object3D;

    #[wasm_bindgen(method, js_name = "setRotationFromAxisAngle")]
    pub fn set_rotation_from_axis_angle(this: &Object3D, axis: &Vector3, angle: f32);

    #[wasm_bindgen(method, js_name = "setRotationFromEuler")]
    pub fn set_rotation_from_euler(this: &Object3D, euler: &Euler);

    #[wasm_bindgen(method, js_name = "setRotationFromMatrix")]
    pub fn set_rotation_from_matrix(this: &Object3D, m: &Matrix4);

    #[wasm_bindgen(method, js_name = "setRotationFromQuaternion")]
    pub fn set_rotation_from_quaternion(this: &Object3D, q: &Quaternion);

    #[wasm_bindgen(method, js_name = "rotateOnAxis")]
    pub fn rotate_on_axis(this: &Object3D, axis: &Vector3, angle: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "rotateOnWorldAxis")]
    pub fn rotate_on_world_axis(this: &Object3D, axis: &Vector3, angle: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "rotateX")]
    pub fn rotate_x(this: &Object3D, angle: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "rotateY")]
    pub fn rotate_y(this: &Object3D, angle: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "rotateZ")]
    pub fn rotate_z(this: &Object3D, angle: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "translateOnAxis")]
    pub fn translate_on_axis(this: &Object3D, axis: &Vector3, distance: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "translateX")]
    pub fn translate_x(this: &Object3D, distance: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "translateY")]
    pub fn translate_y(this: &Object3D, distance: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "translateZ")]
    pub fn translate_z(this: &Object3D, distance: f32) -> Object3D;

    #[wasm_bindgen(method, js_name = "localToWorld")]
    pub fn local_to_world(this: &Object3D, vector: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "worldToLocal")]
    pub fn world_to_local(this: &Object3D, vector: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "lookAt")]
    pub fn look_at(this: &Object3D, x: f32, y: f32, z: f32);

    #[wasm_bindgen(method)]
    pub fn add(this: &Object3D, object: &Object3D) -> Object3D;

    #[wasm_bindgen(method)]
    pub fn remove(this: &Object3D, object: &Object3D) -> Object3D;

    #[wasm_bindgen(method, js_name = "removeFromParent")]
    pub fn remove_from_parent(this: &Object3D, object: &Object3D) -> Object3D;

    #[wasm_bindgen(method)]
    pub fn clear(this: &Object3D) -> Object3D;

    #[wasm_bindgen(method)]
    pub fn attach(this: &Object3D, object: &Object3D) -> Object3D;

    #[wasm_bindgen(method, js_name = "getObjectById")]
    pub fn get_object_by_id(this: &Object3D, id: i32) -> JsValue;

    #[wasm_bindgen(method, js_name = "getObjectByName")]
    pub fn get_object_by_name(this: &Object3D, name: &str) -> JsValue;

    #[wasm_bindgen(method, js_name = "getObjectByProperty")]
    pub fn get_object_by_property(this: &Object3D, name: &str, value: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "getObjectByProperty")]
    pub fn get_objects_by_property(this: &Object3D, name: &str, value: &JsValue) -> Vec<JsValue>;

    #[wasm_bindgen(method, js_name = "getWorldPosition")]
    pub fn get_world_position(this: &Object3D, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "getWorldQuaternion")]
    pub fn get_world_quaternion(this: &Object3D, target: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method, js_name = "getWorldScale")]
    pub fn get_world_scale(this: &Object3D, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "getWorldDirection")]
    pub fn get_world_direction(this: &Object3D, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn raycast(this: &Object3D);

    #[wasm_bindgen(method)]
    pub fn traverse(this: &Object3D, callback: &Function);

    #[wasm_bindgen(method, js_name = "traverseVisible")]
    pub fn traverse_visible(this: &Object3D, callback: &Function);

    #[wasm_bindgen(method, js_name = "traverseAncestors")]
    pub fn traverse_ancestors(this: &Object3D, callback: &Function);

    #[wasm_bindgen(method, js_name = "updateMatrix")]
    pub fn update_matrix(this: &Object3D);

    #[wasm_bindgen(method, js_name = "updateMatrixWorld")]
    pub fn update_matrix_world(this: &Object3D, force: &JsValue);

    #[wasm_bindgen(method, js_name = "updateWorldMatrix")]
    pub fn update_world_matrix(this: &Object3D, force: &JsValue);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Uniform;

    #[wasm_bindgen(constructor)]
    pub fn new(value: &Object) -> Uniform;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type UniformsGroup;

    #[wasm_bindgen(constructor)]
    pub fn new() -> UniformsGroup;

    #[wasm_bindgen(method)]
    pub fn add(this: &UniformsGroup, uniform: &JsValue) -> UniformsGroup;

    #[wasm_bindgen(method)]
    pub fn remove(this: &UniformsGroup, uniform: &JsValue) -> UniformsGroup;

    #[wasm_bindgen(method, js_name = "setName")]
    pub fn set_name(this: &UniformsGroup, name: &JsValue) -> UniformsGroup;

    #[wasm_bindgen(method, js_name = "setUsage")]
    pub fn set_usage(this: &UniformsGroup, value: &JsValue) -> UniformsGroup;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferGeometry)]
    pub type InstancedBufferGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new() -> InstancedBufferGeometry;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type InterleavedBufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(
        interleavedBuffer: &JsValue,
        itemSize: &JsValue,
        offset: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        interleavedBuffer: &JsValue,
        itemSize: &JsValue,
        offset: &JsValue,
        normalized: bool,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, getter)]
    pub fn count(this: &InterleavedBufferAttribute) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn array(this: &InterleavedBufferAttribute) -> JsValue;

    #[wasm_bindgen(method, setter, js_name = "needsUpdate")]
    pub fn needs_update(this: &InterleavedBufferAttribute) -> JsValue;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix_4(
        this: &InterleavedBufferAttribute,
        m: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "applyNormalMatrix")]
    pub fn apply_normal_matrix(
        this: &InterleavedBufferAttribute,
        m: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "transformDirection")]
    pub fn transform_direction(
        this: &InterleavedBufferAttribute,
        m: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "setX")]
    pub fn set_x(
        this: &InterleavedBufferAttribute,
        index: &JsValue,
        x: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "setY")]
    pub fn set_y(
        this: &InterleavedBufferAttribute,
        index: &JsValue,
        y: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "setZ")]
    pub fn set_z(
        this: &InterleavedBufferAttribute,
        index: &JsValue,
        z: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "setW")]
    pub fn set_w(
        this: &InterleavedBufferAttribute,
        index: &JsValue,
        w: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "getX")]
    pub fn get_x(this: &InterleavedBufferAttribute, index: &JsValue) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "getY")]
    pub fn get_y(this: &InterleavedBufferAttribute, index: &JsValue) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "getZ")]
    pub fn get_z(this: &InterleavedBufferAttribute, index: &JsValue) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "getW")]
    pub fn get_w(this: &InterleavedBufferAttribute, index: &JsValue) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "setXY")]
    pub fn set_xy(
        this: &InterleavedBufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "setXYZ")]
    pub fn set_xyz(
        this: &InterleavedBufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
        z: &JsValue,
    ) -> InterleavedBufferAttribute;

    #[wasm_bindgen(method, js_name = "setXYZW")]
    pub fn set_xyzw(
        this: &InterleavedBufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
        z: &JsValue,
        w: &JsValue,
    ) -> InterleavedBufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = InterleavedBuffer)]
    pub type InstancedInterleavedBuffer;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, stride: &JsValue) -> InstancedInterleavedBuffer;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        array: &JsValue,
        stride: &JsValue,
        mesh_per_attribute: i32,
    ) -> InstancedInterleavedBuffer;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, item_size: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new_with(array: &JsValue, item_size: &JsValue, normalized: bool) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "onUploadCallback")]
    pub fn on_upload_callback(this: &BufferAttribute);

    #[wasm_bindgen(method, setter, js_name = "needsUpdate")]
    pub fn set_needs_update(this: &BufferAttribute, value: &JsValue);

    #[wasm_bindgen(method, js_name = "setUsage")]
    pub fn set_usage(this: &BufferAttribute, value: &JsValue);

    #[wasm_bindgen(method, js_name = "copyAt")]
    pub fn copy_at(
        this: &BufferAttribute,
        index1: &JsValue,
        attribute: &JsValue,
        index2: &JsValue,
    ) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "copyArray")]
    pub fn copy_array(this: &BufferAttribute, array: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "applyMatrix3")]
    pub fn apply_matrix_3(this: &BufferAttribute, m: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix_4(this: &BufferAttribute, m: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "applyNormalMatrix")]
    pub fn apply_normal_matrix(this: &BufferAttribute, m: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "transformDirection")]
    pub fn transform_direction(this: &BufferAttribute, m: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method)]
    pub fn set(this: &BufferAttribute, value: &JsValue, offset: i32) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "getX")]
    pub fn get_x(this: &BufferAttribute, index: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setX")]
    pub fn set_x(this: &BufferAttribute, index: &JsValue, x: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "getY")]
    pub fn get_y(this: &BufferAttribute, index: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setY")]
    pub fn set_y(this: &BufferAttribute, index: &JsValue, y: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "getZ")]
    pub fn get_z(this: &BufferAttribute, index: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setZ")]
    pub fn set_z(this: &BufferAttribute, index: &JsValue, z: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "getW")]
    pub fn get_w(this: &BufferAttribute, index: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setW")]
    pub fn set_w(this: &BufferAttribute, index: &JsValue, w: &JsValue) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "setXY")]
    pub fn set_xy(
        this: &BufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
    ) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "setXYZ")]
    pub fn set_xyz(
        this: &BufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
        z: &JsValue,
    ) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "setXYZW")]
    pub fn set_xyzw(
        this: &BufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
        z: &JsValue,
        w: &JsValue,
    ) -> BufferAttribute;

    #[wasm_bindgen(method, js_name = "onUpload")]
    pub fn on_upload(this: &BufferAttribute, callback: &JsValue) -> BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Int8BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, item_size: &JsValue, normalized: &JsValue) -> Int8BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Uint8BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, item_size: &JsValue, normalized: &JsValue) -> Uint8BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Uint8ClampedBufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(
        array: &JsValue,
        item_size: &JsValue,
        normalized: &JsValue,
    ) -> Uint8ClampedBufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Int16BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, item_size: &JsValue, normalized: &JsValue) -> Int16BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Uint16BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, item_size: &JsValue, normalized: &JsValue)
        -> Uint16BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Int32BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, item_size: &JsValue, normalized: &JsValue) -> Int32BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Uint32BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(array: &JsValue, item_size: &JsValue, normalized: &JsValue)
        -> Uint32BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Float16BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(
        array: &JsValue,
        item_size: &JsValue,
        normalized: &JsValue,
    ) -> Float16BufferAttribute;

    #[wasm_bindgen(method, js_name = "getX")]
    pub fn get_x(this: &Float16BufferAttribute, index: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setX")]
    pub fn set_x(
        this: &Float16BufferAttribute,
        index: &JsValue,
        x: &JsValue,
    ) -> Float16BufferAttribute;

    #[wasm_bindgen(method, js_name = "getY")]
    pub fn get_y(this: &Float16BufferAttribute, index: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setY")]
    pub fn set_y(
        this: &Float16BufferAttribute,
        index: &JsValue,
        y: &JsValue,
    ) -> Float16BufferAttribute;

    #[wasm_bindgen(method, js_name = "getZ")]
    pub fn get_z(this: &Float16BufferAttribute, index: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setZ")]
    pub fn set_z(
        this: &Float16BufferAttribute,
        index: &JsValue,
        z: &JsValue,
    ) -> Float16BufferAttribute;

    #[wasm_bindgen(method, js_name = "getW")]
    pub fn get_w(this: &Float16BufferAttribute, index: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "setW")]
    pub fn set_w(
        this: &Float16BufferAttribute,
        index: &JsValue,
        w: &JsValue,
    ) -> Float16BufferAttribute;

    #[wasm_bindgen(method, js_name = "setXY")]
    pub fn set_xy(
        this: &Float16BufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
    ) -> Float16BufferAttribute;

    #[wasm_bindgen(method, js_name = "setXYZ")]
    pub fn set_xyz(
        this: &Float16BufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
        z: &JsValue,
    ) -> Float16BufferAttribute;

    #[wasm_bindgen(method, js_name = "setXYZW")]
    pub fn set_xyzw(
        this: &Float16BufferAttribute,
        index: &JsValue,
        x: &JsValue,
        y: &JsValue,
        z: &JsValue,
        w: &JsValue,
    ) -> Float16BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Float32BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(
        array: &JsValue,
        item_size: &JsValue,
        normalized: &JsValue,
    ) -> Float32BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type Float64BufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(
        array: &JsValue,
        item_size: &JsValue,
        normalized: &JsValue,
    ) -> Float64BufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    #[wasm_bindgen(extends = BufferAttribute)]
    pub type InstancedBufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(
        array: &JsValue,
        item_size: &JsValue,
        normalized: &JsValue,
        mesh_per_attribute: i32,
    ) -> InstancedBufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type GLBufferAttribute;

    #[wasm_bindgen(constructor)]
    pub fn new(
        buffer: &JsValue,
        r#type: &JsValue,
        item_size: &JsValue,
        element_size: &JsValue,
        count: &JsValue,
    ) -> GLBufferAttribute;

    #[wasm_bindgen(method, setter, js_name = "needsUpdate")]
    pub fn set_needs_update(this: &GLBufferAttribute, value: &JsValue);

    #[wasm_bindgen(method, js_name = "setBuffer")]
    pub fn set_buffer(this: &GLBufferAttribute, buffer: &JsValue) -> GLBufferAttribute;

    #[wasm_bindgen(method, js_name = "setType")]
    pub fn set_type(
        this: &GLBufferAttribute,
        r#type: &JsValue,
        element_size: &JsValue,
    ) -> GLBufferAttribute;

    #[wasm_bindgen(method, js_name = "setItemSize")]
    pub fn set_item_size(this: &GLBufferAttribute, item_size: &JsValue) -> GLBufferAttribute;

    #[wasm_bindgen(method, js_name = "setCount")]
    pub fn set_count(this: &GLBufferAttribute, item_size: &JsValue) -> GLBufferAttribute;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Raycaster;

    #[wasm_bindgen(constructor)]
    pub fn new(origin: &JsValue, direction: &JsValue) -> Raycaster;

    #[wasm_bindgen(constructor)]
    pub fn new_with(origin: &JsValue, direction: &JsValue, near: f64, far: f64) -> Raycaster;

    #[wasm_bindgen(method)]
    pub fn set(this: &Raycaster, origin: &JsValue, direction: &JsValue) -> Raycaster;

    #[wasm_bindgen(method, js_name = "setFromCamera")]
    pub fn set_from_camera(this: &Raycaster, coords: &JsValue, camera: &JsValue) -> Raycaster;

    #[wasm_bindgen(method, js_name = "intersectObject")]
    pub fn intersect_object(
        this: &Raycaster,
        object: &JsValue,
        recursive: bool,
        intersects: Vec<JsValue>,
    ) -> Raycaster;

    #[wasm_bindgen(method, js_name = "intersectObjects")]
    pub fn intersect_objects(
        this: &Raycaster,
        object: &JsValue,
        recursive: bool,
        intersects: Vec<JsValue>,
    ) -> Raycaster;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Layers;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Layers;

    #[wasm_bindgen(method)]
    pub fn set(this: &Layers, channel: &JsValue);

    #[wasm_bindgen(method)]
    pub fn enable(this: &Layers, channel: &JsValue);

    #[wasm_bindgen(method, js_name = "enableAll")]
    pub fn enable_all(this: &Layers, channel: &JsValue);

    #[wasm_bindgen(method)]
    pub fn toggle(this: &Layers, channel: &JsValue);

    #[wasm_bindgen(method)]
    pub fn disable(this: &Layers, channel: &JsValue);

    #[wasm_bindgen(method, js_name = "disableAll")]
    pub fn disable_all(this: &Layers, channel: &JsValue);

    #[wasm_bindgen(method)]
    pub fn test(this: &Layers, layers: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "isEnabled")]
    pub fn is_enabled(this: &Layers, channel: &JsValue) -> bool;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type EventDispatcher;

    #[wasm_bindgen(method, js_name = "addEventListener")]
    pub fn add_event_listener(this: &EventDispatcher, r#type: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method, js_name = "hasEventListener")]
    pub fn has_event_listener(this: &EventDispatcher, r#type: &JsValue, listener: &JsValue)
        -> bool;

    #[wasm_bindgen(method, js_name = "removeEventListener")]
    pub fn remove_event_listener(this: &EventDispatcher, r#type: &JsValue, listener: &JsValue);

    #[wasm_bindgen(method, js_name = "dispatchEvent")]
    pub fn dispatch_event(this: &EventDispatcher, event: &JsValue);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Clock;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Clock;

    #[wasm_bindgen(constructor)]
    pub fn new_with(auto_start: bool) -> Clock;

    #[wasm_bindgen(method)]
    pub fn start(this: &Clock);

    #[wasm_bindgen(method)]
    pub fn stop(this: &Clock);

    #[wasm_bindgen(method, js_name = "getElapsedTime")]
    pub fn get_elapsed_time(this: &Clock);

    #[wasm_bindgen(method, js_name = "getDelta")]
    pub fn get_delta(this: &Clock);

}
