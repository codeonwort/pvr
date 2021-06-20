// #todo-math: Not implemented

use pvrlib::math::vec3::*;
use std::default::Default;
use std::convert::Into;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
	translation: Vec3,
	rotation: Rotator,
	scale: Vec3
}

impl Transform {
	pub fn identity(): Transform {
		Transform {
			translation: Vec3::zero(),
			rotation: Rotator::zero(),
			scale: Vec3::one()
		}
	}

	pub fn transform_position(&self, p: Vec3) -> Vec3 {
		//
	}
	pub fn transform_direction(&self, dir: Vec3) -> Vec3 {
		//
	}
}

impl Default for Transform {
	fn default() -> Transform {
		Transform::identity()
	}
}

impl Into<Matrix> for Transform {
	fn into(self) -> Matrix {
		//
	}
}

impl Mul<Transform> for Transform {
	type Output = Transform;
	fn mul(self, rhs: Self) -> Self {
		//
	}
}
