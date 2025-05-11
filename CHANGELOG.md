# Lüt Language Changelog

## Version 0.4.0 - New Data Types & Array Support (Current)

### New Features
* **Arrays**: Added comprehensive array support with 1D and 2D arrays
  * Array literal syntax: `myArray : { array [1, 2, 3, 4, 5] }`
  * 2D array syntax: `matrix : { array [1, 2, 3][4, 5, 6][7, 8, 9] }`
  * Array access: `{ get $myArray, 0 }` and `{ get2d $matrix, 1, 2 }`
* **Floating Point Numbers**: Added support for floating point numbers with the `fp` type
* **Hexadecimal Numbers**: Added support for hexadecimal numbers with the `hex` type
* **Binary Numbers**: Added support for binary numbers with the `bin` type
* **Comment Syntax**: Changed comment syntax from `@@` to `--` for better keyboard compatibility

### Implementation Details
* **Array Handling**: Implemented array literals and 2D arrays in the parser and interpreter
* **Number Type System**: Enhanced the type system to handle different number formats
* **Parser Updates**: Updated parser to recognize new data types and array syntax
* **Documentation**: Updated documentation to reflect new data types and syntax changes
* **Example File**: Added `arrays.lut` to demonstrate array usage

## Version 0.3.0 - Loop Improvements & Bug Fixes

### Fixes
* **Loop Variable Updates**: Fixed a critical issue in the compiler where variable updates inside loops weren't properly applied in compiled programs
* **Compiler Robustness**: Enhanced variable assignment handling to distinguish between declarations and updates
* **Memory Correctness**: Addressed an issue where loop counters weren't being properly incremented in compiled binaries

### Implementation Details
* **Binary Expression Handling**: Improved handling of binary expressions with assignment operators
* **Variable Reference Context**: Better detection of variable update contexts versus declarations
* **LLVM IR Generation**: Fixed LLVM IR generation for variable updates in loop bodies

## Version 0.2.0 - Syntax Unification

### Major Changes

#### Unified Syntax with Curly Braces

* **Standardized Command Syntax**: All commands now use a consistent pattern: `command { arguments }`
  * Old: `-print $variable`
  * New: `print { $variable }`

* **Unified Variable Declarations**: Type commands now use curly braces
  * Old: `name : -type value`
  * New: `name : { type value }`

* **Consistent Block Structure**: All code blocks (commands, conditionals, etc.) now use curly braces
  * Improves readability and makes the code structure more visually consistent

#### New Features

* **Logical Operators**: Fully implemented `&&`, `||`, and `!` operators
* **Ternary Expressions**: Added support for conditional expressions: `condition ? true_value : false_value`
* **Debug Mode**: Added `lut debug filename.lut` command to show tokens and AST for development and learning

### Implementation Details

* **Parser Updates**: Modified parser to handle commands with curly braces in both statements and expressions
* **Lexer Updates**: Updated to recognize commands without hyphen prefixes
* **Interpreter Compatibility**: Added support in the interpreter for commands with both old and new syntax formats
* **Compiler Support**: Updated LLVM IR generation to handle the new syntax
* **Comprehensive Documentation**: Updated SYNTAX.md and README.md to reflect the new unified syntax

### Example Files

* Added `showcase.lut` - A comprehensive demonstration of all language features with the new syntax
* Updated existing examples to use the new syntax

### Design Philosophy

The syntax unification was guided by these principles:
1. **Consistency**: All code blocks use curly braces, making the structure immediately recognizable
2. **Clarity**: Commands and type operations use a consistent `name { arguments }` pattern
3. **Readability**: Variable references still use the `$` prefix for clear distinction
4. **Expressiveness**: Common operations use familiar syntax patterns from mainstream languages
