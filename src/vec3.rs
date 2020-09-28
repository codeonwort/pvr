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

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self {
        Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self {
        Vec3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
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

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        };
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}
