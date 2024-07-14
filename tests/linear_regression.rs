// tests/linear_regression.rs

#[cfg(test)]
mod tests {
    use ling::linear_regression::LinearRegression;

    #[test]
    fn test_linear_regression() {
        // Initialize your linear regression model
        let mut model = LinearRegression::new();

        // Example input data (replace with your actual data)
        let x_train = vec![1.0, 2.0, 3.0, 4.0, 5.0];  // Example feature values
        let y_train = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Corresponding expected output values

        // Perform fitting (train the model)
        let _ = model.fit(&x_train, &y_train);

        // Example input value for prediction
        let input_value = 6.0; // Replace with your actual input value

        // Predict based on the input value
        let predicted_value = model.predict(input_value);

        // Example expected value (assertion)
        let expected_value = 12.0; // Replace with your expected value based on your model

        // Assert the predicted value matches the expected value
        assert_eq!(predicted_value, expected_value);
    }
}

