# Pearson Correlation Coefficient Calculator and Statistics

This Rust program reads values from a CSV file, calculates various statistics, and computes the Pearson Correlation Coefficient.

## Author

**José Luiz**
GitHub: [juninhoojl](https://github.com/juninhoojl)

## Usage

Ensure you have Rust installed, then run the program with the following command:

```bash
cargo run --release <csv_file_path>
```

Replace `<csv_file_path>` with the path to your CSV file.

## How to Run

1. Clone the repository:

    ```bash
    git clone https://github.com/juninhoojl/pearson-correlation-coefficient
    ```

2. Change into the project directory:

    ```bash
    cd pearson-correlation-coefficient
    ```

3. Run the program:

    ```bash
    cargo run --release <csv_file_path>
    ```

## Example

data.csv:

```plaintext
x,y
1.2,2.0
2.2,4.0
3.5,6.0
4.1,8.0
5.0,10.0
```

```bash
cargo run --release data.csv
```


### Output

After execution, the program will print the Pearson Correlation Coefficient and various statistics derived from the input values.

```plaintext
Pearson Correlation Coefficient: 0.9937
Statistics:
  X Values: [1.2, 2.2, 3.5, 4.1, 5.0]
  Y Values: [2.0, 4.0, 6.0, 8.0, 10.0]
  Mean of X Values (Mx): 3.2000
  Mean of Y Values (My): 6.0000
  Deviation Scores (X - Mx): [-2.0, -1.0, 0.2999999999999998, 0.8999999999999995, 1.7999999999999998]
  Deviation Scores (Y - My): [-4.0, -2.0, 0.0, 2.0, 4.0]
  Deviation Squared ((X - Mx)^2): [4.0, 1.0, 0.0899999999999999, 0.809999999999999, 3.2399999999999993]
  Deviation Squared ((Y - My)^2): [16.0, 4.0, 0.0, 4.0, 16.0]
  Product of Deviation Scores ((X - Mx)(Y - My)): [8.0, 2.0, 0.0, 1.799999999999999, 7.199999999999999]
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
