# Java Class File Parser in Rust

This Rust project, `rusvm` is a Java class file parser capable of extracting information about classes, methods, attributes, and more from Java bytecode files (.class). It aims to provide a comprehensive tool for analyzing Java class files in Rust.

## Features

- Parses Java class files to extract metadata and information about classes, methods, fields, and attributes.
- Supports various constant pool types, including class references, method references, field references, strings, integers, floats, doubles, UTF-8 strings, and more.
- Handles class access flags and method access flags according to the Java Virtual Machine Specification.
- Parses attributes such as Code, LineNumberTable, and SourceFile.
- Provides a flexible and extensible structure for further analysis or processing of Java class files.

## Usage

To use `rusvm`, follow these steps:

1. Ensure you have Rust installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).

2. Clone this repository to your local machine:

   ```shell
   git clone https://github.com/edilson258/rusvm.git
   cd rusvm
   cargo build
   cargo run samples/Main.class
   ```
Note: Replace samples/Main.class with the path for you class file

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions to `rusvm` are welcome! If you encounter any issues, have suggestions for improvements, or want to contribute new features, feel free to open an issue or submit a pull request on GitHub at [rusvm](https://github.com/yourusername/rusvm).
