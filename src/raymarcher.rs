use crate::vec3::*;
use crate::volume::volume::*;
use crate::ray::*;
use crate::light::*;

pub struct IntegrationResult {
    pub luminance: Vec3,
    pub transmittance: Vec3
}

// #todo: Replace vol with voxel buffer for now
// #todo: UniformRaymarcher, AdaptiveRaymarcher
#[allow(non_snake_case)]
pub fn integrate_ray(vol: &dyn Volume, ray: Ray, lights: &[Box<dyn Light>]) -> IntegrationResult {
	// #todo: proper step size
	let step_size: f32 = 0.25;

	// Integration bounds
	let interval = vol.get_intersection(ray);
	
	let mut T: Vec3 = Vec3::one(); // total transmittance
	let mut L: Vec3 = Vec3::zero(); // total luminance

	if let Some((t_start, t_end)) = interval {
		let mut t_current = t_start;
	
		while t_current < t_end {
			let p_i: Vec3 = ray.at(t_current);

			// Sampling
			let L_em: Vec3 = vol.emission(p_i);
			let sigma_a: Vec3 = vol.absorption(p_i);
			let sigma_s: Vec3 = vol.scattering(p_i);

			let mut L_sc = Vec3::zero(); // luminance by scattering
			for light in lights {
				let light_sample = light.sample(p_i, ray.d);
				let wi = (p_i - light_sample.position).normalize();
				let sc_prob = vol.phase_function(wi, ray.d);
				L_sc += sigma_s * sc_prob * light_sample.luminance;
			}

			let T_i: Vec3 = (-sigma_a * step_size).exp();

			T *= T_i;
			L += (L_em + L_sc) * T * step_size;

			if T.x < 0.01 && T.y < 0.01 && T.z < 0.01 {
				break;
			}

			t_current += step_size;
		}
	}
	
	IntegrationResult { luminance: L, transmittance: T }
}
