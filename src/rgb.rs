use rand::distributions::{Distribution, Standard};

pub struct Rgb<T>(image::Rgb<T>);

impl<T> Rgb<T> {
    pub fn new(rgb: image::Rgb<T>) -> Self {
        Rgb(rgb)
    }

    pub fn into_inner(self) -> image::Rgb<T> {
        self.0
    }
}

impl<T> Distribution<Rgb<T>> for Standard
where
    Standard: Distribution<[T; 3]>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Rgb<T> {
        Rgb(image::Rgb(rng.gen()))
    }
}
