use crate::math::vec3::*;

// https://github.com/TheRealMJP/BakingLab/blob/master/BakingLab/ACES.hlsl

// #todo: Move to math module
#[allow(non_camel_case_types)]
struct float3x3 {
    pub m: [[f32; 3]; 3]
}

// sRGB => XYZ => D65_2_D60 => AP1 => RRT_SAT
const ACES_INPUT_MAT: float3x3 = float3x3 {
    m: [
        [0.59719, 0.35458, 0.04823],
        [0.07600, 0.90834, 0.01566],
        [0.02840, 0.13383, 0.83777]
    ]
};

// ODT_SAT => XYZ => D60_2_D65 => sRGB
const ACES_OUTPUT_MAT: float3x3 = float3x3 {
    m: [
        [1.60475, -0.53108, -0.07367],
        [-0.10208,  1.10813, -0.00605],
        [-0.00327, -0.07276,  1.07602]
    ]
};

fn mul(mat: float3x3, v: vec3f) -> vec3f {
    let row0 = vec3(mat.m[0][0], mat.m[0][1], mat.m[0][2]);
    let row1 = vec3(mat.m[1][0], mat.m[1][1], mat.m[1][2]);
    let row2 = vec3(mat.m[2][0], mat.m[2][1], mat.m[2][2]);

    vec3(row0.dot(v), row1.dot(v), row2.dot(v))
}

#[allow(non_snake_case)]
fn RRTAndODTFit(v: vec3f) -> vec3f {
    let a: vec3f = v * (v + (0.0245786).into()) - (0.000090537).into();
    let b: vec3f = v * (0.983729 * v + (0.4329510).into()) + (0.238081).into();
    return a / b;
}

pub fn aces_tone_mapping(color: vec3f) -> vec3f {
    let mut ret = mul(ACES_INPUT_MAT, color);

    // Apply RRT and ODT
    ret = RRTAndODTFit(ret);

    ret = mul(ACES_OUTPUT_MAT, ret);

    // Clamp to [0, 1]
    ret = vec3f::saturate(ret);

    return ret;
}
