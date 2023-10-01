use std::{array, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use image::{io::Reader, GenericImageView};
use imageproc::point::Point;
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

struct Bivariate<P, Q>(P, Q);

#[derive(Debug)]
struct Triangle<T>([Point<T>; 3]);

impl<T, P, Q> Distribution<Triangle<T>> for Bivariate<P, Q>
where
    P: Distribution<T>,
    Q: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triangle<T> {
        Triangle(array::from_fn(|_| Point {
            x: self.0.sample(rng),
            y: self.1.sample(rng),
        }))
    }
}

#[derive(Parser)]
pub struct Cli {
    pub path: PathBuf,
}

pub fn run(cli: Cli) -> Result<()> {
    let image = Reader::open(cli.path)?.decode()?;
    let x_uniform = Uniform::new_inclusive(0, image.width());
    let y_uniform = Uniform::new_inclusive(0, image.height());
    let distribution = Bivariate(x_uniform, y_uniform);
    let mut rng = rand::thread_rng();
    let triangle: Triangle<u32> = distribution.sample(&mut rng);
    println!("{:?}", image.dimensions());
    println!("{:?}", triangle);
    Ok(())
}
