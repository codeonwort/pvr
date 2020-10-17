// #todo-math: Not implemented

pub struct Rotator {
	pub yaw: f32,
	pub pitch: f32,
	pub roll: f32
}

pub fn rotator(yaw: f32, pitch: f32, roll: f32) -> Rotator {
	Rotator { yaw: yaw, pitch: pitch, roll: roll }
}

