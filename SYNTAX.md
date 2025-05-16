# Kozeig Programming Language Syntax Guide

## Introduction

Kozeig is a simple, interpreted and compiled programming language with a focus on readability and straightforward syntax. The name "lüt" comes from "loot", reflecting the language's playful nature while taking inspiration from languages like C, C++, and Python, but with a consistent, unified syntax.

## Basic Syntax Elements

### Functions

Functions in Kozeig are defined using the `func` keyword with a visibility modifier (`pub` or `prot`), followed by the function name, parameters in curly braces, and the function body in square brackets:

```koze
func pub|prot functionName { param1 : type !, param2 : type ! } [
    -- Function body statements
    -- The last expression is implicitly returned
]
```

### Main Function

The main function serves as the entry point to every Kozeig program. It is a special function that takes no parameters and should return a value:

```koze
func pub main {} [
    -- Program statements

    'ok'  -- Return value, similar to 'return 0' in C
]
```

The main function:
- Must be named `main`
- Must be `pub` (public)
- Takes no parameters (empty braces `{}`)
- Should return a value as the last expression (conventionally `'ok'`)

The exclamation mark (`!`) after a parameter type indicates that the parameter is uninitialized and must be provided when calling the function.

Functions are called using the `call` keyword:

```koze
call { functionName, arg1, arg2 }
```

You can assign the return value of a function to a variable:

```koze
result : call { functionName, arg1, arg2 }
```

Example of a simple function definition and call:

```koze
-- Define a function to add two numbers
func pub add { a : number !, b : number ! } [
    $a + $b  -- Last expression is returned
]

-- Define the main function as entry point
func pub main {} [
    -- Call the function and store the result
    sum : call { add, 5, 3 }
    print { 'Sum: ', $sum }  -- Outputs: Sum: 8

    'ok'
]
```

Functions can be recursive:

```koze
-- Recursive factorial function
func pub factorial { n : number ! } [
    if { $n <= 1 } [
        1  -- Base case
    ] else [
        $n * call { factorial, $n - 1 }  -- Recursive case
    ]
]

-- Main entry point
func pub main {} [
    print { 'Factorial of 5: ', call { factorial, 5 } }  -- Outputs: Factorial of 5: 120

    'ok'
]
```

### Comments

Comments in Kozeig start with `--` and continue until the end of the line:

```koze
-- This is a comment
```

### Variables

Variables are defined using a name followed by a colon and a value declaration with curly braces:

```koze
variableName : { type value }
```

For example:

```koze
greeting : { text 'Hello World' }
number : { number 42 }
```

### Data Types

Kozeig supports the following data types:

1. **Numbers** - Integer values
   ```
   age : { number 30 }
   negative : { number -10 }
   ```

2. **Floating Point** - Decimal numbers
   ```
   pi : { fp 3.14159 }
   temperature : { fp 98.6 }
   ```

3. **Text** - String values, enclosed in single quotes
   ```
   name : { text 'John' }
   message : { text 'Hello, world!' }
   ```

4. **Arrays** - Collection of values
   ```
   myArray : { array [1, 2, 3, 4, 5] }
   matrix : { array [1, 2, 3][4, 5, 6][7, 8, 9] }  -- 2D array
   mixedArray : { array [1, "text", true, 3.14] }  -- Different types allowed
   ```

5. **Boolean** - True/false values

   Booleans can be created in several ways:

   a. Using boolean literals:
   ```
   isValid : true
   isEmpty : false
   ```

   b. Using the bool command (for conversion):
   ```
   isTrue : { bool 1 }
   isFalse : { bool 0 }
   textTrue : { bool 'yes' }
   textEmpty : { bool '' }  @@ false because empty string
   ```

   c. Using comparison operations:
   ```
   isGreater : $a > $b
   isEqual : $a == $b
   ```

   Boolean literals don't need a special command. You can just use `true` or `false` directly.

   Truthy values in Kozeig (values considered true in boolean context):
   - Any non-zero number
   - Any non-empty string
   - The boolean literal `true`

   Falsy values (values considered false in boolean context):
   - Zero (0)
   - Empty string ('')
   - The boolean literal `false`

### Variable References

To use a variable's value, prefix its name with a dollar sign `$`:

```
print { $variableName }
```

## Commands

Commands in Kozeig use a name followed by arguments enclosed in curly braces:

### Print Command

The print command outputs values to the console:

```
print { $variable }
```

You can print multiple values by separating them with commas:

```
print { $firstName, $lastName, ' is ', $age, ' years old' }
```

### Type Conversion Commands

#### Text Conversion

Convert a value to text:

```
textValue : { text $someValue }
```

#### Number Conversion

Convert a value to a number:

```
numericValue : { number $someValue }
```

#### ASCII Conversion

Convert a number to its ASCII character representation:

```
letter : { asc 65 }  -- Converts to 'A'
```

#### Hexadecimal Conversion

Convert a hexadecimal string to a number:

```
value : { hex '0xFF' }  -- Converts to 255
```

