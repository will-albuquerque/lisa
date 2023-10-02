use imageproc::point::Point;

#[derive(Debug)]
pub struct Triangle<T>([Point<T>; 3]);

impl<T> Triangle<T> {
    pub fn new(vertices: [Point<T>; 3]) -> Self {
        Self(vertices)
    }

    pub fn into_inner(self) -> [Point<T>; 3] {
        self.0
    }
}
