use super::VoxelBuffer;
use crate::math::aabb::*;
use crate::math::vec3::*;

// -------------------------------------
// Auxiliary for sparse buffer impl.

const BLOCK_SIZE: (i32, i32, i32) = (64, 64, 64);

enum Octree {
	Empty,
	Branch(Box<TreeBranch>),
	Leaf(Box<TreeLeaf>)
}

struct TreeBranch {
	min_bounds: (i32, i32, i32), // inclusive
	max_bounds: (i32, i32, i32), // exclusive
	children: [Octree; 8] // index = z | y | x
}

struct TreeLeaf {
	pub coord: (i32, i32, i32),
	pub size: (i32, i32, i32),
	data: Vec<Vec3>
}

impl TreeBranch {
	pub fn contains(&self, p: (i32, i32, i32)) -> bool {
		self.min_bounds.0 <= p.0 && p.0 < self.max_bounds.0
			&& self.min_bounds.1 <= p.1 && p.1 < self.max_bounds.1
			&& self.min_bounds.2 <= p.2 && p.2 < self.max_bounds.2
	}
	pub fn select_child(&self, p: (i32, i32, i32)) -> usize {
		let size_x = self.max_bounds.0 - self.min_bounds.0;
		let size_y = self.max_bounds.1 - self.min_bounds.1;
		let size_z = self.max_bounds.2 - self.min_bounds.2;
		// #todo: it can be cached whether this branch is smaller than the block size
		if size_x <= BLOCK_SIZE.0 && size_y <= BLOCK_SIZE.1 && size_z <= BLOCK_SIZE.2 {
			0
		} else {
			let x = if p.0 < (self.min_bounds.0 + self.max_bounds.0) / 2 { 0 } else { 1 };
			let y = if p.1 < (self.min_bounds.1 + self.max_bounds.1) / 2 { 0 } else { 1 };
			let z = if p.2 < (self.min_bounds.2 + self.max_bounds.2) / 2 { 0 } else { 1 };
			(z << 2) | (y << 1) | x
		}
	}
	pub fn ensure_nonempty_child(&mut self, ix: usize) {
		match self.children[ix] {
			Octree::Empty => {
				let size_x = self.max_bounds.0 - self.min_bounds.0;
				let size_y = self.max_bounds.1 - self.min_bounds.1;
				let size_z = self.max_bounds.2 - self.min_bounds.2;
				if size_x <= BLOCK_SIZE.0 && size_y <= BLOCK_SIZE.1 && size_z <= BLOCK_SIZE.2 {
					let mut data = Vec::new();
					data.resize((size_x * size_y * size_z) as usize, Vec3::zero());
					self.children[ix] = Octree::Leaf(Box::new(TreeLeaf {
						coord: self.min_bounds,
						size: (size_x, size_y, size_z),
						data: data
					}));
				} else {
					let x = ix & 1;
					let y = (ix & 2) >> 1;
					let z = (ix & 4) >> 2;
					let mut min = self.min_bounds;
					let mut max = self.max_bounds;
					let half = ((max.0 - min.0) / 2, (max.1 - min.1) / 2, (max.2 - min.2) / 2);
					if x == 0 {
						max.0 = min.0 + half.0;
					} else {
						min.0 += half.0;
					}
					if y == 0 {
						max.1 = min.1 + half.1;
					} else {
						min.1 += half.1;
					}
					if z == 0 {
						max.2 = min.2 + half.2;
					} else {
						min.2 += half.2;
					}
					self.children[ix] = Octree::Branch(Box::new(TreeBranch {
						min_bounds: min,
						max_bounds: max,
						children: [Octree::Empty, Octree::Empty, Octree::Empty, Octree::Empty,
									Octree::Empty, Octree::Empty, Octree::Empty, Octree::Empty]
					}));
				}
			},
			_ => ()
		}
	}
}

impl TreeLeaf {
	pub fn read(&self, p: (i32, i32, i32)) -> Vec3 {
		let ix = self.index(p);
		self.data[ix]
	}
	pub fn write(&mut self, p: (i32, i32, i32), v: Vec3) {
		let ix = self.index(p);
		self.data[ix] = v;
	}
	pub fn contains(&self, p: (i32, i32, i32)) -> bool {
		(self.coord.0 <= p.0) && (p.0 < self.coord.0 + self.size.0)
		&& (self.coord.1 <= p.1) && (p.1 < self.coord.1 + self.size.1)
		&& (self.coord.2 <= p.2) && (p.2 < self.coord.2 + self.size.2)
	}
	fn index(&self, p: (i32, i32, i32)) -> usize {
		let x = p.0 - self.coord.0;
		let y = p.1 - self.coord.1;
		let z = p.2 - self.coord.2;
		let ix = (z * self.size.0 * self.size.1) + (y * self.size.0) + x;
		ix as usize
	}
}

