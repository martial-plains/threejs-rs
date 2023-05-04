use js_sys::{Array, Number, Object};
use wasm_bindgen::prelude::*;

use crate::{BufferAttribute, Camera, Object3D, Sprite};

pub mod interpolants;

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Interpolant;

    #[wasm_bindgen(constructor)]
    pub fn new(
        parameterPositions: &JsValue,
        sampleValues: &JsValue,
        sampleSize: &JsValue,
        resultBuffer: &JsValue,
    ) -> Interpolant;

    #[wasm_bindgen(method)]
    pub fn evaluate(this: &Interpolant, t: &JsValue);

    #[wasm_bindgen(method, js_name = "getSettings_")]
    pub fn get_settings(this: &Interpolant);

    #[wasm_bindgen(method, js_name = "copySampleValue_")]
    pub fn copy_sample_value(this: &Interpolant, index: &JsValue);

    #[wasm_bindgen(method, js_name = "interpolate_")]
    pub fn interpolate(this: &Interpolant);

    #[wasm_bindgen(method, js_name = "intervalChanged_")]
    pub fn interval_changed(this: &Interpolant);
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Euler;

    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64, order: JsValue) -> Euler;

    #[wasm_bindgen(method)]
    pub fn set(this: &Euler, x: f64, y: f64, z: f64);

    #[wasm_bindgen(method, js_name = "set")]
    pub fn set_with_order(this: &Euler, x: f64, y: f64, z: f64);

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Euler) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Euler, x: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Euler) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Euler, y: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Euler) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Euler, z: f64);

    #[wasm_bindgen(method, getter)]
    pub fn order(this: &Euler) -> JsValue;

    #[wasm_bindgen(method, setter)]
    pub fn set_order(this: &Euler, order: JsValue);

    #[wasm_bindgen(method, js_name = "setFromRotationMatrix")]
    pub fn set_from_rotation_matrix(this: &Euler, m: &Matrix4);

    #[wasm_bindgen(method, js_name = "setFromRotationMatrix")]
    pub fn set_from_rotation_matrix_with(this: &Euler, m: &Matrix4, order: &str) -> Euler;

    #[wasm_bindgen(method, js_name = "setFromQuaternion")]
    pub fn set_from_quaternion(this: &Euler, q: &JsValue, order: &str, update: bool) -> Euler;

    #[wasm_bindgen(method, js_name = "setFromVector3")]
    pub fn set_from_vector3(this: &Euler, v: &JsValue) -> Euler;

    #[wasm_bindgen(method, js_name = "setFromVector3")]
    pub fn set_from_vector3_with(this: &Euler, v: &JsValue, order: &str, update: bool) -> Euler;

    #[wasm_bindgen(method)]
    pub fn reorder(this: &Euler, new_order: &str) -> Euler;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Euler, euler: &Euler) -> bool;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &Line3, array: &Array) -> Line3;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &Line3, array: &Array, offset: i32) -> Array;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Vector3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Vector3;

    #[wasm_bindgen(constructor)]
    pub fn new_with(x: f64, y: f64, z: f64) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn set(this: &Vector3, x: f64, y: f64, z: f64) -> Vector3;

    #[wasm_bindgen(method, js_name = "setScalar")]
    pub fn set_scalar(this: &Vector3, scalar: f64) -> Vector3;

    #[wasm_bindgen(method, js_name = "setX")]
    pub fn set_x(this: &Vector3, x: f64) -> Vector3;

    #[wasm_bindgen(method, js_name = "setY")]
    pub fn set_y(this: &Vector3, y: f64) -> Vector3;

    #[wasm_bindgen(method, js_name = "setZ")]
    pub fn set_z(this: &Vector3, z: f64) -> Vector3;

    #[wasm_bindgen(method, js_name = "setComponent")]
    pub fn set_component(this: &Vector3, index: i32, value: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "getComponent")]
    pub fn get_component(this: &Vector3, index: i32) -> f32;

    #[wasm_bindgen(method)]
    pub fn add(this: &Vector3, v: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "addScalar")]
    pub fn add_scalar(this: &Vector3, s: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "addVectors")]
    pub fn addVectors(this: &Vector3, a: &Vector3, b: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "addScaledVector")]
    pub fn addScaledVector(this: &Vector3, v: &Vector3, s: f32) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn sub(this: &Vector3, v: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "subScalar")]
    pub fn sub_scalar(this: &Vector3, s: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "subVectors")]
    pub fn sub_vectors(this: &Vector3, a: &Vector3, b: &Vector3) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn multiply(this: &Vector3, v: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "multiplyScalar")]
    pub fn multiply_scalar(this: &Vector3, scalar: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "multiplyVectors")]
    pub fn multiply_vectors(this: &Vector3, a: &Vector3, b: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "applyEuler")]
    pub fn apply_euler(this: &Vector3, euler: &Euler) -> Vector3;

    #[wasm_bindgen(method, js_name = "applyAxisAngle")]
    pub fn apply_axis_angle(this: &Vector3, axis: &Vector3, angle: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "applyMatrix3")]
    pub fn apply_matrix3(this: &Vector3, m: &Matrix3) -> Vector3;

    #[wasm_bindgen(method, js_name = "applyNormalMatrix")]
    pub fn apply_normal_matrix(this: &Vector3, m: &Matrix3) -> Vector3;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix4(this: &Vector3, m: &Matrix4) -> Vector3;

    #[wasm_bindgen(method, js_name = "applyQuaternion")]
    pub fn apply_quaternion(this: &Vector3, m: &JsValue) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn project(this: &Vector3, camera: &Camera) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn unproject(this: &Vector3, camera: &Camera) -> Vector3;

    #[wasm_bindgen(method, js_name = "transformDirection")]
    pub fn transform_direction(this: &Vector3, m: &Matrix4) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn divide(this: &Vector3, v: Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "divideScalar")]
    pub fn divide_scalar(this: &Vector3, scalar: f32) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn min(this: &Vector3, v: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn max(this: &Vector3, v: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn clamp(this: &Vector3, min: &Vector3, max: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "clampScalar")]
    pub fn clamp_scalar(this: &Vector3, min_val: f32, max_val: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "clampLength")]
    pub fn clamp_length(this: &Vector3, min: f32, max: f32) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn floor(this: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn ceil(this: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn round(this: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "roundToZero")]
    pub fn round_to_zero(this: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn negate(this: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn dot(this: &Vector3, v: Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "lengthSq")]
    pub fn length_sq(this: &Vector3) -> f32;

    #[wasm_bindgen(method)]
    pub fn length(this: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "manhattanLength")]
    pub fn manhattan_length(this: &Vector3) -> f32;

    #[wasm_bindgen(method)]
    pub fn normalize(this: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "setLength")]
    pub fn set_length(this: &Vector3, length: f32) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn lerp(this: &Vector3, v: &Vector3, alpha: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "lerpVectors")]
    pub fn lerp_vectors(this: &Vector3, v1: &Vector3, v2: &Vector3, alpha: f32) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn cross(this: &Vector3, v: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "crossVectors")]
    pub fn cross_vectors(this: &Vector3, a: &Vector3, b: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "projectOnVector")]
    pub fn project_on_vector(this: &Vector3, v: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "projectOnPlane")]
    pub fn projectOnPlane(this: &Vector3, plane_normal: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn reflect(this: &Vector3, normal: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "angleTo")]
    pub fn angle_to(this: &Vector3, v: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "distanceTo")]
    pub fn distance_to(this: &Vector3, v: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "distanceToSquared")]
    pub fn distance_to_squared(this: &Vector3, v: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "manhattanDistanceTo")]
    pub fn manhattan_distance_to(this: &Vector3, v: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "setFromSpherical")]
    pub fn set_from_spherical(this: &Vector3, s: &Spherical) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromSphericalCoords")]
    pub fn set_from_spherical_coords(this: &Vector3, radius: f32, phi: f32, theta: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromCylindrical")]
    pub fn set_from_cylindrical(this: &Vector3, c: &Cylindrical) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromCylindricalCoords")]
    pub fn set_from_cylindrical_coords(this: &Vector3, radius: f32, theta: f32, y: f32) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromMatrixPosition")]
    pub fn set_from_matrix_position(this: &Vector3, m: &Matrix4) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromMatrixScale")]
    pub fn set_from_matrix_scale(this: &Vector3, m: &Matrix4) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromMatrixColumn")]
    pub fn set_from_matrix_column(this: &Vector3, m: &Matrix4, index: i32) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromMatrix3Column")]
    pub fn set_from_matrix3_column(this: &Vector3, m: &Matrix3, index: i32) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromEuler")]
    pub fn set_from_euler(this: &Vector3, e: &Euler) -> Vector3;

    #[wasm_bindgen(method, js_name = "setFromColor")]
    pub fn set_from_color(this: &Vector3, c: &JsValue) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Vector3, v: &Vector3) -> bool;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &Vector3, array: &Array) -> Vector3;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &Vector3, array: &Array, offset: i32) -> Array;

    #[wasm_bindgen(method, js_name = "fromBufferAttribute")]
    pub fn from_buffer_attribute(
        this: &Vector3,
        attribute: &BufferAttribute,
        index: i32,
    ) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn random(this: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "randomDirection")]
    pub fn random_direction(this: &Vector3) -> Vector3;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Vector2;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Vector2;

    #[wasm_bindgen(constructor)]
    pub fn new_with(x: f32, y: f32) -> Vector2;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &Vector2) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_height(this: &Vector2) -> f32;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &Vector2) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_width(this: &Vector2) -> f32;

    #[wasm_bindgen(method)]
    pub fn set(this: &Vector2, x: f32, y: f32) -> Vector2;

    #[wasm_bindgen(method, js_name = "setScalar")]
    pub fn set_scalar(this: &Vector2, scalar: f64) -> Vector2;

    #[wasm_bindgen(method, js_name = "setX")]
    pub fn set_x(this: &Vector2, x: f64) -> Vector2;

    #[wasm_bindgen(method, js_name = "setY")]
    pub fn set_y(this: &Vector2, y: f64) -> Vector2;

    #[wasm_bindgen(method, js_name = "setComponent")]
    pub fn set_component(this: &Vector2, index: i32, value: f32) -> Vector2;

    #[wasm_bindgen(method, js_name = "getComponent")]
    pub fn get_component(this: &Vector2, index: i32) -> f32;

    #[wasm_bindgen(method)]
    pub fn add(this: &Vector2, v: &Vector2) -> Vector2;

    #[wasm_bindgen(method, js_name = "addScalar")]
    pub fn add_scalar(this: &Vector2, s: f32) -> Vector2;

    #[wasm_bindgen(method, js_name = "addVectors")]
    pub fn addVectors(this: &Vector2, a: &Vector2, b: &Vector2) -> Vector2;

    #[wasm_bindgen(method, js_name = "addScaledVector")]
    pub fn addScaledVector(this: &Vector2, v: &Vector2, s: f32) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn sub(this: &Vector2, v: &Vector2) -> Vector2;

    #[wasm_bindgen(method, js_name = "subScalar")]
    pub fn sub_scalar(this: &Vector2, s: f32) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn multiply(this: &Vector2, v: &Vector2) -> Vector2;

    #[wasm_bindgen(method, js_name = "multiplyScalar")]
    pub fn multiply_scalar(this: &Vector2, scalar: f32) -> Vector2;

    #[wasm_bindgen(method, js_name = "multiplyVectors")]
    pub fn multiply_vectors(this: &Vector2, a: &Vector2, b: &Vector2) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn divide(this: &Vector2, v: Vector2) -> Vector2;

    #[wasm_bindgen(method, js_name = "divideScalar")]
    pub fn divide_scalar(this: &Vector2, scalar: f32) -> Vector2;

    #[wasm_bindgen(method, js_name = "applyMatrix3")]
    pub fn apply_matrix3(this: &Vector2, m: &Matrix3) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn min(this: &Vector2, v: &Vector2) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn max(this: &Vector2, v: &Vector2) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn clamp(this: &Vector2, min: &Vector2, max: &Vector2) -> Vector2;

    #[wasm_bindgen(method, js_name = "clampScalar")]
    pub fn clamp_scalar(this: &Vector2, min_val: f32, max_val: f32) -> Vector2;

    #[wasm_bindgen(method, js_name = "clampLength")]
    pub fn clamp_length(this: &Vector2, min: f32, max: f32) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn floor(this: &Vector2) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn ceil(this: &Vector2) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn round(this: &Vector2) -> Vector2;

    #[wasm_bindgen(method, js_name = "roundToZero")]
    pub fn round_to_zero(this: &Vector2) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn negate(this: &Vector2) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn dot(this: &Vector2, v: Vector2) -> f32;

    #[wasm_bindgen(method)]
    pub fn cross(this: &Vector2, v: &Vector2) -> Vector2;

    #[wasm_bindgen(method, js_name = "lengthSq")]
    pub fn length_sq(this: &Vector2) -> f32;

    #[wasm_bindgen(method)]
    pub fn length(this: &Vector2) -> f32;

    #[wasm_bindgen(method, js_name = "manhattanLength")]
    pub fn manhattan_length(this: &Vector2) -> f32;

    #[wasm_bindgen(method)]
    pub fn normalize(this: &Vector2) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn angle(this: &Vector2) -> f32;

    #[wasm_bindgen(method, js_name = "angleTo")]
    pub fn angle_to(this: &Vector2, v: &Vector2) -> f32;

    #[wasm_bindgen(method, js_name = "distanceTo")]
    pub fn distance_to(this: &Vector2, v: &Vector2) -> f32;

    #[wasm_bindgen(method, js_name = "distanceToSquared")]
    pub fn distance_to_squared(this: &Vector2, v: &Vector2) -> f32;

    #[wasm_bindgen(method, js_name = "manhattanDistanceTo")]
    pub fn manhattan_distance_to(this: &Vector2, v: &Vector2) -> f32;

    #[wasm_bindgen(method, js_name = "setLength")]
    pub fn set_length(this: &Vector2, length: f32) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn lerp(this: &Vector2, v: &Vector2, alpha: f32) -> Vector2;

    #[wasm_bindgen(method, js_name = "lerpVectors")]
    pub fn lerp_vectors(this: &Vector2, v1: &Vector2, v2: &Vector2, alpha: f32) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Vector2, v: Vector2) -> bool;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &Vector2, array: &Array) -> Vector2;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &Vector2, array: &Array, offset: i32) -> Array;

    #[wasm_bindgen(method, js_name = "fromBufferAttribute")]
    pub fn from_buffer_attribute(
        this: &Vector2,
        attribute: &BufferAttribute,
        index: i32,
    ) -> Vector2;

    #[wasm_bindgen(method, js_name = "rotateAround")]
    pub fn rotate_around(this: &Vector2, center: &Vector2, angle: f32) -> Vector2;

    #[wasm_bindgen(method)]
    pub fn random(this: &Vector2) -> Vector2;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type SphericalHarmonics3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> SphericalHarmonics3;

    #[wasm_bindgen(method)]
    pub fn set(this: &SphericalHarmonics3, coefficients: &Array) -> SphericalHarmonics3;

    #[wasm_bindgen(method)]
    pub fn zero(this: &SphericalHarmonics3) -> SphericalHarmonics3;

    #[wasm_bindgen(method, js_name = "getAt")]
    pub fn get_at(this: &SphericalHarmonics3, normal: &Vector3, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "getIrradianceAt")]
    pub fn get_irradiance_at(
        this: &SphericalHarmonics3,
        normal: &Vector3,
        target: &Vector3,
    ) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn add(this: &SphericalHarmonics3, sh: &SphericalHarmonics3) -> SphericalHarmonics3;

    #[wasm_bindgen(method, js_name = "addScaledSH")]
    pub fn add_scaled_sh(
        this: &SphericalHarmonics3,
        sh: &SphericalHarmonics3,
        s: &Number,
    ) -> SphericalHarmonics3;

    #[wasm_bindgen(method)]
    pub fn scale(this: &SphericalHarmonics3, s: Number) -> SphericalHarmonics3;

    #[wasm_bindgen(method)]
    pub fn lerp(this: &SphericalHarmonics3, sh: &JsValue, alpha: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn equals(this: &SphericalHarmonics3, sh: &SphericalHarmonics3) -> bool;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &SphericalHarmonics3, array: &Array, offset: Number) -> JsValue;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &SphericalHarmonics3, array: &JsValue, offset: i32) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Triangle;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Triangle;

    #[wasm_bindgen(constructor)]
    pub fn new_with(a: Vector3, b: Vector3, c: Vector3) -> Triangle;

    #[wasm_bindgen(method)]
    pub fn set(this: &Triangle, a: Vector3, b: Vector3, c: Vector3) -> Triangle;

    #[wasm_bindgen(method, js_name = "setFromPointsAndIndices")]
    pub fn set_from_points_and_indices(
        this: &Triangle,
        points: &JsValue,
        i0: &JsValue,
        i1: &JsValue,
        i2: &JsValue,
    ) -> Triangle;

    #[wasm_bindgen(method, js_name = "setFromAttributeAndIndices")]
    pub fn set_from_attribute_and_indices(
        this: &Triangle,
        points: &JsValue,
        i0: &JsValue,
        i1: &JsValue,
        i2: &JsValue,
    ) -> Triangle;

    #[wasm_bindgen(method, js_name = "getArea")]
    pub fn get_area(this: &Triangle) -> f64;

    #[wasm_bindgen(method, js_name = "getMidpoint")]
    pub fn get_midpoint(this: &Triangle, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "getNormal")]
    pub fn get_normal(this: &Triangle, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "getPlane")]
    pub fn get_plane(this: &Triangle, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "getBarycoord")]
    pub fn get_barycoord(this: &Triangle, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "getUV")]
    pub fn get_uv(
        this: &Triangle,
        point: &JsValue,
        uv1: &JsValue,
        uv2: &JsValue,
        uv3: &JsValue,
        target: &JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = "getInterpolation")]
    pub fn get_interpolation(
        this: &Triangle,
        point: &JsValue,
        uv1: &JsValue,
        uv2: &JsValue,
        uv3: &JsValue,
        target: &JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = "containsPoint")]
    pub fn contains_point(this: &Triangle, point: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "isFrontFacing")]
    pub fn is_front_facing(this: &Triangle, direction: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsBox")]
    pub fn intersects_box(this: &Triangle, r#box: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "closestPointToPoint")]
    pub fn closest_point_to_point(this: &Triangle, p: &JsValue, r#box: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Triangle, triangle: &Triangle) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Spherical;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Spherical;

    #[wasm_bindgen(constructor)]
    pub fn new_with(radius: f64, phi: f64, theta: f64) -> Spherical;

    #[wasm_bindgen(method)]
    pub fn set(this: &Spherical, radius: f64, phi: f64, theta: f64) -> Spherical;

    #[wasm_bindgen(method, js_name = "makeSafe")]
    pub fn make_safe(this: &Spherical) -> Spherical;

    #[wasm_bindgen(method, js_name = "setFromVector3")]
    pub fn set_from_vector3(this: &Spherical, v: &Vector3) -> Spherical;

    #[wasm_bindgen(method, js_name = "setFromCartesianCoords")]
    pub fn set_from_cartesian_coords(this: &Spherical, x: f64, y: f64, z: f64) -> Spherical;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Cylindrical;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Cylindrical;

    #[wasm_bindgen(constructor)]
    pub fn new_with(radius: f64, theta: f64, y: f64) -> Cylindrical;

    #[wasm_bindgen(method)]
    pub fn set(this: &Cylindrical, radius: f64, theta: f64, y: f64) -> Cylindrical;

    #[wasm_bindgen(method, js_name = "setFromVector3")]
    pub fn set_from_vector3(this: &Cylindrical, v: &Vector3) -> Cylindrical;

    #[wasm_bindgen(method, js_name = "setFromCartesianCoords")]
    pub fn set_from_cartesian_coords(this: &Cylindrical, x: f64, y: f64, z: f64) -> Cylindrical;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Plane;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Plane;

    #[wasm_bindgen(constructor)]
    pub fn new_with(normal: Vector3, constant: f64) -> Plane;

    #[wasm_bindgen(method)]
    pub fn set(this: &Plane, normal: Vector3, constant: f64) -> Plane;

    #[wasm_bindgen(method, js_name = "setComponents")]
    pub fn set_components(this: &Plane, x: f64, y: f64, z: f64, w: f64) -> Plane;

    #[wasm_bindgen(method, js_name = "setFromNormalAndCoplanarPoint")]
    pub fn set_from_normal_and_coplanar_point(
        this: &Plane,
        normal: Vector3,
        constant: f64,
    ) -> Plane;

    #[wasm_bindgen(method)]
    pub fn normalize(this: &Plane) -> Plane;

    #[wasm_bindgen(method)]
    pub fn negate(this: &Plane) -> Plane;

    #[wasm_bindgen(method, js_name = "distanceToPoint")]
    pub fn distance_to_point(this: &Plane, point: &JsValue) -> f64;

    #[wasm_bindgen(method, js_name = "distanceToSphere")]
    pub fn distance_to_sphere(this: &Plane, sphere: &JsValue) -> f64;

    #[wasm_bindgen(method, js_name = "projectPoint")]
    pub fn project_point(this: &Plane, point: &JsValue, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "intersectLine")]
    pub fn intersect_line(this: &Plane, line: &JsValue, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "intersectsLine")]
    pub fn intersects_line(this: &Plane, line: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsBox")]
    pub fn intersects_box(this: &Plane, r#box: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "intersectsSphere")]
    pub fn intersects_sphere(this: &Plane, sphere: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "coplanarPoint")]
    pub fn coplanar_point(this: &Plane, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix_4(this: &Plane, matrix: &JsValue, optionalNormalMatrix: &JsValue) -> Plane;

    pub fn translate(this: &Plane, offset: &JsValue) -> Plane;

    pub fn equals(this: &Plane, plane: &Plane) -> JsValue;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Frustum;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Frustum;

    #[wasm_bindgen(constructor)]
    pub fn new_with(
        p0: &Plane,
        p1: &Plane,
        p2: &Plane,
        p3: &Plane,
        p4: &Plane,
        p5: &Plane,
    ) -> Frustum;

    #[wasm_bindgen(constructor)]
    pub fn set(
        this: &Frustum,
        p0: &Plane,
        p1: &Plane,
        p2: &Plane,
        p3: &Plane,
        p4: &Plane,
        p5: &Plane,
    ) -> Frustum;

    #[wasm_bindgen(method, js_name = "setFromProjectionMatrix")]
    pub fn set_from_projection_matrix(this: &Frustum, m: &JsValue) -> Frustum;

    #[wasm_bindgen(method, js_name = "intersectsObject")]
    pub fn intersects_object(this: &Frustum, m: &Object3D) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsSprite")]
    pub fn intersects_sprite(this: &Frustum, m: &Sprite) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsSphere")]
    pub fn intersects_sphere(this: &Frustum, m: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsBox")]
    pub fn intersects_box(this: &Frustum, m: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "containsPoint")]
    pub fn contains_point(this: &Frustum, point: &Vector3) -> bool;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Sphere;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Sphere;

    #[wasm_bindgen(constructor)]
    pub fn new_with(center: &Vector3, radius: f32) -> Sphere;

    #[wasm_bindgen(method)]
    pub fn set(this: &Sphere, center: &Vector3, radius: f32) -> Sphere;

    #[wasm_bindgen(method, js_name = "setFromPoints")]
    pub fn set_from_points(this: &Sphere, points: Array, optionalCenter: &Vector3) -> Sphere;

    #[wasm_bindgen(method, js_name = "isEmpty")]
    pub fn is_empty(this: &Sphere) -> bool;

    #[wasm_bindgen(method, js_name = "makeEmpty")]
    pub fn make_empty(this: &Sphere) -> Sphere;

    #[wasm_bindgen(method, js_name = "containsPoint")]
    pub fn contains_point(this: &Sphere, point: &Vector3) -> bool;

    #[wasm_bindgen(method, js_name = "distanceToPoint")]
    pub fn distance_to_point(this: &Sphere, point: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "intersectsSphere")]
    pub fn intersects_sphere(this: &Sphere, sphere: &Sphere) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsBox")]
    pub fn intersects_box(this: &Sphere, r#box: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsPlane")]
    pub fn intersects_plane(this: &Sphere, r#box: &Plane) -> bool;

    #[wasm_bindgen(method, js_name = "clampPoint")]
    pub fn clamp_point(this: &Sphere, point: &Vector3, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "getBoundingBox")]
    pub fn get_bounding_box(this: &Sphere, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix4(this: &Sphere, matrix: &JsValue) -> Sphere;

    #[wasm_bindgen(method)]
    pub fn translate(this: &Sphere, offset: &Vector3) -> Sphere;

    #[wasm_bindgen(method, js_name = "expandByPoint")]
    pub fn expand_by_point(this: &Sphere, point: &Vector3) -> Sphere;

    #[wasm_bindgen(method)]
    pub fn union(this: &Sphere, sphere: &Sphere) -> Sphere;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Sphere, sphere: &Sphere) -> Sphere;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Ray;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Ray;

    #[wasm_bindgen(constructor)]
    pub fn new_with(origin: &Vector3, direction: &Vector3) -> Ray;

    #[wasm_bindgen(method)]
    pub fn set(this: &Ray, origin: &Vector3, direction: &Vector3) -> Ray;

    #[wasm_bindgen(method)]
    pub fn at(this: &Ray, t: f32, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "lookAt")]
    pub fn look_at(this: &Ray, v: &Vector3) -> Ray;

    #[wasm_bindgen(method)]
    pub fn recast(this: &Ray, t: f32) -> Ray;

    #[wasm_bindgen(method, js_name = "closestPointToPoint")]
    pub fn closest_point_to_point(this: &Ray, point: &Vector3, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "distanceToPoint")]
    pub fn distance_to_point(this: &Ray, point: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "distanceSqToPoint")]
    pub fn distance_sq_to_point(this: &Ray, point: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "distanceSqToSegment")]
    pub fn distance_sq_to_segment(
        this: &Ray,
        v0: &Vector3,
        v1: &Vector3,
        optionalPointOnRay: &Vector3,
        optionalPointOnSegment: &Vector3,
    ) -> f32;

    #[wasm_bindgen(method, js_name = "intersectSphere")]
    pub fn intersect_sphere(this: &Ray, sphere: &Sphere, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "intersectsSphere")]
    pub fn intersects_sphere(this: &Ray, sphere: &Sphere) -> bool;

    #[wasm_bindgen(method, js_name = "distanceToPlane")]
    pub fn distance_to_plane(this: &Ray, plane: &Plane) -> f32;

    #[wasm_bindgen(method, js_name = "intersectPlane")]
    pub fn intersect_plane(this: &Ray, plane: &Plane, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "intersectsPlane")]
    pub fn intersects_plane(this: &Ray, plane: &Plane) -> bool;

    #[wasm_bindgen(method, js_name = "intersectBox")]
    pub fn intersect_box(this: &Ray, plane: &JsValue, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "intersectsBox")]
    pub fn intersects_box(this: &Ray, plane: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "intersectTriangle")]
    pub fn intersect_triangle(
        this: &Ray,
        a: &Vector3,
        b: &Vector3,
        c: &Vector3,
        backfaceCulling: bool,
        target: &Vector3,
    ) -> Vector3;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix4(this: &Ray, matrix4: &JsValue) -> Ray;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Ray, ray: &Ray) -> Ray;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Matrix4;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn set(
        this: &Matrix4,
        n11: f32,
        n12: f32,
        n13: f32,
        n14: f32,
        n21: f32,
        n22: f32,
        n23: f32,
        n24: f32,
        n31: f32,
        n32: f32,
        n33: f32,
        n34: f32,
        n41: f32,
        n42: f32,
        n43: f32,
        n44: f32,
    ) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn identity(this: &Matrix4) -> Matrix4;

    #[wasm_bindgen(method, js_name = "copyPosition")]
    pub fn copy_position(this: &Matrix4, m: &Matrix4) -> Matrix4;

    #[wasm_bindgen(method, js_name = "setFromMatrix3")]
    pub fn set_from_matrix3(this: &Matrix4, m: &JsValue) -> Matrix4;

    #[wasm_bindgen(method, js_name = "extractBasis")]
    pub fn extract_basis(
        this: &Matrix4,
        x_axis: &Vector3,
        y_axis: &Vector3,
        z_axis: &Vector3,
    ) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeBasis")]
    pub fn make_basis(
        this: &Matrix4,
        x_axis: &Vector3,
        y_axis: &Vector3,
        z_axis: &Vector3,
    ) -> Matrix4;

    #[wasm_bindgen(method, js_name = "extractRotation")]
    pub fn extract_rotation(this: &Matrix4, m: &Matrix4) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeRotationFromEuler")]
    pub fn make_rotation_from_euler(this: &Matrix4, euler: &Euler) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeRotationFromQuaternion")]
    pub fn make_rotation_from_quaternion(this: &Matrix4, euler: &JsValue) -> Matrix4;

    #[wasm_bindgen(method, js_name = "lookAt")]
    pub fn look_at(this: &Matrix4, eye: &Vector3, target: &Vector3, up: &Vector3) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn multiply(this: &Matrix4, m: &Matrix4) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn premultiply(this: &Matrix4, m: &Matrix4) -> Matrix4;

    #[wasm_bindgen(method, js_name = "multiplyMatrices")]
    pub fn multiply_matrices(this: &Matrix4, a: &Matrix4, b: &Matrix4) -> Matrix4;

    #[wasm_bindgen(method, js_name = "multiplyScalar")]
    pub fn multiply_scalar(this: &Matrix4, s: f32) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn determinant(this: &Matrix4) -> f32;

    #[wasm_bindgen(method)]
    pub fn transpose(this: &Matrix4) -> Matrix4;

    #[wasm_bindgen(method, js_name = "setPosition")]
    pub fn set_position(this: &Matrix4, x: f32, y: f32, z: f32) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn invert(this: &Matrix4) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn scale(this: &Matrix4, v: &Vector3) -> Matrix4;

    #[wasm_bindgen(method, js_name = "getMaxScaleOnAxis")]
    pub fn get_max_scale_on_axis(this: &Matrix4) -> f32;

    #[wasm_bindgen(method, js_name = "makeTranslation")]
    pub fn make_translation(this: &Matrix4, x: f32, y: f32, z: f32) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeRotationX")]
    pub fn make_rotation_x(this: &Matrix4, theta: f32) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeRotationY")]
    pub fn make_rotation_y(this: &Matrix4, theta: f32) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeRotationZ")]
    pub fn make_rotation_z(this: &Matrix4, theta: f32) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeRotationAxis")]
    pub fn make_rotation_axis(this: &Matrix4, axis: f32, angle: f32) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeScale")]
    pub fn make_scale(this: &Matrix4, x: f32, y: f32, z: f32) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeShear")]
    pub fn make_shear(
        this: &Matrix4,
        xy: f32,
        xz: f32,
        yx: f32,
        yz: f32,
        zx: f32,
        zy: f32,
    ) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn compose(
        this: &Matrix4,
        position: &Vector3,
        quaternion: &JsValue,
        scale: Vector3,
    ) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn decompose(
        this: &Matrix4,
        position: &Vector3,
        quaternion: &JsValue,
        scale: Vector3,
    ) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makePerspective")]
    pub fn make_perspective(
        this: &Matrix4,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) -> Matrix4;

    #[wasm_bindgen(method, js_name = "makeOrthographic")]
    pub fn make_orthographic(
        this: &Matrix4,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) -> Matrix4;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Matrix4, matrix: &Matrix4) -> bool;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &Matrix4, array: Array, offset: i32) -> Matrix4;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &Matrix4, array: Array, offset: i32) -> bool;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Matrix3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn set(
        this: &Matrix3,
        n11: f32,
        n12: f32,
        n13: f32,
        n14: f32,
        n21: f32,
        n22: f32,
        n23: f32,
        n24: f32,
        n31: f32,
        n32: f32,
        n33: f32,
    ) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn identity(this: &Matrix3) -> Matrix3;

    #[wasm_bindgen(method, js_name = "setFromMatrix3")]
    pub fn set_from_matrix4(this: &Matrix3, m: &Matrix4) -> Matrix3;

    #[wasm_bindgen(method, js_name = "extractBasis")]
    pub fn extract_basis(
        this: &Matrix3,
        x_axis: &Vector3,
        y_axis: &Vector3,
        z_axis: &Vector3,
    ) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn multiply(this: &Matrix3, m: &Matrix3) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn premultiply(this: &Matrix3, m: &Matrix3) -> Matrix3;

    #[wasm_bindgen(method, js_name = "multiplyMatrices")]
    pub fn multiply_matrices(this: &Matrix3, a: &Matrix3, b: &Matrix3) -> Matrix3;

    #[wasm_bindgen(method, js_name = "multiplyScalar")]
    pub fn multiply_scalar(this: &Matrix3, s: f32) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn determinant(this: &Matrix3) -> f32;

    #[wasm_bindgen(method)]
    pub fn transpose(this: &Matrix3) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn invert(this: &Matrix3) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn scale(this: &Matrix3, sx: f32, sy: f32) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn rotate(this: &Matrix3, theta: f32) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn translate(this: &Matrix3, tx: f32, ty: f32) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn makeTranslation(this: &Matrix3, x: f32, y: f32) -> Matrix3;

    #[wasm_bindgen(method, js_name = "makeRotation")]
    pub fn make_rotation_z(this: &Matrix3, theta: f32) -> Matrix3;

    #[wasm_bindgen(method, js_name = "makeScale")]
    pub fn make_scale(this: &Matrix3, x: f32, y: f32) -> Matrix3;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Matrix3, matrix: &Matrix3) -> bool;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &Matrix3, array: Array, offset: i32) -> Matrix3;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &Matrix3, array: Array, offset: i32) -> bool;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Box3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Box3;

    #[wasm_bindgen(constructor)]
    pub fn new_with(min: &Vector3, max: &Vector3) -> Box3;

    #[wasm_bindgen(method)]
    pub fn set(this: &Box3, min: &Vector3, max: &Vector3) -> Box3;

    #[wasm_bindgen(method, js_name = "setFromArray")]
    pub fn set_from_array(this: &Box3, array: &Array) -> Box3;

    #[wasm_bindgen(method, js_name = "setFromBufferAttribute")]
    pub fn set_from_buffer_attribute(this: &Box3, attribute: &BufferAttribute) -> Box3;

    #[wasm_bindgen(method, js_name = "setFromPoints")]
    pub fn set_from_points(this: &Box3, points: &Array) -> Box3;

    #[wasm_bindgen(method, js_name = "setFromCenterAndSize")]
    pub fn set_from_center_and_size(this: &Box3, center: &Vector3, size: &Vector3) -> Box3;

    #[wasm_bindgen(method, js_name = "setFromObject")]
    pub fn set_from_object(this: &Box3, object: &Object3D, precise: bool) -> Box3;

    #[wasm_bindgen(method, js_name = "makeEmpty")]
    pub fn make_empty(this: &Box3) -> Box3;

    #[wasm_bindgen(method, js_name = "isEmpty")]
    pub fn is_empty(this: &Box3) -> bool;

    #[wasm_bindgen(method, js_name = "getCenter")]
    pub fn get_center(this: &Box3, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "getSize")]
    pub fn get_size(this: &Box3, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "expandByPoint")]
    pub fn expand_by_point(this: &Box3, point: &Vector3) -> Box3;

    #[wasm_bindgen(method, js_name = "expandByVector")]
    pub fn expand_by_vector(this: &Box3, point: &Vector3) -> Box3;

    #[wasm_bindgen(method, js_name = "expandByScalar")]
    pub fn expand_by_scalar(this: &Box3, scalar: f32) -> Box3;

    #[wasm_bindgen(method, js_name = "expandByObject")]
    pub fn expand_by_object(this: &Box3, object: Object3D, precise: bool) -> Box3;

    #[wasm_bindgen(method, js_name = "containsPoint")]
    pub fn contains_point(this: &Box3, point: &Vector3) -> bool;

    #[wasm_bindgen(method, js_name = "containsBox")]
    pub fn contains_box(this: &Box3, r#box: &Box3) -> bool;

    #[wasm_bindgen(method, js_name = "getParameter")]
    pub fn get_parameter(this: &Box3, point: &Vector3, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "intersectsBox")]
    pub fn intersects_box(this: &Box3, r#box: &Box3) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsSphere")]
    pub fn intersects_sphere(this: &Box3, sphere: &Sphere) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsPlane")]
    pub fn intersects_plane(this: &Box3, sphere: &Sphere) -> bool;

    #[wasm_bindgen(method, js_name = "intersectsTriangle")]
    pub fn intersects_triangle(this: &Box3, triangle: &Triangle) -> bool;

    #[wasm_bindgen(method, js_name = "clampPoint")]
    pub fn clamp_point(this: &Box3, point: &Vector3, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method, js_name = "distanceToPoint")]
    pub fn distance_to_point(this: &Box3, point: &Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "getBoundingSphere")]
    pub fn get_bounding_sphere(this: &Box3, target: &Sphere) -> Sphere;

    #[wasm_bindgen(method)]
    pub fn intersect(this: &Box3, r#box: &Box3) -> Box3;

    #[wasm_bindgen(method)]
    pub fn union(this: &Box3, r#box: &Box3) -> Box3;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix4(this: &Box3, matrix: &Matrix4) -> Box3;

    #[wasm_bindgen(method)]
    pub fn translate(this: &Box3, offset: &Vector3) -> Box3;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Box3, r#box: &Box3) -> bool;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Box2;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Box2;

    #[wasm_bindgen(constructor)]
    pub fn new_with(min: &JsValue, max: &JsValue) -> Box2;

    #[wasm_bindgen(method)]
    pub fn set(this: &Box2, min: &JsValue, max: &JsValue) -> Box2;

    #[wasm_bindgen(method, js_name = "setFromPoints")]
    pub fn set_from_points(this: &Box2, points: &Array) -> Box2;

    #[wasm_bindgen(method, js_name = "setFromCenterAndSize")]
    pub fn set_from_center_and_size(this: &Box2, center: &JsValue, size: &JsValue) -> Box2;

    #[wasm_bindgen(method, js_name = "makeEmpty")]
    pub fn make_empty(this: &Box2) -> Box2;

    #[wasm_bindgen(method, js_name = "isEmpty")]
    pub fn is_empty(this: &Box2) -> bool;

    #[wasm_bindgen(method, js_name = "getCenter")]
    pub fn get_center(this: &Box2, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "getSize")]
    pub fn get_size(this: &Box2, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "expandByPoint")]
    pub fn expand_by_point(this: &Box2, point: &JsValue) -> Box2;

    #[wasm_bindgen(method, js_name = "expandByVector")]
    pub fn expand_by_vector(this: &Box2, point: &JsValue) -> Box2;

    #[wasm_bindgen(method, js_name = "expandByScalar")]
    pub fn expand_by_scalar(this: &Box2, scalar: f32) -> Box2;

    #[wasm_bindgen(method, js_name = "containsPoint")]
    pub fn contains_point(this: &Box2, point: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = "containsBox")]
    pub fn contains_box(this: &Box2, r#box: &Box2) -> bool;

    #[wasm_bindgen(method, js_name = "getParameter")]
    pub fn get_parameter(this: &Box2, point: &JsValue, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "intersectsBox")]
    pub fn intersects_box(this: &Box2, r#box: &Box2) -> bool;

    #[wasm_bindgen(method, js_name = "clampPoint")]
    pub fn clamp_point(this: &Box2, point: &JsValue, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "distanceToPoint")]
    pub fn distance_to_point(this: &Box2, point: &JsValue) -> f32;

    #[wasm_bindgen(method)]
    pub fn intersect(this: &Box2, r#box: &Box2) -> Box2;

    #[wasm_bindgen(method)]
    pub fn union(this: &Box2, r#box: &Box2) -> Box2;

    #[wasm_bindgen(method)]
    pub fn translate(this: &Box2, offset: &JsValue) -> Box2;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Box2, r#box: &Box2) -> bool;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Line3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Line3;

    #[wasm_bindgen(constructor)]
    pub fn new_with(start: &Vector3, max: &Vector3) -> Line3;

    #[wasm_bindgen(method)]
    pub fn set(this: &Line3, start: &Vector3, end: &Vector3) -> Line3;

    #[wasm_bindgen(method)]
    pub fn delta(this: &Line3, target: &Vector3) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn distance_sq(this: &Line3) -> f32;

    #[wasm_bindgen(method)]
    pub fn distance(this: &Line3) -> f32;

    #[wasm_bindgen(method)]
    pub fn at(this: &Line3, t: f32, target: Vector3) -> f32;

    #[wasm_bindgen(method, js_name = "closestPointToPointParameter")]
    pub fn closest_point_to_point_parameter(this: &Line3, point: Vector3, clampToLine: bool)
        -> f32;

    #[wasm_bindgen(method, js_name = "closestPointToPoint")]
    pub fn closest_point_to_point(
        this: &Line3,
        point: &Vector3,
        clampToLine: bool,
        target: &Vector3,
    ) -> f32;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix4(this: &Line3, matrix: &Matrix4) -> Line3;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Line3, line: &Line3) -> bool;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Vector4;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Vector4;

    #[wasm_bindgen(constructor)]
    pub fn new_with(x: f32, y: f32, z: f32, w: f32) -> Vector4;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &Vector4);

    #[wasm_bindgen(method, setter)]
    pub fn set_width(this: &Vector4, value: f32);

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &Vector4) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_height(this: &Vector4, value: f32);

    #[wasm_bindgen(method)]
    pub fn set(this: &Vector4, x: f32, y: f32, z: f32, w: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "setScalar")]
    pub fn set_scalar(this: &Vector4, scalar: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "setX")]
    pub fn set_x(this: &Vector4, x: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "setY")]
    pub fn set_y(this: &Vector4, y: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "setZ")]
    pub fn set_z(this: &Vector4, z: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "setW")]
    pub fn set_w(this: &Vector4, z: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "setComponent")]
    pub fn set_component(this: &Vector4, index: i32, value: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "getComponent")]
    pub fn get_component(this: &Vector4, index: i32) -> f32;

    #[wasm_bindgen(method)]
    pub fn add(this: &Vector4, v: &Vector4) -> Vector4;

    #[wasm_bindgen(method, js_name = "addScalar")]
    pub fn add_scalar(this: &Vector4, s: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "addVectors")]
    pub fn addVectors(this: &Vector4, a: &Vector4, b: &Vector4) -> Vector4;

    #[wasm_bindgen(method, js_name = "addScaledVector")]
    pub fn addScaledVector(this: &Vector4, v: &Vector4, s: f32) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn sub(this: &Vector4, v: &Vector4) -> Vector4;

    #[wasm_bindgen(method, js_name = "subScalar")]
    pub fn sub_scalar(this: &Vector4, s: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "subVectors")]
    pub fn sub_vectors(this: &Vector4, a: &Vector4, b: &Vector4) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn multiply(this: &Vector4, v: &Vector4) -> Vector4;

    #[wasm_bindgen(method, js_name = "multiplyScalar")]
    pub fn multiply_scalar(this: &Vector4, scalar: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "applyMatrix4")]
    pub fn apply_matrix4(this: &Vector4, m: &Matrix4) -> Vector4;

    #[wasm_bindgen(method, js_name = "divideScalar")]
    pub fn divide_scalar(this: &Vector4, scalar: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "setAxisAngleFromQuaternion")]
    pub fn set_axis_angle_from_quaternion(this: &Vector4, q: &JsValue) -> Vector4;

    #[wasm_bindgen(method, js_name = "setAxisAngleFromRotationMatrix")]
    pub fn set_axis_angle_from_rotation_matrix(this: &Vector4, m: &Matrix4) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn min(this: &Vector4, v: &Vector4) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn max(this: &Vector4, v: &Vector4) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn clamp(this: &Vector4, min: &Vector4, max: &Vector4) -> Vector4;

    #[wasm_bindgen(method, js_name = "clampScalar")]
    pub fn clamp_scalar(this: &Vector4, min_val: f32, max_val: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "clampLength")]
    pub fn clamp_length(this: &Vector4, min: f32, max: f32) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn floor(this: &Vector4) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn ceil(this: &Vector4) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn round(this: &Vector4) -> Vector4;

    #[wasm_bindgen(method, js_name = "roundToZero")]
    pub fn round_to_zero(this: &Vector4) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn negate(this: &Vector4) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn dot(this: &Vector4, v: Vector4) -> f32;

    #[wasm_bindgen(method, js_name = "lengthSq")]
    pub fn length_sq(this: &Vector4) -> f32;

    #[wasm_bindgen(method)]
    pub fn length(this: &Vector4) -> f32;

    #[wasm_bindgen(method, js_name = "manhattanLength")]
    pub fn manhattan_length(this: &Vector4) -> f32;

    #[wasm_bindgen(method)]
    pub fn normalize(this: &Vector4) -> Vector4;

    #[wasm_bindgen(method, js_name = "setLength")]
    pub fn set_length(this: &Vector4, length: f32) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn lerp(this: &Vector4, v: &Vector4, alpha: f32) -> Vector4;

    #[wasm_bindgen(method, js_name = "lerpVectors")]
    pub fn lerp_vectors(this: &Vector4, v1: &Vector4, v2: &Vector4, alpha: f32) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Vector4, v: &Vector4) -> bool;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &Vector4, array: &Array) -> Vector4;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &Vector4, array: &Array, offset: i32) -> Array;

    #[wasm_bindgen(method, js_name = "fromBufferAttribute")]
    pub fn from_buffer_attribute(
        this: &Vector4,
        attribute: &BufferAttribute,
        index: i32,
    ) -> Vector4;

    #[wasm_bindgen(method)]
    pub fn random(this: &Vector4) -> Vector4;
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Quaternion;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Quaternion;

    #[wasm_bindgen(constructor)]
    pub fn new_with(x: f32, y: f32, z: f32, w: f32) -> Quaternion;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Quaternion) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Quaternion, value: f32);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Quaternion) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Quaternion, value: f32);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Quaternion) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Quaternion, value: f32);

    #[wasm_bindgen(method, getter)]
    pub fn w(this: &Quaternion) -> f32;

    #[wasm_bindgen(method, setter)]
    pub fn set_w(this: &Quaternion, value: f32);

    #[wasm_bindgen(method)]
    pub fn set(this: &Quaternion, x: f32, y: f32, z: f32, w: f32) -> Quaternion;

    #[wasm_bindgen(method, js_name = "setFromEuler")]
    pub fn set_from_euler(this: &Quaternion, euler: &Euler, update: bool) -> Quaternion;

    #[wasm_bindgen(method, js_name = "setFromAxisAngle")]
    pub fn set_from_axis_angle(this: &Quaternion, axis: &Vector3, angle: f32) -> Quaternion;

    #[wasm_bindgen(method, js_name = "setFromRotationMatrix")]
    pub fn set_from_rotation_matrix(this: &Quaternion, m: &Matrix4) -> Quaternion;

    #[wasm_bindgen(method, js_name = "setFromUnitVectors")]
    pub fn set_from_unit_vectors(this: &Quaternion, from: &Vector3, to: &Vector3) -> Quaternion;

    #[wasm_bindgen(method, js_name = "angleTo")]
    pub fn angle_to(this: &Quaternion, q: &Quaternion) -> f32;

    #[wasm_bindgen(method, js_name = "rotateTowards")]
    pub fn rotate_towards(this: &Quaternion, q: &Quaternion, step: f32) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn identity(this: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn invert(this: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn conjugate(this: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn dot(this: &Quaternion, q: Quaternion) -> f32;

    #[wasm_bindgen(method, js_name = "lengthSq")]
    pub fn length_sq(this: &Quaternion) -> f32;

    #[wasm_bindgen(method)]
    pub fn length(this: &Quaternion) -> f32;

    #[wasm_bindgen(method)]
    pub fn normalize(this: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn multiply(this: &Quaternion, q: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn premultiply(this: &Quaternion, q: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method, js_name = "multiplyQuaternions")]
    pub fn multiply_quaternions(this: &Quaternion, a: &Quaternion, b: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn slerp(this: &Quaternion, qb: &Quaternion, t: f32) -> Quaternion;

    #[wasm_bindgen(method, js_name = "slerpQuaternions")]
    pub fn slerp_quaternions(
        this: &Quaternion,
        qa: &Quaternion,
        qb: &Quaternion,
        t: f32,
    ) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn random(this: &Quaternion) -> Quaternion;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Quaternion, quaternion: &Quaternion) -> bool;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &Quaternion, array: &Array) -> Quaternion;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &Quaternion, array: &Array, offset: i32) -> Array;

    #[wasm_bindgen(method, js_name = "fromBufferAttribute")]
    pub fn from_buffer_attribute(
        this: &Quaternion,
        attribute: &BufferAttribute,
        index: i32,
    ) -> Quaternion;

}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Color;

    #[wasm_bindgen(constructor)]
    pub fn new(r: f32, g: f32, b: f32) -> Color;

    #[wasm_bindgen(method)]
    pub fn set(this: &Color, value: &str) -> Color;

    #[wasm_bindgen(method, js_name = "setScalar")]
    pub fn set_scalar(this: &Color, scalar: f32) -> Color;

    #[wasm_bindgen(method, js_name = "setHex")]
    pub fn set_hex(this: &Color, hex: i32, color_space: &str) -> Color;

    #[wasm_bindgen(method, js_name = "setRGB")]
    pub fn set_rgb(this: &Color, r: f32, g: f32, b: f32, color_space: &str) -> Color;

    #[wasm_bindgen(method, js_name = "setHSL")]
    pub fn set_hsl(this: &Color, h: f32, s: f32, l: f32, color_space: &str) -> Color;

    #[wasm_bindgen(method, js_name = "setStyle")]
    pub fn set_style(this: &Color, style: &str, color_space: &str) -> Color;

    #[wasm_bindgen(method, js_name = "setColorName")]
    pub fn set_color_name(this: &Color, style: &str, color_space: &str) -> Color;

    #[wasm_bindgen(method, js_name = "copySRGBToLinear")]
    pub fn copy_srgb_to_linear(this: &Color, color: &Color) -> Color;

    #[wasm_bindgen(method, js_name = "copyLinearToSRGB")]
    pub fn copy_linear_to_srgb(this: &Color, color: &Color) -> Color;

    #[wasm_bindgen(method, js_name = "convertSRGBToLinear")]
    pub fn convert_srgb_to_linear(this: &Color) -> Color;

    #[wasm_bindgen(method, js_name = "convertLinearToSRGB")]
    pub fn convert_linear_to_srgb(this: &Color) -> Color;

    #[wasm_bindgen(method, js_name = "getHex")]
    pub fn get_hex(this: &Color, color_space: &str) -> i32;

    #[wasm_bindgen(method, js_name = "getHexString")]
    pub fn get_hex_string(this: &Color, color_space: &str) -> String;

    #[wasm_bindgen(method, js_name = "getHSL")]
    pub fn get_hsl(this: &Color, target: &Object, color_space: &str) -> Object;

    #[wasm_bindgen(method, js_name = "getRGB")]
    pub fn get_rgb(this: &Color, target: &Color, color_space: &str) -> Color;

    #[wasm_bindgen(method, js_name = "getStyle")]
    pub fn get_style(this: &Color, color_space: &str) -> String;

    #[wasm_bindgen(method, js_name = "offsetHSL")]
    pub fn offset_hsl(this: &Color, h: f32, s: f32, l: f32) -> Color;

    #[wasm_bindgen(method)]
    pub fn add(this: &Color, color: &Color) -> Color;

    #[wasm_bindgen(method, js_name = "addColors")]
    pub fn add_colors(this: &Color, color: &Color) -> Color;

    #[wasm_bindgen(method, js_name = "addScalar")]
    pub fn add_scalar(this: &Color, color: &Color) -> Color;

    #[wasm_bindgen(method)]
    pub fn sub(this: &Color, color: &Color) -> Color;

    #[wasm_bindgen(method)]
    pub fn multiply(this: &Color, color: &Color) -> Color;

    #[wasm_bindgen(method, js_name = "multiplyScalar")]
    pub fn multiply_scalar(this: &Color, s: f32) -> Color;

    #[wasm_bindgen(method)]
    pub fn lerp(this: &Color, color: &Color, alpha: f32) -> Color;

    #[wasm_bindgen(method, js_name = "lerpColors")]
    pub fn lerp_colors(this: &Color, color1: &Color, color2: &Color, alpha: f32) -> Color;

    #[wasm_bindgen(method, js_name = "lerpHSL")]
    pub fn lerp_hsl(this: &Color, color: &Color, alpha: f32) -> Color;

    #[wasm_bindgen(method, js_name = "setFromVector3")]
    pub fn set_from_vector3(this: &Color, v: &Vector3) -> Color;

    #[wasm_bindgen(method, js_name = "applyMatrix3")]
    pub fn apply_matrix3(this: &Color, m: &Matrix3) -> Color;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Color, c: &Color) -> Color;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &Color, array: &Array) -> Color;

    #[wasm_bindgen(method, js_name = "toArray")]
    pub fn to_array(this: &Color, array: &Array, offset: i32) -> Array;

    #[wasm_bindgen(method, js_name = "fromBufferAttribute")]
    pub fn from_buffer_attribute(this: &Color, attribute: &BufferAttribute, index: i32) -> Color;

    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Color) -> Number;

}
