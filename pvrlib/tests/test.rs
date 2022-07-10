use pvrlib::math::vec3::*;

macro_rules! assert_eq_float {
    ($x: expr, $y: expr) => {
        if ($x - $y).abs() > f32::EPSILON { panic!(); }
    }
}

#[test]
fn test_vec3() {
    // TEST: ctor
    {
        let v = Vec3::new(123.0, 456.0, 789.0);
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
        assert_eq_float!(Vec3::distance(x, y), 5.0);
        assert_eq_float!(Vec3::distance_sq(x, y), 25.0);
    }
}
