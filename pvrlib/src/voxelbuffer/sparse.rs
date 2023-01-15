use std::ops::*;
use super::VoxelBuffer;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::math::aabb::*;

#[allow(non_camel_case_types)]
type usize3 = (usize, usize, usize);

const BLOCK_SIZE: usize3 = (32, 32, 32);
const BLOCK_ITEM_COUNT: usize = BLOCK_SIZE.0 * BLOCK_SIZE.1 * BLOCK_SIZE.2;

fn get_block_coord(coord: usize3) -> usize3 {
    (coord.0 / BLOCK_SIZE.0,
    coord.1 / BLOCK_SIZE.1,
    coord.2 / BLOCK_SIZE.2)
}

fn get_block_local_coord(coord: usize3, block_coord: usize3) -> usize3 {
    let x = coord.0 - block_coord.0 * BLOCK_SIZE.0;
    let y = coord.1 - block_coord.1 * BLOCK_SIZE.1;
    let z = coord.2 - block_coord.2 * BLOCK_SIZE.2;

    (x, y, z)
}

fn get_block_local_index(local_coord: usize3) -> usize {
    (local_coord.2 * BLOCK_SIZE.0 * BLOCK_SIZE.1) + (local_coord.1 * BLOCK_SIZE.0) + local_coord.0
}

fn to_usize3(i: i32, j: i32, k: i32) -> usize3 {
    (i as usize, j as usize, k as usize)
}

// ceil(float(x) / float(y))
fn div_ceil(x: usize, y: usize) -> usize {
    x / y + ((x % y != 0) as usize)
}

struct SparseFieldBlock<T> {
    data: Vec<T>
}

pub struct SparseField<T> {
    size: usize3,
    block_count: usize3,
    blocks: Vec<SparseFieldBlock<T>>,
    default_value: T
}

impl<T: Clone + Copy> SparseFieldBlock<T> {

    pub fn new() -> Self {
        SparseFieldBlock {
            data: Vec::new()
        }
    }

    pub fn allocate(&mut self, length: usize, value: T) {
        self.data.resize(length, value);
    }

    pub fn is_allocated(&self) -> bool {
        self.data.len() > 0
    }

    pub fn set(&mut self, ix: usize, value: T) {
        self.data[ix] = value;
    }

    pub fn get(&self, ix: usize) -> T { self.data[ix] }

}

impl<T: Clone + Copy> SparseField<T> {

    pub fn new(size: usize3, default_value: T) -> Self {
        let block_count: usize3 = (
            div_ceil(size.0, BLOCK_SIZE.0),
            div_ceil(size.1, BLOCK_SIZE.1),
            div_ceil(size.2, BLOCK_SIZE.2));

        let num_blocks = block_count.0 * block_count.1 * block_count.2;

        let mut blocks = Vec::with_capacity(num_blocks);
        for _ in 0..num_blocks {
            blocks.push(SparseFieldBlock::new());
        }
        
        SparseField {
            size,
            block_count,
            blocks,
            default_value
        }
    }

    pub fn sparse_block_occupancy(&self) -> f32 {
        let num_blocks = self.block_count.0 * self.block_count.1 * self.block_count.2;
        let mut num_alloc: usize = 0;
        for i in 0..num_blocks {
            if self.blocks[i].is_allocated() {
                num_alloc += 1;
            }
        }

        (num_alloc as f32) / (num_blocks as f32)
    }

    pub fn read_safe(&self, coord: usize3) -> Option<T> {
        if self.out_of_index(coord) {
            None
        } else {
            Some(self.read_raw(coord))
        }
    }

    pub fn write_safe(&mut self, coord: usize3, new_value: T) {
        if self.out_of_index(coord) == false {
            self.write_raw(coord, new_value);
        }
    }

    pub fn read_raw(&self, coord: usize3) -> T {
        if self.is_block_allocated(coord) == false {
            return self.default_value;
        }

        let block_coord = get_block_coord(coord);
        let block_index = self.get_block_index(block_coord);
        let local_coord = get_block_local_coord(coord, block_coord);
        let local_index = get_block_local_index(local_coord);
        return self.blocks[block_index].get(local_index);
    }

    pub fn write_raw(&mut self, coord: usize3, new_value: T) {
        let block_coord = get_block_coord(coord);
        let block_index = self.get_block_index(block_coord);
        let local_coord = get_block_local_coord(coord, block_coord);
        let local_index = get_block_local_index(local_coord);

        if self.is_block_allocated(coord) == false {
            self.blocks[block_index].allocate(BLOCK_ITEM_COUNT, self.default_value);
        }
        self.blocks[block_index].set(local_index, new_value);
    }

