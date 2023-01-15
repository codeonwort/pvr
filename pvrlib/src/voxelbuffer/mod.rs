pub mod dense;
pub mod sparse;

use crate::math::vec3::vec3f;
use crate::math::ray::Ray;
use crate::math::aabb::AABB;

use std::marker::Sync;

// #wip: Remove VoxelBuffer?
//
// I should define my own struct and implement `Volume` trait
// using `DenseField` or `SparseField` as I want.
// 
// Example) Custom volume based on density voxel buffer:
// DensityVoxelVolume {
//     densityField: SparseField<f32>,
//     emission_coeff: vec3f,   // constant, multiplied by density
//     absorption_coeff: vec3f, // constant, multiplied by density
//     scattering_coeff: vec3f  // constant, multiplied by density
// }

pub trait VoxelBuffer<T> : Sync {
	// Sample by uniform coordinates (0.0 <= u, v, w <= 1.0)
	// Use read() to sample by raw coordinates.
	fn sample_by_local_position(&self, u: f32, v: f32, w: f32) -> T;

	fn get_size(&self) -> (i32, i32, i32);
	fn get_sizef(&self) -> vec3f;

	// List of (t_min, t_max) of the ray
	fn find_intersections(&self, ray: Ray, world_bounds: AABB) -> Vec<(f32, f32)>;

	// [0.0, 1.0] How many voxels have been materialized?
	fn get_occupancy(&self) -> f32;

	fn read(&self, i: i32, j: i32, k: i32) -> T;

	fn write(&mut self, i: i32, j: i32, k: i32, value: T);

	// #todo-voxel: Anti-aliasing
	//fn read_aa(&self, uvw: Vec3) -> Vec3;
	//fn write_aa(&mut self, uvw: Vec3, value: Vec3);
}
