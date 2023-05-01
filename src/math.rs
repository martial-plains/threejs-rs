use wasm_bindgen::prelude::*;

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
}

#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type Vector3;

    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;

    #[wasm_bindgen(method)]
    pub fn set(this: &Vector3, x: f64, y: f64, z: f64);

    #[wasm_bindgen(method)]
    pub fn setScalar(this: &Vector3, scalar: f64);

    #[wasm_bindgen(method)]
    pub fn setX(this: &Vector3, x: f64);

    #[wasm_bindgen(method)]
    pub fn setY(this: &Vector3, y: f64);

    #[wasm_bindgen(method)]
    pub fn setZ(this: &Vector3, z: f64);
}
#[wasm_bindgen(module = "/node_modules/three/build/three.module.js")]
extern "C" {
    pub type SphericalHarmonics3;

    #[wasm_bindgen(constructor)]
    pub fn new() -> SphericalHarmonics3;

    #[wasm_bindgen(method)]
    pub fn set(this: &SphericalHarmonics3, coefficients: &JsValue) -> SphericalHarmonics3;

    #[wasm_bindgen(method)]
    pub fn zero(this: &SphericalHarmonics3) -> SphericalHarmonics3;

    #[wasm_bindgen(method, js_name = "getAt")]
    pub fn get_at(this: &SphericalHarmonics3, normal: &JsValue, target: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "getIrradianceAt")]
    pub fn get_irradiance_at(
        this: &SphericalHarmonics3,
        normal: &JsValue,
        target: &JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn add(this: &SphericalHarmonics3, sh: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "addScaledSH")]
    pub fn add_scaled_sh(this: &SphericalHarmonics3, sh: &JsValue, s: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn scale(this: &SphericalHarmonics3, s: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn lerp(this: &SphericalHarmonics3, sh: &JsValue, alpha: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn equals(this: &SphericalHarmonics3, sh: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = "fromArray")]
    pub fn from_array(this: &SphericalHarmonics3, array: &JsValue, offset: i32) -> JsValue;

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

    #[wasm_bindgen(method, js_name = "translate")]
    pub fn translate(this: &Plane, offset: &JsValue) -> Plane;

    #[wasm_bindgen(method, js_name = "equals")]
    pub fn equals(this: &Plane, plane: &Plane) -> JsValue;
}