#### Binary Conversion

Convert a binary string to a number:

```
value : { bin '0b1010' }  -- Converts to 10
```

### Arithmetic Operations

Kozeig supports standard arithmetic operators:

#### Addition

```
sum : $a + $b
```

#### Subtraction

```
difference : $a - $b
```

#### Multiplication

```
product : $a * $b
```

#### Division

```
quotient : $a / $b
```

#### Modulo

```
remainder : $a % $b
```

### Array Operations

Kozeig provides several commands for working with arrays:

#### Array Length

Get the number of elements in a 1D array or the number of rows in a 2D array:

```
len : { length $myArray }
```

#### 2D Array Width

Get the number of columns in a 2D array:

```
width : { width $matrix }
```

#### Array Element Access

Access elements in a 1D array (0-based indexing):

```
firstElement : { get $myArray, 0 }
```

Access elements in a 2D array (0-based indexing for rows and columns):

```
element : { get2d $matrix, 1, 2 }  -- Gets the element at row 1, column 2
```

#### Compound Operations

Kozeig supports compound operations with proper operator precedence:

```
result : $a + $b * $c
```

#### Grouping with Parentheses

You can use parentheses to control the order of operations:

```
result : ($a + $b) * $c
```

## Statement Separators

Statements in Kozeig are typically separated by newlines. You can also use double semicolons (`;;`) to separate statements on the same line, which allows for compact one-liners:

```koze
-- Multiple statements on a single line
name : { text 'John' } ;; age : { number 30 } ;; print { $name, ' is ', $age, ' years old' }
```

The statement separator `;;` works at both the top level of your program and inside control block structures like if statements and loops:

```koze
-- Statement separators inside control blocks
for { i : 0, $i + 1, $i < 3 } [
    value : $i * 2 ;; print { 'i =', $i, 'value =', $value }
]

-- Statement separators in if/else blocks
if { $score >= 90 } [
    grade : 'A' ;; message : 'Excellent!'
] else [
    grade : 'B' ;; message : 'Good job!'
]
```

This feature allows for more compact code while maintaining readability.

## Example Programs

### Hello World

```koze
-- Simple Hello World program
func pub main {} [
    greeting : { text 'Hello, World!' }
    print { $greeting }

    'ok'
]
```

### Basic Arithmetic

```koze
-- Simple arithmetic example
func pub main {} [
    a : { number 5 }
    b : { number 10 }

    -- Addition
    sum : $a + $b
    print { 'Sum: ', $sum }

    -- Subtraction
    difference : $b - $a
    print { 'Difference: ', $difference }

    -- Multiplication
    product : $a * $b
    print { 'Product: ', $product }

    -- Division (includes runtime division-by-zero protection)
    quotient : $b / $a
    print { 'Quotient: ', $quotient }

    -- Modulo (includes runtime modulo-by-zero protection)
    remainder : $b % $a
    print { 'Remainder: ', $remainder }

    -- Compound operations with operator precedence
    compound1 : $a + $b * 2
    print { 'Compound (a + b * 2): ', $compound1 }  -- Multiplication happens first

    -- Using parentheses to override precedence
    compound2 : ($a + $b) * 2
    print { 'Compound ((a + b) * 2): ', $compound2 }

    -- Conditional logic with current syntax
    if { $a < $b } [
        print { '$a is less than $b' }

        if { $a * 2 > $b } [
            print { 'But $a * 2 is greater than $b' }
        ]
    ]

    'ok'
]
```

### ASCII Conversion

```koze
-- ASCII conversion example
func pub main {} [
    h : { asc 72 }
    e : { asc 101 }
    l : { asc 108 }
    l2 : { asc 108 }
    o : { asc 111 }

    print { $h, $e, $l, $l2, $o }  -- Prints "hello"

    'ok'
]
```

## Identifiers and Naming Conventions

Variable names in Kozeig:
- Can contain alphanumeric characters and underscores
- Must start with a letter
- Are case-sensitive
- Cannot be keywords

## Running Kozeig Programs

### Interpretation

Run a Kozeig program directly:

```
koze run yourprogram.ko
```

### Compilation

Compile a Kozeig program to a standalone executable:

```
koze build yourprogram.ko
```

This creates an executable file named after your program that can be run directly:

```
./yourprogram
```

## Control Flow

### If-Else Statements

Kozeig supports conditional execution with if-else statements:

```koze
if { <expression> } [
    @@ then branch statements
] else [
    @@ else branch statements (optional)
]
```

Example:

```koze
age : { number 25 }

if { $age >= 18 } [
    print { 'You are an adult' }
] else [
    print { 'You are under 18' }
]
```

If statements evaluate an expression, and if the expression is "truthy" (non-zero, non-empty, or true), the then branch is executed. Otherwise, the else branch is executed if it exists.

Nested if statements are also supported:

```koze
score : { number 85 }

if { $score >= 60 } [
    print { 'You passed!' }

    if { $score >= 90 } [
        print { 'Excellent job!' }
    ] else [
        print { 'Good job!' }
    ]
] else [
    print { 'You failed.' }
]
```

