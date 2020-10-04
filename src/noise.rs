use crate::vec3::*;

// Noise functions from https://www.shadertoy.com/view/4dS3Wd

// fract_glsl(-1.57) = -1.57 - (-2) = 0.43
// fract(-1.57) = -0.57
fn fract_glsl(x: f32) -> f32 { x - x.floor() }

fn hash(p: f32) -> f32 {
    let mut p = fract_glsl(p * 0.011);
    p *= p + 7.5;
    p *= p + p;
    fract_glsl(p)
}
pub fn clamp(x: f32, x_min: f32, x_max: f32) -> f32 {
    if x < x_min {
        x_min
    } else if x > x_max {
        x_max
    } else {
        x
    }
}
pub fn noise(x: Vec3) -> f32 {
	let step = vec3(110.0, 241.0, 171.0);
	let i = x.floor();
	let f = x.fract_glsl();
    let n = i & step;
    
	let u = f * f * (vec3(3.0, 3.0, 3.0) - 2.0 * f);
	return lerp(lerp(lerp( hash(n + (step & vec3(0.0, 0.0, 0.0))), hash(n + (step & vec3(1.0, 0.0, 0.0))), u.x),
                   lerp( hash(n + (step & vec3(0.0, 1.0, 0.0))), hash(n + (step & vec3(1.0, 1.0, 0.0))), u.x), u.y),
               lerp(lerp( hash(n + (step & vec3(0.0, 0.0, 1.0))), hash(n + (step & vec3(1.0, 0.0, 1.0))), u.x),
                   lerp( hash(n + (step & vec3(0.0, 1.0, 1.0))), hash(n + (step & vec3(1.0, 1.0, 1.0))), u.x), u.y), u.z);
}

/*
// default lacunarity = 1.92
#[allow(non_snake_case)]
pub fn fBm(p: Vec3, octaves: i32, octave_gain: f32, lacunarity: f32) -> f32 {
    let mut p2 = p;
    let mut a = 0.5;
    let mut value = 0.0;
    for _i in 0..octaves {
        value += noise(p2) * a;
        p2 *= lacunarity;
        a *= octave_gain;
    }

    value
}
*/

///*
#[allow(non_snake_case)]
pub fn fBm(x0: Vec3) -> f32 {
    let mut x = x0;
	let mut v: f32 = 0.0;
	let mut a: f32 = 0.5;
    let shift = vec3(100.0, 100.0, 100.0);
    for _i in 0..5 {
		v += a * noise(x);
		x = x * 2.0 + shift;
		a *= 0.5;
    }
    
	return v
}
//*/

pub fn pyroclastic(distance: f32, noise: f32, filter_width: f32) -> f32 {
    let width = filter_width * 0.5;
    let pyro_value = fit(distance - noise, -width, width, 1.0, 0.0);

    clamp(pyro_value, 0.0, 1.0)
}
