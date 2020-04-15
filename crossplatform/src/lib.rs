use std::path::Path;

pub trait Model {
    fn path() -> &'static Path;
    fn name() -> &'static str;
}


pub struct Circle {
    pub r: f32,
}

impl Model for Circle {
    fn path() -> &'static Path { Path::new("circle/target/release") }
    fn name() -> &'static str { "circle" }
}


pub struct Square {
    pub s: f32,
}

impl Model for Square {
    fn path() -> &'static Path { Path::new("square/target/release") }
    fn name() -> &'static str { "square" }
}