### Comparison Operators

Kozeig supports the following comparison operators:

- `==` - Equal to
- `!=` - Not equal to
- `<` - Less than
- `<=` - Less than or equal to
- `>` - Greater than
- `>=` - Greater than or equal to

Example:

```koze
a : { number 10 }
b : { number 20 }

if { $a < $b } [
    print { '$a is less than $b' }
]

if { $a == $b } [
    print { '$a is equal to $b' }
] else [
    print { '$a is not equal to $b' }
]
```

### Logical Operators

Kozeig supports the following logical operators:

- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT

Example:

```koze
a : { number 5 }
b : { number 10 }

@@ Check if a is between 0 and 10
in_range : $a > 0 && $a < 10

@@ Check if b is negative or greater than 100
out_of_range : $b < 0 || $b > 100

@@ Negate a boolean
valid : true
invalid : !$valid

if { $in_range && !$out_of_range } [
    print { 'All conditions met' }
]
```

### Loops

Kozeig provides two types of loops for iteration: while loops and for loops. Both use square brackets `[]` to denote the loop body.

#### While Loops

While loops execute a block of code as long as a condition is true:

```koze
while { condition } [
    @@ loop body statements
    @@ Variable updates within loops work correctly in both interpreter and compiler
]
```

Example:

```koze
counter : 0
while { $counter < 5 } [
    print { 'Counter: ', $counter }
    counter : $counter + 1
]
```

#### For Loops

For loops provide a more structured approach with initialization, update, and condition expressions:

```koze
for { initialization, update, condition } [
    @@ loop body statements
    @@ Variable updates within loops work correctly in both interpreter and compiler
]
```

Example:

```koze
@@ Count from 0 to 4
for { i : 0, $i + 1, $i < 5 } [
    print { 'Index: ', $i }
]

@@ Count by 2s (even numbers)
for { j : 0, $j + 2, $j < 10 } [
    print { 'Even number: ', $j }
]
```

The three expressions in a for loop are:
1. **Initialization**: Executed once before the loop starts (typically a variable declaration)
2. **Update**: Applied after each iteration (typically incrementing a counter)
3. **Condition**: Checked before each iteration - the loop continues as long as this is true

#### Loop Control Statements

Kozeig provides two special statements to control loop execution:

##### Break Statement

The `break` statement immediately exits a loop:

```koze
count : 0
while { true } [  @@ Infinite loop
    print { 'Count: ', $count }
    count : $count + 1

    if { $count >= 5 } [
        break  @@ Exit the loop when count reaches 5
    ]
]
```

##### Continue Statement

The `continue` statement skips the rest of the current iteration and jumps to the next iteration:

```koze
for { i : 0, $i + 1, $i < 10 } [
    if { $i % 2 == 0 } [
        continue  @@ Skip even numbers
    ]
    print { 'Odd number: ', $i }
]
```

#### Nested Loops

Loops can be nested inside other loops:

```koze
for { i : 0, $i + 1, $i < 3 } [
    for { j : 0, $j + 1, $j < 3 } [
        print { 'Position (', $i, ',', $j, ')' }
    ]
]
```

## Implementation Details

Kozeig is now implemented as a true compiler that uses LLVM through the Inkwell Rust bindings. This gives several advantages:

1. **Native Compilation**: Kozeig programs are compiled directly to efficient native machine code without any intermediate language.
2. **Just-In-Time (JIT) Execution**: The `koze jit` command compiles and immediately executes your program without creating an executable file.
3. **Ahead-of-Time (AOT) Compilation**: The `koze build` command creates optimized standalone executables.
4. **LLVM Optimizations**: Benefit from LLVM's powerful optimization passes for better performance.

### Safety Features

The compiler implements several safety features:

1. **Division by zero protection**: Runtime checks prevent division and modulo operations with a zero denominator.
2. **Type tracking**: Variables are tracked by type (Integer, String, Boolean) to ensure proper LLVM IR generation.
3. **Detailed error messages**: Compilation errors provide specific information about where and why things went wrong.
4. **Variable update tracking**: Variables updates are properly handled in loops, conditionals, and other contexts.

### Compiler Pipeline

The compiler works by:
1. Lexing the source code into tokens
2. Parsing the tokens into an abstract syntax tree (AST)
3. Generating LLVM IR (Intermediate Representation) from the AST
4. Optimizing the LLVM IR
5. Generating native machine code for the target platform

### Syntax Design Philosophy

Kozeig's syntax is designed around these principles:

1. **Consistency**: All code blocks use curly braces, making the structure immediately recognizable.
2. **Clarity**: Commands and type operations use a consistent `name { arguments }` pattern.
3. **Readability**: Variable references always use the `$` prefix for clear distinction.
4. **Expressiveness**: Common operations like arithmetic and comparison use familiar operators.

Contributions are welcome to improve the compiler, add new language features, or enhance the standard library!
