# Lüt Language Changelog

## Version 0.2.0 - Syntax Unification (Current)

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
