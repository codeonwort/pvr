// #todo-pvrlib: Move to pvrlib
use crate::volume::volume::Volume;
use crate::light::Light;

pub struct Scene {
    pub volume: Box<dyn Volume>,
    pub lights: Vec<Box<dyn Light>>
}
