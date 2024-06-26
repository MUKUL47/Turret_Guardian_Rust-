use ggez::graphics::Color;
use rand::Rng;

pub type Pointf64 = (f64, f64);
pub type Pointf32 = (f32, f32);
pub fn get_rotation_angle(
    origin: &Pointf32,
    position: &Pointf32,
    angle: f32,
    distance: Option<f32>,
) -> Pointf32 {
    let d: f32 = match distance {
        None => euclidean_distance(origin, position),
        Some(v) => v,
    };
    let (o1, o2) = *origin;
    let (c_x, c_y) = ((d * angle.cos() as f32), d * angle.sin() as f32);
    return ((c_x + o1 as f32), c_y + o2 as f32);
}

pub fn euclidean_distance(p1: &Pointf32, p2: &Pointf32) -> f32 {
    f32::sqrt(f32::powi(f32::abs(p1.0 - p2.0), 2) + f32::powi(f32::abs(p1.1 - p2.1), 2))
}

pub fn angle_btw_2_points(p1: &Pointf32, p2: &Pointf32) -> f32 {
    (p2.1 - p1.1).atan2(p2.0 - p1.0)
}

pub fn get_rand_color() -> Color {
    let mut rand = rand::thread_rng();
    let (r, g, b) = (
        rand.gen_range(0..256),
        rand.gen_range(0..256),
        rand.gen_range(0..256),
    );
    return Color::from_rgb(r as u8, g as u8, b as u8);
}
