use std::error::Error;

mod linear_regression;
mod plotting;

use linear_regression::LinearRegression;
use plotting::draw_chart;

fn main() -> Result<(), Box<dyn Error>> {
    let mut linear_regression_model = LinearRegression::new();

    let fahrenheit = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let celsius = vec![-17.22, -16.67, -16.11, -15.56, -15.0, -14.44, -13.89, -13.33, -12.78];

    linear_regression_model.fit(&fahrenheit, &celsius)?;

    let input_value: f64 = {
        use std::io::{self, Write};
        print!("Enter a Fahrenheit value: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        buf.trim().parse().unwrap_or(0.0)
    };

    let predicted_celsius = linear_regression_model.predict(input_value);
    println!(
        "Predicted Celsius at {} Fahrenheit: {:.2}",
        input_value, predicted_celsius
    );

    draw_chart(&linear_regression_model)?;

    Ok(())
}

