# SHA9 Encryption Tool

## Description

This Rust program is designed to encrypt or decrypt text files using a custom encryption algorithm named **SHA9**. It employs a letter-shifting method in the alphabet with a variable shifting logic to transform the file's content.

## Features

- **Encryption**: Encrypts the content of a text file using the **SHA9** algorithm.
- **Decryption**: Decrypts a text file previously encrypted by this program.

## Installation

Ensure you have Rust and Cargo installed on your system. You can download and install them from [the official Rust website](https://www.rust-lang.org/tools/install).

Then clone this repository using Git and navigate to the cloned project directory:

```
git clone https://github.com/0xenj/mini_project_rust.git
cd ./encrypt
```

## Usage

To use the program, navigate to the project directory and use Cargo to run it. You need to specify the path of the file to process and the mode (encryption or decryption).

### Basic Command

`cargo run -- [FILE_NAME] [MODE]`

- [FILE_NAME]: The path to the file you want to encrypt or decrypt.
- [MODE]: Specify ***true*** for encryption or ***false*** for decryption.

### Examples

To **encrypt** a file named `secret.txt`:

`cargo run -- secret.txt true`

To **decrypt** the same file:

`cargo run -- secret.txt false`

## Important Notes

- Using this program will replace the content of the original file. Ensure to back up important data before proceeding with encryption or decryption.
- This tool is intended for educational and experimental purposes and should not be used for the security of critical information.

