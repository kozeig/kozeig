# Lüt Programming Language Syntax Guide

## Introduction

Lüt is a simple, interpreted and compiled programming language with a focus on readability and straightforward syntax. The name "lüt" comes from "loot", reflecting the language's playful nature while taking inspiration from languages like C, C++, and Python, but with a more accessible and fun syntax (in my opinion at least).

## Basic Syntax Elements

### Comments

Comments in Lüt start with `@@` and continue until the end of the line:

```lut
@@ This is a comment
```

### Variables

Variables are defined using a name followed by a colon and a value declaration:

```lut
variableName : -type value
```

For example:

```lut
greeting : -text 'Hello World'
number : -number 42
```

### Data Types

Lüt supports the following data types:

1. **Numbers** - Integer values 
   ```
   age : -number 30
   negative : -number -10
   ```

2. **Text** - String values, enclosed in single quotes
   ```
   name : -text 'John'
   message : -text 'Hello, world!'
   ```

3. **Boolean** - True/false values
   ```
   isValid : true
   isEmpty : false
   ```
   
   Boolean literals don't need a special command. You can just use `true` or `false` directly.

### Variable References

To use a variable's value, prefix its name with a dollar sign `$`:

```
-print $variableName
```

## Commands

Commands in Lut start with a hyphen (`-`):

### Print Command

The print command outputs values to the console:

```
-print $variable
```

You can print multiple values by separating them with commas:

```
-print $firstName, $lastName, -text ' is ', $age, -text ' years old'
```

### Type Conversion Commands

#### Text Conversion

Convert a value to text:

```
textValue : -text $someValue
```

#### Number Conversion

Convert a value to a number:

```
numericValue : -number $someValue
```

#### ASCII Conversion

Convert a number to its ASCII character representation:

```
letter : -asc 65  @@ Converts to 'A'
```

### Arithmetic Operations

Lüt supports standard arithmetic operators:

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

#### Compound Operations

Lüt supports compound operations with proper operator precedence:

```
result : $a + $b * $c
```

#### Grouping with Parentheses

You can use parentheses to control the order of operations:

```
result : ($a + $b) * $c
```

## Line Separators

Statements in Lüt are typically separated by newlines. You can also use double semicolons (`;;`) to separate statements on the same line:

```
name : -text 'John' ;; age : -number 30 ;; -print $name, -text ' is ', $age, -text ' years old'
```

## Example Programs

### Hello World

```
@@ Simple Hello World program
greeting : -text 'Hello, World!'
-print $greeting
```

### Basic Arithmetic

```
@@ Simple arithmetic example
a : -number 5
b : -number 10

@@ Addition
sum : $a + $b
-print 'Sum: ', $sum

@@ Subtraction
difference : $b - $a
-print 'Difference: ', $difference

@@ Multiplication
product : $a * $b
-print 'Product: ', $product

@@ Division
quotient : $b / $a
-print 'Quotient: ', $quotient

@@ Modulo
remainder : $b % $a
-print 'Remainder: ', $remainder

@@ Compound operation
compound : ($a + $b) * 2
-print 'Compound: ($a + $b) * 2 = ', $compound
```

### ASCII Conversion

```
@@ ASCII conversion example
h : -asc 72
e : -asc 101
l : -asc 108
l2 : -asc 108
o : -asc 111

-print $h, $e, $l, $l2, $o  @@ Prints "hello"
```

## Identifiers and Naming Conventions

Variable names in Lüt:
- Can contain alphanumeric characters and underscores
- Must start with a letter
- Are case-sensitive
- Cannot be keywords

## Running Lüt Programs

### Interpretation

Run a Lüt program directly:

```
lut run yourprogram.lut
```

### Compilation

Compile a Lüt program to a standalone executable:

```
lut build yourprogram.lut
```

This creates an executable file named after your program that can be run directly:

```
./yourprogram
```

## Control Flow

### If-Else Statements

Lüt supports conditional execution with if-else statements:

```lut
if <expression> {
    @@ then branch statements
} else {
    @@ else branch statements (optional)
}
```

If statements evaluate an expression, and if the expression is "truthy" (non-zero, non-empty, or true), the then branch is executed. Otherwise, the else branch is executed if it exists.

Example:

```lut
age : -number 25

if $age >= 18 {
    -print 'You are an adult'
} else {
    -print 'You are under 18'
}
```

Nested if statements are also supported:

```lut
score : -number 85

if $score >= 60 {
    -print 'You passed!'
    
    if $score >= 90 {
        -print 'Excellent job!'
    } else {
        -print 'Good job!'
    }
} else {
    -print 'You failed.'
}
```

### Comparison Operators

Lüt supports the following comparison operators:

- `==` - Equal to
- `!=` - Not equal to
- `<` - Less than
- `<=` - Less than or equal to
- `>` - Greater than
- `>=` - Greater than or equal to

Example:

```lut
a : -number 10
b : -number 20

if $a < $b {
    -print '$a is less than $b'
}

if $a == $b {
    -print '$a is equal to $b'
} else {
    -print '$a is not equal to $b'
}
```

## Implementation Details

Lüt is now implemented as a true compiler that uses LLVM through the Inkwell Rust bindings. This gives several advantages:

1. **Native Compilation**: Lüt programs are compiled directly to efficient native machine code without any intermediate language.
2. **Just-In-Time (JIT) Execution**: The `lut jit` command compiles and immediately executes your program without creating an executable file.
3. **Ahead-of-Time (AOT) Compilation**: The `lut build` command creates optimized standalone executables.
4. **LLVM Optimizations**: Benefit from LLVM's powerful optimization passes for better performance.

The compiler works by:
1. Lexing the source code into tokens
2. Parsing the tokens into an abstract syntax tree (AST)
3. Generating LLVM IR (Intermediate Representation) from the AST
4. Optimizing the LLVM IR
5. Generating native machine code for the target platform

Contributions are welcome to improve the compiler, add new language features, or enhance the standard library!