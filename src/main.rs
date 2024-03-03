// Author: José Luiz
// GitHub: juninhoojl

use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Statistics {
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    mean_x: f64,
    mean_y: f64,
    deviation_x: Vec<f64>,
    deviation_y: Vec<f64>,
    deviation_squared_x: Vec<f64>,
    deviation_squared_y: Vec<f64>,
    product_deviation_scores: Vec<f64>,
    pearson_coefficient: f64,
}

#[derive(Debug)]
struct Value {
    x: f64,
    y: f64,
}

// Function to calculate statistics and Pearson Coefficient from values
fn pearson_coefficient(values: &Vec<Value>) -> Statistics {
    // Calculate the mean of x and y
    let mean_x: f64 = values.iter().map(|v: &Value| v.x).sum::<f64>() / values.len() as f64;
    let mean_y: f64 = values.iter().map(|v: &Value| v.y).sum::<f64>() / values.len() as f64;

    // Calculate the terms needed for the Pearson coefficient
    let mut numerator: f64 = 0.0;
    let mut denominator_x: f64 = 0.0;
    let mut denominator_y: f64 = 0.0;

    for v in values {
        numerator += (v.x - mean_x) * (v.y - mean_y);
        denominator_x += (v.x - mean_x).powi(2);
        denominator_y += (v.y - mean_y).powi(2);
    }

    // Calculate the Pearson coefficient
    let coefficient: f64 = numerator / (denominator_x.sqrt() * denominator_y.sqrt());

    // Additional statistics
    let x_values: Vec<f64> = values.iter().map(|v| v.x).collect();
    let y_values: Vec<f64> = values.iter().map(|v| v.y).collect();
    let deviation_x: Vec<f64> = x_values.iter().map(|&xi| xi - mean_x).collect();
    let deviation_y: Vec<f64> = y_values.iter().map(|&yi| yi - mean_y).collect();
    let deviation_squared_x: Vec<f64> = deviation_x.iter().map(|&di| di.powi(2)).collect();
    let deviation_squared_y: Vec<f64> = deviation_y.iter().map(|&di| di.powi(2)).collect();
    let product_deviation_scores: Vec<f64> = deviation_x.iter().zip(deviation_y.iter()).map(|(&xi, &yi)| xi * yi).collect();

    Statistics {
        x_values,
        y_values,
        mean_x,
        mean_y,
        deviation_x,
        deviation_y,
        deviation_squared_x,
        deviation_squared_y,
        product_deviation_scores,
        pearson_coefficient: coefficient,
    }
}


// Function to read values from a CSV file, handling the header if it exists
fn read_values_from_file(file_path: &str) -> io::Result<Vec<Value>> {
    let mut values: Vec<Value> = Vec::new();

    // Open the file
    if let Ok(file) = File::open(file_path) {
        let reader: io::BufReader<File> = io::BufReader::new(file);

        // Iterate over the lines of the file
        for (index, line) in reader.lines().enumerate() {
            let line: String = line?;

            // Skip the header if it doesn't contain valid data
            if !has_valid_data(&line) {
                continue;
            }

            // Split the line into values separated by a comma
            let values_str: Vec<&str> = line.split(',').collect();

            // Convert the values to floating-point numbers and create an instance of Value
            if let (Ok(x), Ok(y)) = (values_str[0].parse::<f64>(), values_str[1].parse::<f64>()) {
                values.push(Value { x, y });
            } else {
                eprintln!("Error reading line {}: Invalid format.", index + 1);
            }
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found."));
    }
    Ok(values)
}

// Function to check if a line contains valid data (two floating-point values)
fn has_valid_data(line: &str) -> bool {
    let values_str: Vec<&str> = line.split(',').collect();
    values_str.len() == 2
        && values_str[0].trim().parse::<f64>().is_ok()
        && values_str[1].trim().parse::<f64>().is_ok()
}

fn main() {
    // Grabbing arguments
    let args: Vec<String> = env::args().collect();

    // Check if the argument was given
    if args.len() != 2 {
        eprintln!("Usage: {} <csv_file_path>", args[0]);
        std::process::exit(1);
    }

    // CSV Path
    let file_path: &String = &args[1];

    // Check if the file path is empty
    if file_path.is_empty() {
        eprintln!("File path cannot be empty.");
        std::process::exit(1);
    }

    // Call the function to read values from the file
    match read_values_from_file(file_path) {
        Ok(values) => {
            // Block executed if reading is successful (Ok)
            let statistics: Statistics = pearson_coefficient(&values);
            // Print the Pearson Correlation Coefficient
            println!("Pearson Correlation Coefficient: {:.4}", statistics.pearson_coefficient);
            // Print the statistics using the struct
            println!("Statistics:");
            println!("  X Values: {:?}", statistics.x_values);
            println!("  Y Values: {:?}", statistics.y_values);
            println!("  Mean of X Values (Mx): {:.4}", statistics.mean_x);
            println!("  Mean of Y Values (My): {:.4}", statistics.mean_y);
            println!("  Deviation Scores (X - Mx): {:?}", statistics.deviation_x);
            println!("  Deviation Scores (Y - My): {:?}", statistics.deviation_y);
            println!("  Deviation Squared ((X - Mx)^2): {:?}", statistics.deviation_squared_x);
            println!("  Deviation Squared ((Y - My)^2): {:?}", statistics.deviation_squared_y);
            println!("  Product of Deviation Scores ((X - Mx)(Y - My)): {:?}", statistics.product_deviation_scores);
        }
        Err(e) => {
            // Block executed if there is an error during reading (Err)
            eprintln!("Error: {}", e);
        }
    }
}
