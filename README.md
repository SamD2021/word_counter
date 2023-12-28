# Word Counter

**Version:** 1.0  
**Author:** Samuel Dasilva

## Overview

The Word Counter is a simple command-line application that counts the number of words in a text file.

## Features

- Counts the number of words in a given text file.
- Easy to use with command-line arguments.
- Built with Rust using the Clap library.

## Usage

### Installation

1. Ensure you have Rust installed. If not, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the repository or download the source code.

3. Navigate to the project directory in the terminal.

4. Build the application:

    ```bash
    cargo build --release
    ```

5. Run the application:

    ```bash
    ./target/release/word_counter <file_path>
    ```

### Command-Line Options

- `<file_path>`: The path to the text file for which you want to count words.

### Example

```bash
./target/release/word_counter path/to/your/file.txt
