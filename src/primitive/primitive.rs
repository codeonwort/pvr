use crate::voxel::voxel::VoxelBuffer;

pub trait Primitive {
	//
}

pub trait RasterizationPrimitive : Primitive {
    fn rasterize(&self, voxel_buffer: &mut dyn VoxelBuffer);
}
