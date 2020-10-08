use crate::voxel::voxel::VoxelBuffer;

pub trait Primitive {
    fn rasterize(&self, voxel_buffer: &mut dyn VoxelBuffer);
}
