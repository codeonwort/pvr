pub mod rast;

use crate::volume::voxel::VoxelVolume;

// #wip
/*
/// Rasterization primitives calculate **density: f32** for positions to be rasterized.<br/>
/// `DensityMapper` maps that density to an actual value which is written to the voxel buffer.
type DensityMapper<T> = fn (density: f32) -> T;
pub fn density_mapper_identity(density: f32) -> f32 { density }
*/

// #todo: Needs Geometry trait
pub trait Primitive {
	//
}

pub trait RasterizationPrimitive : Primitive {
    fn rasterize(&self, voxel_buffer: &mut VoxelVolume);
}
