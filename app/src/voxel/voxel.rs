use pvrlib::math::vec3::*;
use crate::aabb::*;
use std::marker::Sync;

pub trait VoxelBuffer : Sync {
	fn sample_by_local_position(&self, u: f32, v: f32, w: f32) -> Vec3;
	fn sample_by_world_position(&self, p: Vec3) -> Vec3;

	fn world_to_voxel(&self, p: Vec3) -> Vec3;
	fn voxel_to_world(&self, p: Vec3) -> Vec3;

	fn get_size(&self) -> (i32, i32, i32);
	fn get_sizef(&self) -> Vec3;

	// #todo: Shouldn't this be in VoxelVolume?
	fn get_ws_bounds(&self) -> AABB;

	fn get_occupancy(&self) -> f32; // (min=0.0, max=1.0) How many voxels have been materialized?

	fn read(&self, i: i32, j: i32, k: i32) -> Vec3;
	fn write(&mut self, i: i32, j: i32, k: i32, value: Vec3);
}

