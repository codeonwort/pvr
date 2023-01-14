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
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Copy, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub struct vec3f {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Into<vec3f> for f32 {
    fn into(self) -> vec3f {
        vec3f::new(self, self, self)
    }
}

pub fn vec3(x: f32, y: f32, z: f32) -> vec3f {
    vec3f::new(x, y, z)
}

// #todo-math: Not a good place to put these generics?
pub fn fit<T>(v: T, old_min: T, old_max: T, new_min: T, new_max: T) -> T
    where T: Add<Output=T> + Sub<Output=T> + Div<Output=T> + Mul<Output=T> + Copy
{
    new_min + (new_max - new_min) * (v - old_min) / (old_max - old_min)
}

pub fn lerp<T>(x: T, y: T, a: f32) -> T
    where T: Add<Output=T> + Mul<f32, Output=T>
{
    x * (1.0 - a) + (y * a)
}

impl vec3f {
    // Constructors
    pub fn new(x: f32, y: f32, z: f32) -> vec3f {
        vec3f { x: x, y: y, z: z }
    }
    pub fn zero() -> vec3f { vec3f::new(0.0, 0.0, 0.0) }
    pub fn one() -> vec3f { vec3f::new(1.0, 1.0, 1.0) }

    // Static methods
    pub fn distance(a: vec3f, b: vec3f) -> f32 {
        (a - b).length()
    }
    pub fn distance_sq(a: vec3f, b: vec3f) -> f32 {
        (a - b).length_sq()
    }
    pub fn min(a: vec3f, b: vec3f) -> vec3f {
        vec3f::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
    }
    pub fn max(a: vec3f, b: vec3f) -> vec3f {
        vec3f::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
    }

    // Member methods
    pub fn dot(&self, rhs: vec3f) -> f32 {
        (*self) & rhs
    }
    pub fn cross(&self, rhs: vec3f) -> vec3f {
        (*self) ^ rhs
    }
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
    pub fn normalize(&self) -> vec3f {
        let w = 1.0 / self.length();
        w * self.clone()
    }

    pub fn max_component(&self) -> f32 {
        if self.x < self.y {
            if self.y < self.z {
                self.z
            } else {
                self.y
            }
        } else if self.x < self.z {
            self.z
        } else {
            self.x
        }
    }

    pub fn min_component(&self) -> f32 {
        if self.x < self.y {
            if self.x < self.z {
                self.x
            } else {
                self.z
            }
        } else if self.y < self.z {
            self.y
        } else {
            self.z
        }
    }

    pub fn pow(&self, n: f32) -> vec3f {
        vec3f { x: self.x.powf(n), y: self.y.powf(n), z: self.z.powf(n) }
    }
    pub fn exp(&self) -> vec3f {
        vec3f { x: self.x.exp(), y: self.y.exp(), z: self.z.exp() }
    }
    pub fn floor(&self) -> vec3f {
        vec3f { x: self.x.floor(), y: self.y.floor(), z: self.z.floor() }
    }
    pub fn fract(&self) -> vec3f {
        vec3f { x: self.x.fract(), y: self.y.fract(), z: self.z.fract() }
    }
    pub fn fract_glsl(&self) -> vec3f {
        *self - self.floor()
    }
}

impl Neg for vec3f {
    type Output = vec3f;
    fn neg(self) -> Self {
        vec3f { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for vec3f {
    type Output = vec3f;
    fn add(self, rhs: Self) -> Self {
        vec3f { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for vec3f {
    type Output = vec3f;
    fn sub(self, rhs: Self) -> Self {
        vec3f { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

// Component-wise product
impl Mul<vec3f> for vec3f {
    type Output = vec3f;
    fn mul(self, rhs: Self) -> Self {
        vec3f { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl Mul<f32> for vec3f {
    type Output = vec3f;
    fn mul(self, rhs: f32) -> Self {
        vec3f { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<vec3f> for f32 {
    type Output = vec3f;
    fn mul(self, rhs: vec3f) -> vec3f {
        vec3f { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

// Component-wise division
impl Div<vec3f> for vec3f {
    type Output = vec3f;
    fn div(self, rhs: Self) -> Self {
        vec3f { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}
impl Div<f32> for vec3f {
    type Output = vec3f;
    fn div(self, rhs: f32) -> Self {
        vec3f { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
impl Div<vec3f> for f32 {
    type Output = vec3f;
    fn div(self, rhs: vec3f) -> vec3f {
        vec3f { x: self / rhs.x, y: self / rhs.y, z: self / rhs.z }
    }
}

// dot product (&)
impl BitAnd for vec3f {
    type Output = f32;
    fn bitand(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

// cross product (^)
impl BitXor for vec3f {
    type Output = vec3f;
    fn bitxor(self, rhs: Self) -> Self {
        vec3f { x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.x }
    }
}

impl AddAssign for vec3f {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl SubAssign for vec3f {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl MulAssign<vec3f> for vec3f {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        };
    }
}
impl MulAssign<f32> for vec3f {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl DivAssign<vec3f> for vec3f {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        };
    }
}
impl DivAssign<f32> for vec3f {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        };
    }
}

impl PartialEq for vec3f {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl Index<usize> for vec3f {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        if index == 0 {
            &self.x
        } else if index == 1 {
            &self.y
        } else if index == 2 {
            &self.z
        } else {
            panic!("undefined index {}", index)
        }
    }
}

impl IndexMut<usize> for vec3f {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        if index == 0 {
            &mut self.x
        } else if index == 1 {
            &mut self.y
        } else if index == 2 {
            &mut self.z
        } else {
            panic!("undefined index {}", index)
        }
    }
}
