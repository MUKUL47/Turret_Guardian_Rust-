use std::sync::Mutex;

lazy_static! {
    pub static ref WINDOW_HEIGHT: Mutex<f32> = Mutex::new(800.);
    pub static ref WINDOW_WIDTH: Mutex<f32> = Mutex::new(800.);
}
