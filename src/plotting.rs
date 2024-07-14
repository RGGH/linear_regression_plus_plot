// src/plotting.rs
use plotters::prelude::*;
use plotters::style::Palette99;
use std::error::Error;

use crate::linear_regression::LinearRegression;



#[allow(dead_code)]
pub fn plot_linear_regression() {
    // Implementation of plotting function
    println!("Plotting linear regression...");
}



pub fn draw_chart(
    linear_regression_model: &LinearRegression,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Linear Regression Model", ("sans-serif", 20))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..100.0, -20.0..40.0)?;

    chart.configure_mesh().draw()?;

    // Prepare data points for plotting
    let data_points: Vec<(f64, f64)> = (0..=100)
        .map(|x| (x as f64, linear_regression_model.predict(x as f64)))
        .collect();

    chart
        .draw_series(LineSeries::new(
            data_points.iter().map(|(x, y)| (*x, *y)),
            &Palette99::pick(0),
        ))?
        .label("Linear Regression")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &Palette99::pick(0)));

    chart.configure_series_labels().draw()?;
    Ok(())
}

