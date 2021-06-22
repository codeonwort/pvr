pub mod math;
pub mod light;
pub mod phasefn;
pub mod camera;
pub mod scene;
pub mod voxelbuffer;
pub mod primitive;
pub mod volume;
pub mod render;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
