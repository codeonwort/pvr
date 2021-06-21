use crate::math::vec3::*;
use crate::math::ray::*;
use crate::light::*;
use crate::volume::*;

pub struct IntegrationResult {
    pub luminance: Vec3,
    pub transmittance: Vec3
}

// #todo: Replace vol with voxel buffer for now
// #todo: UniformRaymarcher, AdaptiveRaymarcher
#[allow(non_snake_case)]
pub fn integrate_ray(vol: &dyn Volume, ray: Ray, lights: &[Box<dyn Light>]) -> IntegrationResult {
	// #todo: proper step size
	let step_size: f32 = 1.0;

	// Integration bounds
	let intervals = vol.get_intersection(ray);
	
	let mut T: Vec3 = Vec3::one(); // total transmittance
	let mut L: Vec3 = Vec3::zero(); // total luminance

	for (t_start, t_end) in intervals {
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
				let wi = (light_sample.position - p_i).normalize();

				// Transmittance between current sampling point and light source
				let mut T_L: Vec3 = Vec3::one();
				{
					let step_L = 1.0;
					let mut t_L = 0.0;

					let mut t_L_end = (light_sample.position - p_i).length();
					for (_, t_L_end2) in vol.get_intersection(Ray::new(p_i, wi)) {
						t_L_end = if t_L_end2 < t_L_end { t_L_end2 } else { t_L_end };
					}

					while t_L < t_L_end {
						let p_L = p_i + wi * t_L;
						let sigma_a_L = vol.absorption(p_L);

						T_L *= (-sigma_a_L * step_L).exp();
						t_L += step_L;
						
						if T_L.x < 0.01 && T_L.y < 0.01 && T_L.z < 0.01 {
							break;
						}
					}
				}

				// Scattering probability
				let sc_prob = vol.phase_function(p_i, -wi, ray.d);

				// #todo: L_sc contributes almost nothing. (sc_prob is too small)
				L_sc += sigma_s * sc_prob * light_sample.luminance * T_L;
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
