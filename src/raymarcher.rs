use crate::vec3::*;
use crate::volume::*;
use crate::ray::*;

pub struct IntegrationResult {
    pub luminance: Vec3,
    pub transmittance: Vec3
}

// #todo: UniformRaymarcher, AdaptiveRaymarcher
pub fn integrate_ray(vol: &dyn Volume, ray: Ray) -> IntegrationResult {
	// #todo: proper step size
	let step_size: f32 = 0.25;

	// Integration bounds
	let interval = vol.get_intersection(ray);
	
	let mut T: Vec3 = Vec3::one();
	let mut L: Vec3 = Vec3::zero();

	if let Some((t_start, t_end)) = interval {
		let mut t_current = t_start;
	
		while t_current < t_end {
			let p_i: Vec3 = ray.at(t_current);
			let Le: Vec3 = vol.emission(p_i);
			let sigma_a: Vec3 = vol.absorption(p_i);
			let T_i: Vec3 = (-sigma_a * step_size).exp();

			T *= T_i;
			L = L + T * Le;

			t_current += step_size;
		}
	}
	
	IntegrationResult { luminance: L, transmittance: T }
}
