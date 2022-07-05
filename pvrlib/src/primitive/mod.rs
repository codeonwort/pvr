pub mod rast;

use crate::volume::voxel::VoxelVolume;

// #todo: Needs Geometry trait
pub trait Primitive {
	//
}

pub trait RasterizationPrimitive : Primitive {
    fn rasterize(&self, voxel_buffer: &mut VoxelVolume);
}
