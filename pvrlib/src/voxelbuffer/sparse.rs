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
            size.0 / BLOCK_SIZE.0 + ((size.0 % BLOCK_SIZE.0 != 0) as usize),
            size.1 / BLOCK_SIZE.1 + ((size.1 % BLOCK_SIZE.1 != 0) as usize),
            size.2 / BLOCK_SIZE.2 + ((size.2 % BLOCK_SIZE.2 != 0) as usize));

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

    pub fn get_occupancy(&self) -> f32 {
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
