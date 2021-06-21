// #todo-pvrlib: Move to pvrlib
use pvrlib::light::Light;
use pvrlib::volume::Volume;

pub struct Scene {
    pub volume: Box<dyn Volume>,
    pub lights: Vec<Box<dyn Light>>
}
