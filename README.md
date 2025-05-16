# Kozeig

![Kozeig](https://img.shields.io/badge/language-kozeig-blue)
![Status](https://img.shields.io/badge/status-alpha-orange)
![License](https://img.shields.io/badge/license-MIT-green)

Kozeig (pronounced "koh-sig") is a fun, quirky and simple programming language with bold opinions on its syntax and design. However, it's also a practical language that can be (in the near future) used for a wide range of applications.

## Project Status

⚠️

Kozeig is currently in early development. While the core features work, you may encounter bugs and limitations. The compiler and runtime are being actively improved.

**Kozeig is under active development and is open to community contributions in any form!**

See [Benchmarks](./BENCHMARKS.md) for more information on the current performance of the language.
See [Changelog](./CHANGELOG.md) for a list of recent changes.

## Features

- **Simple, Consistent Syntax**: Clean, unified syntax for all block constructs
- **Compiled & Interpreted**: Run with `koze run` or compile with `koze build` designed to bridge the gap between languages like Python and C
- **Native LLVM Compilation**: Generates optimized binaries using the LLVM compiler infrastructure
- **JIT Execution**: Supports Just-In-Time compilation for quick testing with `koze jit` *Note: this feature is experimental and may be removed later depending on feedback, my own personal preference, or if it gets unecessarily difficult to maintain*
- **Cross-Platform**: Produces standalone executables that work on multiple platforms
- **Beginner-Friendly**: Ideal for learning programming concepts
- **Logical Operators**: Full support for AND, OR, and NOT operations
- **Control Flow**: If-else statements and ternary conditional expressions
- **Functions**: Support for defining and calling functions with parameters and return values

## Installation

Currently, you need to build Kozeig from source:

```bash
# Clone the repository
git clone https://github.com/kozeig/kozeig.git
cd koze

# Build with the automated script (it installs LLVM if needed)
./build.sh

# Add the compiler to your PATH or run it directly
```

### Build Requirements

Kozeig requires the following dependencies:

- Rust (latest stable version) - Install from [rust-lang.org](https://www.rust-lang.org/tools/install)
- LLVM 16 - The build script will install this for you if needed
- On macOS: Homebrew (for LLVM installation)
- On Linux: apt package manager (for LLVM installation)

### Build Script

The `build.sh` script handles the following automatically:

1. Detects your operating system (macOS, Linux, or Windows)
2. Checks if LLVM 16 is installed (required for the compiler)
3. Installs LLVM 16 if not found:
   - On macOS: Uses Homebrew to install `llvm@16`
   - On Linux: Uses apt to install LLVM 16 packages
   - On Windows: Provides instructions for manual installation
4. Sets up the required environment variables in your shell profile
5. Compiles Kozeig with optimizations using `cargo build --release`

After running the build script, the compiler will be available at `./target/release/koze`.

### Manual Installation

If the build script doesn't work for your system, you can manually install the requirements:

1. Install LLVM 16 using your package manager or from [LLVM downloads](https://releases.llvm.org/download.html)
2. Set the environment variable: `export LLVM_SYS_160_PREFIX=/path/to/your/llvm`
3. Build with Cargo: `cargo build --release`

## Known Issues and Limitations

Kozeig is still in early development and has several known issues and limitations:

- Memory management for strings in compiled programs may cause issues in complex cases
- Limited error reporting and debugging capabilities
- String concatenation is limited for complex cases

**Arrays - Current Implementation Status:**
- Basic 1D and 2D array creation and display works correctly: `myArray : { array [1, 2, 3, 4, 5] }`
- Mixed type arrays are supported: `mixed : { array [1, 'text', true, 3.14] }`
- Arrays display properly when referenced in print statements
- **Limitations:**
  - Matrix operations (transpose, determinant) are partially implemented in the interpreter but not fully tested
  - Array element access is limited
  - Compiled code for arrays returns a simplified string representation rather than full array functionality

**Other Known Limitations:**
- No support for maps or other complex data structures (COMING SOON THOUGH!)
- Limited standard library functionality (GROWING WITH EACH RELEASE!)
- Type checking is minimal, which can lead to unexpected behavior in some cases

**What Works Well:**
- ✅ **Functions**: Function definitions with parameters, function calls, recursive functions, and return values
- ✅ **Array Creation and Display**: 1D and 2D arrays with the new `[1, 2, 3]` syntax work perfectly
- ✅ **One-Liners**: Statement separation with `;;` works in all contexts including control blocks
- ✅ **Data Types**: All numeric types (integer, floating point, hex, binary) work as expected
- ✅ **Comments**: The new `--` comment syntax is fully implemented and compatible
- ✅ **Syntax**: Unified syntax with consistent curly braces is fully functional
- ✅ **Logic**: Logical operators (AND, OR, NOT) work perfectly
- ✅ **Arithmetic**: All basic operations work correctly with proper type handling
- ✅ **Control Flow**: If/else statements, loops, and break/continue are fully functional
- ✅ **Conditionals**: Boolean expressions, comparisons, and ternary expressions all work reliably
- ✅ **Compiled Code**: Loop variables and control flow work correctly in compiled binaries

If you encounter issues, please file a bug report so the community can investigate and fix the problem.

## Quick Start

### Hello World

```koze
-- Hello World in Kozeig
func pub main {} [
    greeting : { text 'Hello, World!' }
    print { $greeting }

    'ok'  -- Return value similar to 'return 0' in C
]
```

### Running a Program

```bash
# Interpret and run
koze run hello.ko

# Compile to executable
koze build hello.ko
./hello

# JIT compile and execute
koze jit hello.ko

# Debug mode - show tokens and AST
koze debug hello.ko
```

The debug mode is particularly useful for language development and understanding how the parser interprets your code. It displays:
1. All tokens generated by the lexer
2. The complete abstract syntax tree (AST) created by the parser

## Syntax Overview

Kozeig has a simple, consistent syntax that's easy to learn:

- **Comments** start with `--`
- **Variables** are defined with `name : { type value }`
- **Commands** use a name followed by arguments in curly braces: `command { args }`
- **Variable references** use a dollar sign `$variableName`
- **Statement separation** uses double semicolons `;;` for one-liners

### Basic Data Types

- **Numbers**: `age : { number 42 }`
- **Floating Point**: `pi : { fp 3.14159 }`
- **Text**: `name : { text 'John Doe' }`
- **Booleans**: `isActive : true` or `isValid : { bool 1 }`
- **Arrays**: `myArray : { array [1, 2, 3, 4, 5] }` or `matrix : { array [1, 2][3, 4] }`

### Functions

```koze
-- Define a function to add two numbers
func pub add { a : number !, b : number ! } [
    $a + $b  -- Last expression is returned
]

-- Main function is the entry point
func pub main {} [
    -- Call the function
    result : call { add, 5, 7 }
    print { 'The sum is: ', $result }  -- Outputs: The sum is: 12

    'ok'
]
```

### Arithmetic Operations

```koze
a : { number 10 }
b : { number 5 }

sum : $a + $b
diff : $a - $b
product : $a * $b
quotient : $a / $b
remainder : $a % $b

print { 'Sum: ', $sum }
print { 'Product: ', $product }
```

### One-Liners with Statement Separators

```koze
-- Multiple statements on one line using ;; separators
x : 10 ;; y : 20 ;; print { 'Sum: ', $x + $y }

-- Works inside control blocks too
for { i : 0, $i + 1, $i < 3 } [
    value : $i * 2 ;; print { 'i =', $i, 'value =', $value }
]
```

### ASCII Conversion

```koze
char : { asc 65 }  -- Converts ASCII code 65 to 'A'
print { $char }
```

For a complete syntax reference, see [SYNTAX.md](SYNTAX.md).

## Roadmap

### Completed
- ✅ **Functions**: Definition and calling of functions with parameters and return values, including recursive functions
- ✅ **Main Function Entry Point**: Standard format with `func pub main {}` as program entry
- ✅ **Core Arithmetic Operations**: Addition, subtraction, multiplication, division, and modulo
- ✅ **Basic Control Flow**: If/else conditionals and boolean operations
- ✅ **Native Compilation**: LLVM-based compiler with JIT support
- ✅ **Logical Operators**: AND, OR, and NOT operations
- ✅ **Unified Syntax**: Consistent syntax with curly braces for all code blocks
- ✅ **Ternary Expressions**: Conditional expressions with the `?:` operator
- ✅ **Loop Structures**: While and for loops with proper variable updating
- ✅ **Loop Control Flow**: Support for break and continue statements
- ✅ **Arrays**: Basic 1D and 2D array support with array literals and display capabilities
- ✅ **Numeric Types**: Integer, floating-point, hexadecimal, and binary number support
- ✅ **One-Liners**: Statement separation with `;;` allows compact code patterns

### In Progress
- **Type System Improvements**: Better type handling and conversion
- **Memory Management**: More robust heap memory management for strings and complex data structures
- **Syntax Refinements**: Ongoing improvements to the language syntax
- **Function Library**: Growing the standard function library

### Future
- **Improved Error Messages**: Better diagnostics and debugging
- **More Data Types**: Maps and user-defined types
- **Advanced Control Flow**: Switch statements and more complex conditionals
- **Modules**: Code organization and namespaces
- **Array Improvements**:
  - Complete array element access
  - Matrix operations like transpose and determinant
  - Advanced array functions like map, filter, and reduce

## Contributing

Kozeig is open to community contributions in any form! Whether you're fixing bugs, improving documentation, or adding features, your help is appreciated.

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

Kozeig is released under the MIT License. See the LICENSE file for details.

## Acknowledgments

Kozeig was inspired by languages like Python, Lua, and Rust, with a goal of creating a simple, fun language for learning and experimenting.

---

Happy coding with Kozeig!
