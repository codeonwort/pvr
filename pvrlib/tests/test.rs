use pvrlib::math::vec3::*;
use pvrlib::math::noise::*;
use pvrlib::voxelbuffer::VoxelBuffer;
use pvrlib::voxelbuffer::sparse::SparseField;
use pvrlib::render::rendertarget::RenderTarget;

use bit_vec::BitVec;

macro_rules! assert_eq_float {
    ($x: expr, $y: expr) => {
        if ($x - $y).abs() > f32::EPSILON { panic!(); }
    }
}

#[test]
fn test_vec3() {
    // TEST: ctor
    {
        let v = vec3f::new(123.0, 456.0, 789.0);
        assert_eq!(v[0], 123.0);
        assert_eq!(v.y, 456.0);
        assert_eq!(v[2], 789.0);
        
        // Successfully panic: 'undefined index 3'
        //assert_eq!(v[3], 0.0);

        assert_eq!(-v, vec3(-123.0, -456.0, -789.0));
    }

    // TEST: addition & subtraction
    {
        let x = vec3(-5.0, -26.0, 16123.0);
        let y = vec3(6123.0, -1623.0, -4625.0);
        assert_eq!(x + y, vec3(6118.0, -1649.0, 11498.0));
        assert_eq!(x - y, vec3(-6128.0, 1597.0, 20748.0));
    }

    // TEST: multiplication & division
    {
        let x = vec3(5.0, 0.0, 4.0);
        let y = vec3(2.0, 4.0, 8.0);
        assert_eq!(x * 3.0, vec3(15.0, 0.0, 12.0));
        assert_eq!(x * y, vec3(10.0, 0.0, 32.0));

        let z = vec3(128.0, 64.0, 32.0);
        let w = vec3(4.0, 8.0, 16.0);
        assert_eq!(z / 2.0, vec3(64.0, 32.0, 16.0));
        assert_eq!(z / w, vec3(32.0, 8.0, 2.0));
    }

    // TEST: dot product (&)
    {
        let x = vec3(1.0, 2.0, 3.0);
        let y = vec3(4.0, 5.0, 6.0);
        assert_eq!(x.dot(y), 32.0);

        let x = vec3(99.0, 0.0, 0.0);
        let y = vec3(0.0, 27.0, 5929.0);
        assert_eq!(x & y, 0.0);
    }

    // TEST: cross product (^)
    {
        let x = vec3(1.0, 0.0, 0.0);
        let y = vec3(0.0, 1.0, 0.0);
        assert_eq!(x.cross(y), vec3(0.0, 0.0, 1.0));
    }

    // TEST: normalize
    {
        let x = vec3(1.0, -1.0, 0.45);
        let y = x.normalize();
        assert_eq_float!(y.length(), 1.0);
    }

    // TEST: distance
    {
        let x = vec3(0.0, 3.0, 0.0);
        let y = vec3(4.0, 0.0, 0.0);
        assert_eq_float!(vec3f::distance(x, y), 5.0);
        assert_eq_float!(vec3f::distance_sq(x, y), 25.0);
    }
}

#[test]
fn test_sparse_buffer() {
    let mut buffer = SparseField::<vec3f>::new((512, 512, 256));

    println!("> write sparse buffer...");
    buffer.write(0, 0, 0, vec3(3.0, 4.0, 5.0));
    buffer.write(50, 0, 70, vec3(7.0, 5.0, 2.0));
    buffer.write(5, 0, 99, vec3(8.0, 1.0, 6.0));
    buffer.write(99, 99, 99, vec3(5.0, 3.0, 1.0));
    buffer.write(46, 0, -270, vec3(31.0, 42.0, 53.0));
    //for y in 0..512 { buffer.write(0, y, 0, vec3(y as f32, 1.0, 1.0)); }

    println!("> read sparse buffer...");
    assert_eq!(buffer.read(0, 0, 0), vec3(3.0, 4.0, 5.0));
    assert_eq!(buffer.read(50, 0, 70), vec3(7.0, 5.0, 2.0));
    assert_eq!(buffer.read(5, 0, 99), vec3(8.0, 1.0, 6.0));
    assert_eq!(buffer.read(99, 99, 99), vec3(5.0, 3.0, 1.0));
    //assert_eq!(buffer.read(46, 0, -270), vec3(31.0, 42.0, 53.0));
    //for y in 0..512 { println!("buffer[0,{},0] = {:?}", y, buffer.read(0, y, 0)); }

    /*
    // Debug occupancy
    {
        let buffer_size = (64, 64, 64);
        let mut buffer = SparseBuffer::new(buffer_size, bounds);
        println!("> buffer size: {:?}", buffer_size);

        println!("> occupancy = {}", buffer.get_occupancy());

        println!("> write to (0,0,0)");
        buffer.write(0, 0, 0, vec3(1.0, 1.0, 1.0));
        println!("> occupancy = {}", buffer.get_occupancy());

        println!("> write to (63,63,63)");
        buffer.write(63, 63, 63, vec3(1.0, 1.0, 1.0));
        println!("> occupancy = {}", buffer.get_occupancy());
    }
    */
}

#[test]
fn test_procedural_noise() {
    let mut rt = RenderTarget::new(128, 128);

    let width = rt.get_width();
    let height = rt.get_height();
    let inv_width = 1.0 / (width as f32);
    let inv_height = 1.0 / (height as f32);

    let z: f32 = 0.0;
    for y in 0..height {
        for x in 0..width {
            let u = 2.0 * (x as f32) * inv_width - 1.0;
            let v = 2.0 * (y as f32) * inv_height - 1.0;
            let uv_len = (1.0_f32 - z * z).sqrt();
            let p = vec3(uv_len * u, uv_len * v, z);
            
            let noise = fBm(p * 4.0);

            let sphere_func = p.length() - 1.0;
            let filter_width = 2.0;
            let pyro = pyroclastic(sphere_func, noise, filter_width);

            rt.set(x as i32, y as i32, pyro.into());
        }
    }

    // #todo-test: Hmm.. what to assert here?
}

#[test]
fn test_bit_vec() {
    let nbits = 1024 * 1024;
    let mut bvec = BitVec::from_elem(nbits, false);
    //bvec[0] = true; // No IndexMut...

    bvec.set(1, true);
    bvec.set(2, true);
    bvec.set(7, true);

    assert_eq!(bvec.get(0).unwrap(), false);
    assert_eq!(bvec.get(1).unwrap(), true);
    assert_eq!(bvec.get(nbits - 1), Some(false));
    assert_eq!(bvec.get(nbits), None);
    assert_eq!(bvec.get(nbits + 0), None);
}
