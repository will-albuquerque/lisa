use std::array;

use imageproc::point::Point;
use rand::{distributions::Distribution, Rng};

use crate::triangle::Triangle;

pub struct Bivariate<X, Y>(X, Y);

impl<X, Y> Bivariate<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Bivariate(x, y)
    }
}

impl<T, X, Y> Distribution<Triangle<T>> for Bivariate<X, Y>
where
    X: Distribution<T>,
    Y: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triangle<T> {
        Triangle::new(array::from_fn(|_| Point {
            x: self.0.sample(rng),
            y: self.1.sample(rng),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Standard, rngs::mock::StepRng};

    #[test]
    fn sample_random_triangle() {
        let mut rng = StepRng::new(0, 1);
        let dist = Bivariate::new(Standard, Standard);
        let triangle = dist.sample(&mut rng);
        assert_eq!(
            triangle.into_inner(),
            [
                Point { x: 0, y: 1 },
                Point { x: 2, y: 3 },
                Point { x: 4, y: 5 }
            ]
        );
    }
}
