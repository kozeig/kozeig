# Lüt Programming Language

![Lut](https://img.shields.io/badge/language-lut-blue)
![Status](https://img.shields.io/badge/status-alpha-orange)
![License](https://img.shields.io/badge/license-MIT-green)

Lüt (pronounced "loot" or "lewt") is a fun, simple programming language with a clean syntax designed for readability and ease of use. It's an experimental language under active development.

> 26.3% of this codebase is AI Generated; includes .md files, examples and source code for Lüt

## Project Status

⚠️

Lüt is currently in early development. While the core features work, you may encounter bugs and limitations. The compiler and runtime are being actively improved.

**Lüt is under active development and is open to community contributions in any form!**

## Features

- **Simple, Clean Syntax**: Easy to read and write
- **Compiled & Interpreted**: Run with `lut run` or compile with `lut build` designed to bridge the gap between languages like Python and C
- **Cross-Platform**: Produces standalone executables that work on multiple platforms
- **Beginner-Friendly**: Ideal for learning programming concepts

## Quick Start

### Hello World

```lut
@@ Hello World in Lut
greeting : -text 'Hello, World!'
-print $greeting
```

### Running a Program

```bash
# Interpret and run
lut run hello.lut

# Compile to executable
lut build hello.lut
./hello
```

## Syntax Overview

Lut has a simple syntax that's easy to learn:

- **Comments** start with `@@`
- **Variables** are defined with `name : -type value`
- **Commands** start with a hyphen `-` (like `-print`)
- **Variable references** use a dollar sign `$variableName`

### Basic Data Types

- **Numbers**: `age : -number 42`
- **Text**: `name : -text 'John Doe'`

### Arithmetic Operations

```lut
a : -number 10
b : -number 5

sum : -add $a, $b
diff : -sub $a, $b
product : -mul $a, $b
quotient : -div $a, $b
remainder : -mod $a, $b

-print -text 'Sum: ', $sum
```

### ASCII Conversion

```lut
char : -asc 65  @@ Converts ASCII code 65 to 'A'
-print $char
```

For a complete syntax reference, see [SYNTAX.md](SYNTAX.md).

## Installation

Currently, you need to build from source:

```bash
# Clone the repository
git clone https://github.com/frgmt0/lut.git
cd lut

# Build with Cargo
cargo build --release

# You can add the binary to your PATH or run it directly
```

## Known Issues

Lüt is still in early development and has several known issues:

- The memory management in compiled programs may cause issues in complex cases
- Subtraction operations in compiled mode can be unreliable
- Limited error reporting and debugging capabilities

If you encounter issues, please file a bug report so the community can investigate and fix the problem.

## Roadmap

- **Improved Error Messages**: Better diagnostics and debugging
- **More Data Types**: Boolean, lists, maps
- **Control Flow**: Conditionals and loops
- **Functions & Modules**: Code organization
- **Standard Library**: Common utilities

## Contributing

Lüt is open to community contributions in any form! Whether you're fixing bugs, improving documentation, or adding features, your help is appreciated.

### Ways to Contribute

- **Code**: Implement new features or fix bugs
- **Documentation**: Improve guides or add examples
- **Testing**: Write test cases or find bugs
- **Ideas**: Suggest new language features

### Reporting Issues

If you find bugs or have suggestions, please open an issue on GitHub. Include:

1. A description of the issue
2. Steps to reproduce
3. Expected vs. actual behavior
4. Any relevant code examples

## License

Lüt is released under the MIT License. See the LICENSE file for details.

## Acknowledgments

Lüt was inspired by languages like Python, Lua, and Rust, with a goal of creating a simple, fun language for learning and experimenting.

---

Happy coding with Lüt!