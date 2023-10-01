use std::{array, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use image::{io::Reader, Rgb, RgbImage};
use imageproc::{drawing, point::Point};
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

    // Generate random triangle
    let distribution = Bivariate(
        Uniform::new_inclusive(0, image.width() as i32),
        Uniform::new_inclusive(0, image.height() as i32),
    );
    let mut rng = rand::thread_rng();
    let triangle: Triangle<_> = distribution.sample(&mut rng);

    // Draw on new image
    let mut canvas = RgbImage::from_pixel(image.width(), image.height(), Rgb([255; 3]));
    drawing::draw_polygon_mut(&mut canvas, triangle.0.as_slice(), Rgb([0; 3]));
    canvas.save("output.png")?;

    Ok(())
}
