// Octree-based sparse field implementation.
// Replaced with sparse grid version.

/*
use super::VoxelBuffer;
use crate::math::aabb::*;
use crate::math::vec3::*;
use crate::math::ray::Ray;

// -------------------------------------
// Auxiliary for sparse buffer impl.

const BLOCK_SIZE: (i32, i32, i32) = (64, 64, 64);

enum Octree<T> {
	Empty,
	Branch(Box<TreeBranch<T>>),
	Leaf(Box<TreeLeaf<T>>)
}

struct TreeBranch<T> {
	min_bounds: (i32, i32, i32), // inclusive
	max_bounds: (i32, i32, i32), // exclusive
	pub children: [Octree<T>; 8] // index = z | y | x
}

struct TreeLeaf<T> {
	pub coord: (i32, i32, i32), // min bounds
	pub size: (i32, i32, i32),
	data: Vec<T>
}

impl<T: Default + Clone> TreeBranch<T> {
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
					data.resize((size_x * size_y * size_z) as usize, T::default());
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

impl<T: Default + Copy> TreeLeaf<T> {
	pub fn read(&self, p: (i32, i32, i32)) -> T {
		let ix = self.index(p);
		self.data[ix]
	}
	pub fn write(&mut self, p: (i32, i32, i32), v: T) {
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

pub struct SparseField<T> {
	size: (i32, i32, i32),
	root: Octree<T>
}

impl<T: Default + Copy> SparseField<T> {
	pub fn new(size: (i32, i32, i32)) -> SparseField<T> {
		if size.0 <= 0 || size.1 <= 0 || size.2 <= 0 {
			panic!("Invalid size: {:?}", size);
		}
		SparseField {
			size: size,
			root: Octree::Empty
		}
	}

	fn read_recurse(&self, node: &Octree<T>, coord: (i32, i32, i32)) -> T {
		match node {
			Octree::Empty => T::default(),
			Octree::Branch(branch) => {
				if branch.contains(coord) {
					let ix = branch.select_child(coord);
					self.read_recurse(&branch.children[ix], coord)
				} else {
					T::default()
				}
			},
			Octree::Leaf(leaf) => {
				leaf.read(coord)
			}
		}
	}

	fn get_occupancy_recurse(&self, node: &Octree<T>, total_voxels: i32) -> f32 {
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

	fn find_leaf<'a>(&self, node: &'a Octree<T>, coord: (i32, i32, i32)) -> Option<&'a TreeLeaf<T>> {
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

impl VoxelBuffer for SparseField<vec3f> {
	fn sample_by_local_position(&self, u: f32, v: f32, w: f32) -> vec3f {
		let vp = vec3(u, v, w) * self.get_sizef();
		let f = (vp - vec3(0.5, 0.5, 0.5)).floor();
		let a = vp - vec3(0.5, 0.5, 0.5) - f;

		// saves a few seconds, but still too slow than dense buffer.
		let sample_offsets: [vec3f; 8] = [
			vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 1.0),
			vec3(1.0, 0.0, 0.0), vec3(1.0, 0.0, 1.0), vec3(1.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0)];
		let mut values: [vec3f; 8] = [
			vec3f::zero(), vec3f::zero(), vec3f::zero(), vec3f::zero(),
			vec3f::zero(), vec3f::zero(), vec3f::zero(), vec3f::zero()];
		let mut prev_node: Option<&TreeLeaf<vec3f>> = None;
		for i in 0..8 {
			let pos = f + sample_offsets[i];
			let posi = (pos.x as i32, pos.y as i32, pos.z as i32);
			match prev_node {
				None => {
					let current_node = self.find_leaf(&self.root, posi);
					match current_node {
						None => {
							values[i] = vec3f::zero();
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

	fn get_size(&self) -> (i32, i32, i32) {
		self.size
	}
	fn get_sizef(&self) -> vec3f {
		vec3(self.size.0 as f32, self.size.1 as f32, self.size.2 as f32)
	}

	// #todo-emptyspace: Slow as hell
	// Each read() needs to traverse the hierarchy from the root,
	// but we identified which leaf is hit with each interval.
	// Access to those leaves should be cached, not just the intervals.
	fn find_intersections(&self, ray: Ray, world_bounds: AABB) -> Vec<(f32, f32)> {
		fn to_vec3(iv: (i32, i32, i32)) -> vec3f {
			vec3(iv.0 as f32, iv.1 as f32, iv.2 as f32)
		}
		fn recurse(node: &Octree<vec3f>, intervals: &mut Vec<(f32, f32)>, ray: Ray) {
			match node {
				Octree::Empty => {
					return;
				},
				Octree::Branch(branch) => {
					for i in 0..8 {
						recurse(&branch.children[i], intervals, ray);
					}
				},
				Octree::Leaf(leaf) => {
					let aabb = AABB {
						min: to_vec3(leaf.coord),
						max: to_vec3(leaf.coord) + to_vec3(leaf.size)
					};
					match aabb.intersect(ray) {
						Some(interval) => {
							intervals.push(interval);
						},
						None => {}
					}
				}
			}
		}

		let mut intervals: Vec<(f32, f32)> = Vec::new();
		let ray_o_vs = fit(ray.o, world_bounds.min, world_bounds.max, vec3f::zero(), self.get_sizef());
		let ray2 = Ray::new(ray_o_vs, ray.d);
		recurse(&self.root, &mut intervals, ray2);

		intervals.sort_by(|(t0,_t1), (s0,_s1)| t0.partial_cmp(s0).unwrap());
		
		// #todo-emptyspace: To cache the leaves, we should not merge the intervals.
		// Merge consecutive intervals
		let n = intervals.len();
		let mut merged: Vec<(f32, f32)> = Vec::new();
		if n > 0 {
			let mut p = 0;
			// #todo: Why warning? (value assigned to q is never read?)
			#[allow(unused_assignments)]
			let mut q = 0;
			while p < n {
				q = p;
				while q + 1 < n {
					if intervals[q].1 + (1e-6) >= intervals[q + 1].0 {
						q += 1;
					} else {
						break;
					}
				}
				merged.push((intervals[p].0, intervals[q].1));
				p = q + 1;
			}
		}

		//if intervals.len() > 0 {
		//	println!("=== sparse intervals ===");
		//	println!("original");
		//	for (t0,t1) in &intervals {
		//		println!("{}, {}", t0, t1);
		//	}
		//	println!("merged");
		//	for (t0,t1) in &merged {
		//		println!("{}, {}", t0, t1);
		//	}
		//}

        merged
	}

	fn get_occupancy(&self) -> f32 {
		let total_voxels = self.size.0 * self.size.1 * self.size.2;
		self.get_occupancy_recurse(&self.root, total_voxels)
	}

	fn read(&self, i: i32, j: i32, k: i32) -> vec3f {
		if i < 0 || j < 0 || k < 0 || i >= self.size.0 || j >= self.size.1 || k >= self.size.2 {
			vec3f::zero()
		} else {
			self.read_recurse(&self.root, (i, j, k))
		}
	}
	fn write(&mut self, i: i32, j: i32, k: i32, value: vec3f) {
		// Nested here because of some shitty error that self cannot be borrowed as mutable twice
		fn recurse(mut node: &mut Octree<vec3f>, v: vec3f, p: (i32, i32, i32), min: (i32, i32, i32), max: (i32, i32, i32)) {
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
*/
