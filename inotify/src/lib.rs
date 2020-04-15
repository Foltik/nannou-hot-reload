use std::path::Path;

pub trait Model {
    fn path() -> &'static Path;
}


pub struct Circle {
    pub r: f32,
}

impl Model for Circle {
    fn path() -> &'static Path { Path::new("circle/target/release/libcircle.so") }
}


pub struct Square {
    pub s: f32,
}

impl Model for Square {
    fn path() -> &'static Path { Path::new("square/target/release/libsquare.so") }
}