// -------------------------------------
// Sparse buffer impl.

pub struct SparseBuffer {
	size: (i32, i32, i32),
	ws_bounds: AABB,
	root: Octree
}

impl SparseBuffer {
	pub fn new(size: (i32, i32, i32), ws_bounds: AABB) -> SparseBuffer {
		if size.0 <= 0 || size.1 <= 0 || size.2 <= 0 {
			panic!("Invalid size: {:?}", size);
		}
		SparseBuffer {
			size: size,
			ws_bounds: ws_bounds,
			root: Octree::Empty
		}
	}

	fn sample_by_voxel_position(&self, p: Vec3) -> Vec3 {
		let vp = p + vec3(0.5, 0.5, 0.5);
		let (x, y, z) = (vp.x as i32, vp.y as i32, vp.z as i32);

		if x < 0 || y < 0 || z < 0 || x >= self.size.0 || y >= self.size.1 || z >= self.size.2 {
			Vec3::zero()
		} else {
			self.read(x, y, z)
		}
	}

	fn read_recurse(&self, node: &Octree, coord: (i32, i32, i32)) -> Vec3 {
		match node {
			Octree::Empty => Vec3::zero(),
			Octree::Branch(branch) => {
				if branch.contains(coord) {
					let ix = branch.select_child(coord);
					self.read_recurse(&branch.children[ix], coord)
				} else {
					Vec3::zero()
				}
			},
			Octree::Leaf(leaf) => {
				leaf.read(coord)
			}
		}
	}

	fn get_occupancy_recurse(&self, node: &Octree, total_voxels: i32) -> f32 {
		match node {
			Octree::Empty => 0.0,
			Octree::Branch(branch) => {
				let branch_voxels = (branch.max_bounds.0 - branch.min_bounds.0)
					* (branch.max_bounds.1 - branch.min_bounds.1)
					* (branch.max_bounds.2 - branch.min_bounds.2);
				let mut occ = 0.0;
				for ix in 0..8 {
					occ += self.get_occupancy_recurse(&branch.children[ix], branch_voxels);
				}
				occ * (branch_voxels as f32) / (total_voxels as f32)
			},
			Octree::Leaf(leaf) => {
				let leaf_voxels = leaf.size.0 * leaf.size.1 * leaf.size.2;
				(leaf_voxels as f32) / (total_voxels as f32)
			}
		}
	}

	fn find_leaf<'a>(&self, node: &'a Octree, coord: (i32, i32, i32)) -> Option<&'a TreeLeaf> {
		match node {
			Octree::Empty => None,
			Octree::Branch(branch) => {
				if branch.contains(coord) {
					let ix = branch.select_child(coord);
					self.find_leaf(&branch.children[ix], coord)
				} else {
					None
				}
			},
			Octree::Leaf(leaf) => Some(&leaf)
		}
	}
}

impl VoxelBuffer for SparseBuffer {
	fn sample_by_local_position(&self, u: f32, v: f32, w: f32) -> Vec3 {
		let size = self.get_sizef();
		self.sample_by_voxel_position(size * vec3(u, v, w))
	}
	fn sample_by_world_position(&self, p: Vec3) -> Vec3 {
		let vp = self.world_to_voxel(p);
		let f = (vp - vec3(0.5, 0.5, 0.5)).floor();
		let a = vp - vec3(0.5, 0.5, 0.5) - f;

		// saves a few seconds, but still too slow than dense buffer.
		let sample_offsets: [Vec3; 8] = [
			vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 1.0),
			vec3(1.0, 0.0, 0.0), vec3(1.0, 0.0, 1.0), vec3(1.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0)];
		let mut values: [Vec3; 8] = [
			Vec3::zero(), Vec3::zero(), Vec3::zero(), Vec3::zero(),
			Vec3::zero(), Vec3::zero(), Vec3::zero(), Vec3::zero()];
		let mut prev_node: Option<&TreeLeaf> = None;
		for i in 0..8 {
			let pos = f + sample_offsets[i];
			let posi = (pos.x as i32, pos.y as i32, pos.z as i32);
			match prev_node {
				None => {
					let current_node = self.find_leaf(&self.root, posi);
					match current_node {
						None => {
							values[i] = Vec3::zero();
						},
						Some(current_leaf) => {
							values[i] = current_leaf.read(posi);
							prev_node = current_node;
						}
					}
				},
				Some(leaf) => {
					if leaf.contains(posi) {
						values[i] = leaf.read(posi);
					} else {
						let current_node = self.find_leaf(&self.root, posi);
						if let Some(current_leaf) = current_node {
							if current_leaf.contains(posi) {
								values[i] = current_leaf.read(posi);
								prev_node = current_node;
							}
						}
					}
				}
			}
		}

