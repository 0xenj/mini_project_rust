# Coin Total Calculator

## Description

This Rust program is a simple tool designed to calculate the total value of a collection of coins. Users enter the number of each type of coin, and the program calculates and displays the total value in euros.

## Features

- **Interactive Input**: The program prompts the user to input the number of each type of coin (2€, 1€, 50c, 20c, 10c, and 5c).
- **Total Calculation**: After receiving the input for each coin type, the program calculates the total value in euros.
- **Unit Testing**: Includes a basic unit test to ensure the functionality of the total calculation.

## Installation

Ensure you have Rust and Cargo installed on your system. You can download and install them from [the official Rust website](https://www.rust-lang.org/tools/install).

Clone this repository using Git and navigate to the cloned project directory:

```
git clone https://github.com/0xenj/mini_project_rust.git
cd ./lil_bank
```

## Usage

To use the program, navigate to the project directory and use Cargo to run it.

### Command to Run the Program

`cargo run`

The program will then prompt you to enter the number of each type of coin. Enter the numbers as requested.

### Example of Program Interaction

```
Enter number of coins:
2€: [Enter number of 2€ coins]
1€: [Enter number of 1€ coins]
50c: [Enter number of 50c coins]
20c: [Enter number of 20c coins]
10c: [Enter number of 10c coins]
5c: [Enter number of 5c coins]

Total: [Total value in euros]
```

## Unit Testing

`cargo test`

The test will verify the functionality of the total calculation.

## Important Notes

- The program is designed for simple calculations and does not handle input validation extensively.
- It is intended for educational and experimental purposes.
