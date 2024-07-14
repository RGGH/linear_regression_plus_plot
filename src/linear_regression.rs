pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}

impl LinearRegression {
    pub fn new() -> LinearRegression {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }

    pub fn fit(&mut self, input: &[f64], output: &[f64]) -> Result<(), String> {
        if input.len() != output.len() {
            return Err("The number of input and output values is different".to_string());
        }
        let size_of_input = input.len() as f64;
        let sum_of_input: f64 = input.iter().sum();
        let sum_of_output: f64 = output.iter().sum();
        let sum_of_input_output: f64 = input.iter().zip(output.iter()).map(|(&x, &y)| x * y).sum();
        let square_sum_input: f64 = input.iter().map(|&x| x * x).sum();
        self.slope = (size_of_input * sum_of_input_output - sum_of_input * sum_of_output)
            / (size_of_input * square_sum_input - sum_of_input * sum_of_input);
        self.intercept = (sum_of_output * square_sum_input - sum_of_input * sum_of_input_output)
            / (size_of_input * square_sum_input - sum_of_input * sum_of_input);
        Ok(())
    }

    pub fn predict(&self, input: f64) -> f64 {
        self.slope * input + self.intercept
    }
}

