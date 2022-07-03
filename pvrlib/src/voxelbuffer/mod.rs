pub mod dense;
pub mod sparse;

use crate::math::vec3::Vec3;
use crate::math::aabb::AABB;
use crate::math::ray::Ray;

use std::marker::Sync;

// #todo-refactor: VoxelBuffer - float or Vec3?
pub trait VoxelBuffer : Sync {
	fn sample_by_local_position(&self, u: f32, v: f32, w: f32) -> Vec3;
	fn sample_by_world_position(&self, p: Vec3) -> Vec3;

	fn world_to_voxel(&self, p: Vec3) -> Vec3;
	fn voxel_to_world(&self, p: Vec3) -> Vec3;

	fn get_size(&self) -> (i32, i32, i32);
	fn get_sizef(&self) -> Vec3;

	// #todo-refactor: Remove this and use VoxelVolume::world_bounds
	fn get_ws_bounds(&self) -> AABB;

	// List of (t_min, t_max) of the ray
	fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)>;

	// [0.0, 1.0] How many voxels have been materialized?
	fn get_occupancy(&self) -> f32;

	fn read(&self, i: i32, j: i32, k: i32) -> Vec3;
	fn write(&mut self, i: i32, j: i32, k: i32, value: Vec3);
}
