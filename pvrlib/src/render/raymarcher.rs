use crate::math::vec3::*;
use crate::math::ray::*;
use crate::light::*;
use crate::volume::*;

/* Math cheatsheet

dL(p,w) = Li(p,w) - Lo(p,w)
        = emission + scattering_in - scattering_out - absorption

dLa = -sigma_a * Li(p,w)ds
dLe = Le(p,w)ds
dLo(p,w) = -sigma_s * Li(p,w)ds
dLi(p,w) = sigma_s * phase(w,w') * S(p,w')ds
	where S = radiance coming from light source

*/

pub struct IntegrationResult {
    pub luminance: vec3f,
    pub transmittance: vec3f
}

// #todo: UniformRaymarcher, AdaptiveRaymarcher
#[allow(non_snake_case)]
pub fn integrate_ray(
	vol: &dyn Volume,
	ray: Ray,
	lights: &[Box<dyn Light>],
	primary_step_size: f32,
	secondary_step_size: f32) -> IntegrationResult
{
	// #todo-refactor: Interval struct
	// Integration bounds
	let intervals: Vec<(f32, f32)> = vol.find_intersections(ray);
	
	let mut T: vec3f = vec3f::one(); // total transmittance
	let mut L: vec3f = vec3f::zero(); // total luminance

	// Loop for primary ray
	for (t_start, t_end) in intervals {
		let mut t_current = t_start;
	
		while t_current < t_end {
			let p_i: vec3f = ray.at(t_current);

			// Sample the volume
			let vol_sample: VolumeSample = vol.sample(p_i);
			let L_em = vol_sample.emission;
			let sigma_a = vol_sample.absorption_coeff;
			let sigma_s = vol_sample.scattering_coeff;

			let mut L_sc = vec3f::zero(); // luminance by scattering
			
			// Loop for secondary ray
			for light in lights {
				let light_sample: LightSample = light.sample(p_i, ray.d);
				let wi = (light_sample.position - p_i).normalize();

				// Transmittance between current sampling point and light source
				let mut T_L: vec3f = vec3f::one();
				{
					let mut t_L = 0.0;

					let mut t_L_end = (light_sample.position - p_i).length();
					for (_, t_L_end2) in vol.find_intersections(Ray::new(p_i, wi)) {
						t_L_end = if t_L_end2 < t_L_end { t_L_end2 } else { t_L_end };
					}

					while t_L < t_L_end {
						let p_L = p_i + wi * t_L;
						let sigma_a_L = vol.absorption_coeff(p_L);

						T_L *= (-sigma_a_L * secondary_step_size).exp();
						t_L += secondary_step_size;
						
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
			
			let T_i: vec3f = (-sigma_a * primary_step_size).exp();

			T *= T_i;
			L += (L_em + L_sc) * T * primary_step_size;

			// Stop raymarching if too opaque
			if T.max_component() < 0.01 {
				break;
			}

			t_current += primary_step_size;
		}
	}
	
	IntegrationResult { luminance: L, transmittance: T }
}