		let front = lerp(lerp(values[0], values[4], a.x), lerp(values[2], values[6], a.x), a.y);
		let back = lerp(lerp(values[1], values[5], a.x), lerp(values[3], values[7], a.x), a.y);

		lerp(front, back, a.z)

		/* 8 individual traverses are too slow
		let v000 = self.sample_by_voxel_position(f);
		let v001 = self.sample_by_voxel_position(f + vec3(0.0, 0.0, 1.0));
		let v010 = self.sample_by_voxel_position(f + vec3(0.0, 1.0, 0.0));
		let v011 = self.sample_by_voxel_position(f + vec3(0.0, 1.0, 1.0));
		let v100 = self.sample_by_voxel_position(f + vec3(1.0, 0.0, 0.0));
		let v101 = self.sample_by_voxel_position(f + vec3(1.0, 0.0, 1.0));
		let v110 = self.sample_by_voxel_position(f + vec3(1.0, 1.0, 0.0));
		let v111 = self.sample_by_voxel_position(f + vec3(1.0, 1.0, 1.0));

		let front = lerp(lerp(v000, v100, a.x), lerp(v010, v110, a.x), a.y);
		let back = lerp(lerp(v001, v101, a.x), lerp(v011, v111, a.x), a.y);

		lerp(front, back, a.z)
		*/
	}

	fn world_to_voxel(&self, p: Vec3) -> Vec3 {
		fit(p, self.ws_bounds.min, self.ws_bounds.max, Vec3::zero(), self.get_sizef())
	}
	fn voxel_to_world(&self, p: Vec3) -> Vec3 {
		fit(p, Vec3::zero(), self.get_sizef(), self.ws_bounds.min, self.ws_bounds.max)
	}

	fn get_size(&self) -> (i32, i32, i32) {
		self.size
	}
	fn get_sizef(&self) -> Vec3 {
		vec3(self.size.0 as f32, self.size.1 as f32, self.size.2 as f32)
	}
	fn get_ws_bounds(&self) -> AABB {
		self.ws_bounds
	}

	fn get_occupancy(&self) -> f32 {
		let total_voxels = self.size.0 * self.size.1 * self.size.2;
		self.get_occupancy_recurse(&self.root, total_voxels)
	}

	fn read(&self, i: i32, j: i32, k: i32) -> Vec3 {
		if i < 0 || j < 0 || k < 0 || i >= self.size.0 || j >= self.size.1 || k >= self.size.2 {
			Vec3::zero()
		} else {
			self.read_recurse(&self.root, (i, j, k))
		}
	}
	fn write(&mut self, i: i32, j: i32, k: i32, value: Vec3) {
		// Nested here because of some shitty error that self cannot be burrowed as mutable twice :/
		fn recurse(mut node: &mut Octree, v: Vec3, p: (i32, i32, i32), min: (i32, i32, i32), max: (i32, i32, i32)) {
			match node {
				Octree::Empty => {
					*node = Octree::Branch( Box::new(TreeBranch {
						min_bounds: min,
						max_bounds: max,
						children: [Octree::Empty, Octree::Empty, Octree::Empty, Octree::Empty,
									Octree::Empty, Octree::Empty, Octree::Empty, Octree::Empty]
					}) );
					recurse(&mut node, v, p, min, max);
				},
				Octree::Branch(branch) => {
					let ix = branch.select_child(p);
					branch.ensure_nonempty_child(ix);
					let child_min;
					let child_max;
					match &branch.children[ix] {
						Octree::Empty => {
							panic!("Unexpected case");
						},
						Octree::Branch(child) => {
							child_min = child.min_bounds;
							child_max = child.max_bounds;
						},
						Octree::Leaf(child) => {
							child_min = child.coord;
							child_max = (child.coord.0 + child.size.0, child.coord.1 + child.size.1, child.coord.2 + child.size.2);
						}
					}
					recurse(&mut branch.children[ix], v, p, child_min, child_max);
				},
				Octree::Leaf(leaf) => {
					leaf.write(p, v);
				}
			}
		}

		if i < 0 || j < 0 || k < 0 || i >= self.size.0 || j >= self.size.1 || k >= self.size.2 {
			// out of bounds
		} else {
			recurse(&mut self.root, value, (i, j, k), (0, 0, 0), self.size);
		}
	}
}

