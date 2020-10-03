use crate::voxel::VoxelBuffer;

pub trait Primitive {
    fn rasterize(&self, voxel_buffer: &mut VoxelBuffer);
}
