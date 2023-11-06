use std::io::Write;
use std::path::PathBuf;

use anyhow::{anyhow, Error};
use charts_rs::svg_to_png;
use directories::BaseDirs;
use itertools::Itertools;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use rand::{thread_rng, Rng};
use rand_xorshift::XorShiftRng;
use rand_xoshiro::{rand_core::SeedableRng, Xoshiro256PlusPlus};

fn plot(full_path: &PathBuf, points: &Vec<u64>) -> Result<(), Error> {
    let max_y = points.iter().max().unwrap();
    let mut current_fitness: i64 = *max_y as i64 / 2;

    // Start with turning our random sequence into a vec of tuples of x, y
    let time_series: Plot = Plot::new(
        points
            .iter()
            .enumerate()
            .map(|x| (x.0 as f64, *x.1 as f64))
            .collect::<Vec<_>>(),
    )
    .point_style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355") // and a custom colour
            .size(3.),
    );

    let mut last_val = points[0] as i64;

    // Now plot the fitness indicator
    let fitness_indicator: Plot = Plot::new(
        points
            .iter()
            .map(|val| {
                let val = *val as i64;
                if val != last_val {
                    current_fitness += (val - last_val) / (val - last_val).abs()
                };
                last_val = val;
                current_fitness
            })
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .map(|x| (x.0 as f64, *x.1 as f64))
            .collect::<Vec<_>>(),
    )
    .point_style(
        PointStyle::new() // uses the default marker
            .colour("#35C788")
            .size(4.),
    ); // and a different colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(time_series)
        .add(fitness_indicator)
        .x_range(0., points.len() as f64)
        .y_range(0., *max_y as f64)
        .x_label("Time")
        .y_label("Value");

    // A page with a single view is then saved to an PNG file
    let png_path = full_path.with_extension("png");

    let mut file = std::fs::File::create(png_path)?;
    file.write_all(
        &svg_to_png(
            &Page::single(&v)
                .dimensions(1920, 1080)
                .to_svg()
                .unwrap()
                .to_string(),
        )
        .unwrap(),
    )?;

    Ok(())
}

fn sequence_rng_plot(base_path: &PathBuf, range: u64, point_count: u64) -> Result<(), Error> {
    let base_vec = (0..range).collect_vec();

    let mut point_vec: Vec<u64> = Vec::new();

    while point_vec.len() < point_count as usize {
        point_vec.extend(base_vec.iter());
    }

    let mut final_path = base_path.clone();
    final_path.push(format!("sequence_{}_{}_rng", range, point_count));

    plot(&final_path, &point_vec)
}

fn standard_rng_plot(base_path: &PathBuf, range: u64, point_count: u64) -> Result<(), Error> {
    let mut rng = thread_rng();

    let point_vec = (0..point_count)
        .map(|_| rng.gen_range(0..range))
        .collect_vec();

    let mut final_path = base_path.clone();
    final_path.push(format!("standard_{}_{}_rng", range, point_count));

    plot(&final_path, &point_vec)
}

fn xorshift_rng_plot(base_path: &PathBuf, range: u64, point_count: u64) -> Result<(), Error> {
    let mut rng = XorShiftRng::from_entropy();

    let point_vec = (0..point_count)
        .map(|_| rng.gen_range(0..range))
        .collect_vec();

    let mut final_path = base_path.clone();
    final_path.push(format!("xorshift_{}_{}_rng", range, point_count));

    plot(&final_path, &point_vec)
}

fn xoshiro256plusplus_rng_plot(
    base_path: &PathBuf,
    range: u64,
    point_count: u64,
) -> Result<(), Error> {
    let mut rng = Xoshiro256PlusPlus::from_entropy();

    let point_vec = (0..point_count)
        .map(|_| rng.gen_range(0..range))
        .collect_vec();

    let mut final_path = base_path.clone();
    final_path.push(format!("xoshiro256plusplus_{}_{}_rng", range, point_count));

    plot(&final_path, &point_vec)
}

fn run() -> Result<(), Error> {
    if let Some(base_dirs) = BaseDirs::new() {
        let base_path = PathBuf::from(base_dirs.home_dir());
        let point_count = 10000;
        let range = 10000;

        sequence_rng_plot(&base_path, range, point_count)?;
        standard_rng_plot(&base_path, range, point_count)?;
        xorshift_rng_plot(&base_path, range, point_count)?;
        xoshiro256plusplus_rng_plot(&base_path, range, point_count)?;
    } else {
        return Err(anyhow!("Unable to determine base dirs"));
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
