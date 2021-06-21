use crate::light::Light;
use crate::volume::Volume;

pub struct Scene {
    pub volume: Box<dyn Volume>,
    pub lights: Vec<Box<dyn Light>>
}