    fn out_of_index(&self, coord: usize3) -> bool {
        coord.0 >= self.size.0 || coord.1 >= self.size.1 || coord.2 >= self.size.2
    }

    fn get_cell_sizef(&self) -> vec3f {
        let x = (self.size.0 as f32) / (self.block_count.0 as f32);
        let y = (self.size.1 as f32) / (self.block_count.1 as f32);
        let z = (self.size.2 as f32) / (self.block_count.2 as f32);
        vec3(x, y, z)
    }

    fn get_block_index(&self, block_coord: usize3) -> usize {
        let size_xy = self.block_count.0 * self.block_count.1;

        (block_coord.2 * size_xy) + (block_coord.1 * self.block_count.0) + block_coord.0
    }

    fn is_block_allocated(&self, coord: usize3) -> bool {
        let block_coord = get_block_coord(coord);
        let block_index = self.get_block_index(block_coord);

        self.blocks[block_index].is_allocated()
    }

}

impl<T> VoxelBuffer<T> for SparseField<T>
    where T: Sync + Copy + Add<Output=T> + Mul<f32, Output=T>
{
    fn sample_by_local_position(&self, u: f32, v: f32, w: f32) -> T {
        if u < 0.0 || v < 0.0 || w < 0.0 || u >= 1.0 || v >= 1.0 || w >= 1.0 {
			self.default_value
		} else {
            let fx = 0.5 + u * (self.size.0 as f32);
			let fy = 0.5 + v * (self.size.1 as f32);
			let fz = 0.5 + w * (self.size.2 as f32);
			let f = vec3(fx, fy, fz);
			let a = f - f.floor();

			let read_raw = |vf: vec3f| -> T {
                let coord = (vf.x as usize, vf.y as usize, vf.z as usize);
                self.read_raw(coord)
			};

			let v000 = read_raw(f);
			let v001 = read_raw(f + vec3(0.0, 0.0, 1.0));
			let v010 = read_raw(f + vec3(0.0, 1.0, 0.0));
			let v011 = read_raw(f + vec3(0.0, 1.0, 1.0));
			let v100 = read_raw(f + vec3(1.0, 0.0, 0.0));
			let v101 = read_raw(f + vec3(1.0, 0.0, 1.0));
			let v110 = read_raw(f + vec3(1.0, 1.0, 0.0));
			let v111 = read_raw(f + vec3(1.0, 1.0, 1.0));

			let front = lerp(lerp(v000, v100, a.x), lerp(v010, v110, a.x), a.y);
			let back = lerp(lerp(v001, v101, a.x), lerp(v011, v111, a.x), a.y);
			let final_value = lerp(front, back, a.z);

			final_value
        }
    }

    fn get_size(&self) -> (i32, i32, i32) {
        (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32)
    }
    fn get_sizef(&self) -> vec3f {
        vec3(self.size.0 as f32, self.size.1 as f32, self.size.2 as f32)
    }

    fn find_intersections(&self, ray: Ray, world_bounds: AABB) -> Vec<(f32, f32)> {
        match world_bounds.intersect(ray) {
            None => Vec::new(),
            Some((t0, t1)) => {
                let start_pos = ray.at(t0);
                let block_countf = vec3(self.block_count.0 as f32, self.block_count.1 as f32, self.block_count.2 as f32);
                let coordf = fit(start_pos, world_bounds.min, world_bounds.max, vec3f::zero(), block_countf);
                
                // Current block coord
                let coord = (coordf.x as usize, coordf.y as usize, coordf.z as usize);

                let sgn_x = if ray.d.x >= 0.0 { 1 } else { -1 };
                let sgn_y = if ray.d.y >= 0.0 { 1 } else { -1 };
                let sgn_z = if ray.d.z >= 0.0 { 1 } else { -1 };

                // #wip: Finish this
                let end_pos = ray.at(t1);
                let diff = vec3f::abs(endPos - startPos);
                let cell_sizef = self.get_cell_sizef();

                let mut intervals = Vec::new();

                intervals
            }
        }
    }

    fn get_occupancy(&self) -> f32 {
        self.sparse_block_occupancy()
    }

    fn read(&self, i: i32, j: i32, k: i32) -> T {
        self.read_raw(to_usize3(i, j, k))
	}
	fn write(&mut self, i: i32, j: i32, k: i32, value: T) -> () {
        self.write_raw(to_usize3(i, j, k), value);
	}

}
