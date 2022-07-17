
use crate::math::vec3::*;
use crate::math::ray::Ray;
use crate::math::sphere::Sphere;

// Forked from my OpenGL project: https://github.com/codeonwort/pathosengine

const MAGIC_RAYLEIGH: f32 = 1.0;
const MAGIC_MIE: f32 = 0.3;
const PI: f32 = std::f32::consts::PI;

const NUM_PRIMARY_STEPS: i32 = 64;
const NUM_SECONDARY_STEPS: i32 = 8;

// Unit: meters
const SUN_DISTANCE: f32          = 1.496e11;
const SUN_RADIUS: f32            = 6.9551e8;
const EARTH_RADIUS: f32          = 6.36e6;
const ATMOSPHERE_RADIUS: f32     = 6.42e6;
const GROUND_EPSILON: f32        = 1.84; // Avoid collision to ground at uv.x = 0
//const MAX_ALTITUDE: f32          = ATMOSPHERE_RADIUS - EARTH_RADIUS - GROUND_EPSILON;
#[allow(non_upper_case_globals)]
const Hr: f32                    = 7.994e3;
#[allow(non_upper_case_globals)]
const Hm: f32                    = 1.2e3;
// Unit: 1 / meters
#[allow(non_upper_case_globals)]
const BetaR: Vec3                = Vec3 { x: 5.8e-6, y: 13.5e-6, z: 33.1e-6 };
#[allow(non_upper_case_globals)]
const BetaM: Vec3                = Vec3 { x: 21e-6, y: 21e-6, z: 21e-6 };

#[allow(non_snake_case)]
fn phaseR(cos_theta: f32) -> f32
{
    3.0 / (16.0 * PI) * (1.0 + cos_theta * cos_theta)
}
#[allow(non_snake_case, non_upper_case_globals)]
fn phaseM(t: f32) -> f32
{
    const g: f32 = 0.76;
    let gg = g * g;
    let tt = t * t;
    let gt = g * t;
    
    let num = 3.0 * (1.0 - gg) * (1.0 + tt);
	let denom = (8.0 * PI) * (2.0 + gg) * (1.0 + gg - 2.0 * gt).powf(1.5);

    num / denom
}

fn get_sun_image(camera_ray: Ray, sun_direction: Vec3, sun_size: f32, sun_intensity: f32) -> f32 {
    let threshold = (SUN_RADIUS / SUN_DISTANCE).asin();
    let angle = camera_ray.d.dot(-sun_direction).acos();
    if angle <= threshold * sun_size {
        sun_intensity
    } else {
        0.0
    }
}

// Generates atmospheric scattering
pub struct SkyAtmosphere {
    is_empty: bool,
    sun_direction: Vec3, // Sun incoming direction
    sun_size: f32,
    sun_intensity: f32,
    atmosphere: Sphere
}

impl SkyAtmosphere {
    // Static methods
    pub fn new_empty() -> SkyAtmosphere {
        SkyAtmosphere {
            is_empty: true,
            sun_direction: vec3(0.0, -1.0, 0.0),
            sun_size: 1.0,
            sun_intensity: 0.0,
            atmosphere: Sphere::default()
        }
    }
    pub fn new_atmosphere(sun_direction: Vec3, sun_size: f32, sun_intensity: f32) -> SkyAtmosphere {
        SkyAtmosphere {
            is_empty: false,
            sun_direction: sun_direction.normalize(),
            sun_size: sun_size,
            sun_intensity: sun_intensity,
            atmosphere: Sphere { origin: Vec3::zero(), radius: ATMOSPHERE_RADIUS }
        }
    }

    pub fn get_camera_ray_on_earth(original_ray: Ray) -> Ray {
        Ray::new(original_ray.o + vec3(0.0, EARTH_RADIUS + GROUND_EPSILON, 0.0), original_ray.d)
    }

    // #todo-sky: Multiple scattering
    #[allow(non_snake_case)]
    pub fn sample(&self, ray: Ray) -> Vec3 {
        if self.is_empty {
            return Vec3::zero();
        }

        // Return value
        let mut result = Vec3::zero();

        let sun_size = self.sun_size;
        let sun_dir = self.sun_direction;
        let sun_intensity = self.sun_intensity;

        let hit = self.atmosphere.intersect(ray);
        
        let mu = (-sun_dir).dot(ray.d);
        let mut optical_depth = Vec3::zero();

        let P0 = ray.o;
        let mut P = P0;
        //let Q = ray.at(hit.ray_t);
        let segment_length = hit.ray_t / (NUM_PRIMARY_STEPS as f32);
        let p_step_size = ray.d * segment_length;
        let mut is_ground = false;

        // From camera origin to the outer end of atmosphere
        for _i in 0..NUM_PRIMARY_STEPS {
            let height = P.length() - EARTH_RADIUS;

            // Accidentally generates not-so-bad fake ground if let it contiue.
            if height < 0.0 {
                is_ground = true;
                //break;
            }

            optical_depth += segment_length * (BetaR * (-height / Hr).exp());
            optical_depth += segment_length * (BetaM * (-height / Hm).exp());

            // Single scattering
            let ray2 = Ray::new(P, -sun_dir);
            let hit2 = self.atmosphere.intersect(ray2);
            let light_segment_length = hit2.ray_t / (NUM_SECONDARY_STEPS as f32);
            let PL_step_size = ray2.d * light_segment_length;
            let mut PL = P;

            let mut TL = Vec3::zero();
            let mut apply_scattering = true;
            // From atmosphere boundary to the Sun
            for _j in 0..NUM_SECONDARY_STEPS {
                let height2 = PL.length() - EARTH_RADIUS;
                if height2 < 0.0 {
                    apply_scattering = false;
                    break;
                }

                TL += light_segment_length * BetaR * (-height2 / Hr).exp();
                TL += light_segment_length * BetaM * (-height2 / Hm).exp();

                PL += PL_step_size;
            }

            if apply_scattering {
                TL = (-TL).exp();
                let curr_t = (-optical_depth).exp();

                let mut single_scattering = Vec3::zero();
                single_scattering += MAGIC_RAYLEIGH * segment_length * curr_t * (BetaR * (-height / Hr).exp()) * phaseR(mu) * (TL * sun_intensity);
                single_scattering += MAGIC_MIE * segment_length * curr_t * (BetaM * (-height / Hm).exp()) * phaseM(mu) * (TL * sun_intensity);
                
                result += single_scattering;
            }

            P += p_step_size;
        }

        if is_ground {
            //return Vec3::zero();
        }

        let T = (-optical_depth).exp();

        // Zero scattering
        let L0 = T * get_sun_image(ray, sun_dir, sun_size, sun_intensity);
        result += L0;

        result
    }
}
