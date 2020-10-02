use crate::voxel::VoxelBuffer;
use crate::aabb::AABB;

pub trait Primitive {
    fn rasterize(&self, voxel_buffer: &mut VoxelBuffer);
}
