use crate::light::Light;
use crate::volume::Volume;
use crate::skyatmosphere::SkyAtmosphere;

pub struct Scene {
    pub volume: Box<dyn Volume>,
    pub lights: Vec<Box<dyn Light>>,
    pub sky_atmosphere: SkyAtmosphere
}
