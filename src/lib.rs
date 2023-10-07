mod bivariate;
mod rgb;
mod triangle;

use anyhow::Result;
use bivariate::Bivariate;
use clap::Parser;
use image::{io::Reader, RgbImage};
use imageproc::drawing;
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::path::PathBuf;
use triangle::Triangle;

#[derive(Parser)]
pub struct Cli {
    pub path: PathBuf,
}

pub fn run(cli: Cli) -> Result<()> {
    let image = Reader::open(cli.path)?.decode()?;

    // Generate random triangle
    let distribution = Bivariate::new(
        Uniform::new_inclusive(0, image.width() as i32),
        Uniform::new_inclusive(0, image.height() as i32),
    );
    let mut rng = rand::thread_rng();
    let triangle: Triangle<_> = distribution.sample(&mut rng);

    // Draw on new image
    let mut canvas = RgbImage::from_pixel(image.width(), image.height(), image::Rgb([255; 3]));
    drawing::draw_polygon_mut(
        &mut canvas,
        &triangle.into_inner(),
        rng.gen::<rgb::Rgb<_>>().into_inner(),
    );
    canvas.save("output.png")?;

    Ok(())
}
