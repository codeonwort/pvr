use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::BitAnd; // dot product (&)
use std::ops::BitXor; // cross product (^)
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::cmp::PartialEq;

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Vec3 {
    //pub fn new(v: f32) -> Vec3 { Vec3 { x: v, y: v, z: v } } // no overloading, huh :/
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn distance(a: Vec3, b: Vec3) -> f32 {
        (a - b).length()
    }
    pub fn distance_sq(a: Vec3, b: Vec3) -> f32 {
        (a - b).length_sq()
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        (*self) & rhs
    }
    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        (*self) ^ rhs
    }
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
    pub fn normalize(&self) -> Vec3 {
        let w = 1.0 / self.length();
        w * self.clone()
    }

    pub fn pow(&self, n: f32) -> Vec3 {
        Vec3 { x: self.x.powf(n), y: self.y.powf(n), z: self.z.powf(n) }
    }
    pub fn exp(&self) -> Vec3 {
        Vec3 { x: self.x.exp(), y: self.y.exp(), z: self.z.exp() }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self {
        Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self {
        Vec3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self / rhs.x, y: self / rhs.y, z: self / rhs.z }
    }
}

// dot product (&)
impl BitAnd for Vec3 {
    type Output = f32;
    fn bitand(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

// cross product (^)
impl BitXor for Vec3 {
    type Output = Vec3;
    fn bitxor(self, rhs: Self) -> Self {
        Vec3 { x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.x }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        };
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        };
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        };
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

/*
fn test_vec3() {
	let v1 = Vec3::new(5.0, 1.0, 2.5);
	let v2 = Vec3::new(2.5, 3.3, 1.0);
	println!("v1 = {:?}", v1);
	println!("v2 = {:?}", v2);
	println!("-v1 = {:?}", -v1);
	println!("v1 + v2 = {:?}", v1 + v2);
	println!("v1 - v2 = {:?}", v1 - v2);
	println!("v1 * v2 = {:?}", v1 * v2);
	println!("v1 / v2 = {:?}", v1 / v2);
	println!("v1 & v2 = {:?}", v1 & v2);
	println!("v1 ^ v2 = {:?}", v1 ^ v2);
	println!("v1 == v2 = {:?}", v1 == v2);
	println!("v1.length() = {:?}", v1.length());
	println!("v1.normalize() = {:?}", v1.normalize());
	println!("distance(v1, v2) = {:?}", Vec3::distance(v1, v2));
	println!("v1 * 2.0 = {:?}", v1 * 2.0);
	println!("3.0 * v1 = {:?}", 3.0 * v1);
	println!("v2 / 2.0 = {:?}", v2 / 2.0);
}
*/
