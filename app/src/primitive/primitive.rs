use pvrlib::voxelbuffer::VoxelBuffer;

// #todo: Needs Geometry trait
pub trait Primitive {
	//
}

pub trait RasterizationPrimitive : Primitive {
    fn rasterize(&self, voxel_buffer: &mut dyn VoxelBuffer);
}
