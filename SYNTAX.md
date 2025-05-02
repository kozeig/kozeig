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

Lüt has two primary data types (with more planned):

1. **Numbers** - Integer values 
   ```
   age : -number 30
   ```

2. **Text** - String values, enclosed in single quotes
   ```
   name : -text 'John'
   ```

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

Lut supports basic arithmetic operations:

#### Addition

```
sum : -add $a, $b
```

#### Subtraction

```
difference : -sub $a, $b
```

#### Multiplication

```
product : -mul $a, $b
```

#### Division

```
quotient : -div $a, $b
```

#### Modulo

```
remainder : -mod $a, $b
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
sum : -add $a, $b
-print -text 'Sum: ', $sum

@@ Subtraction
difference : -sub $b, $a
-print -text 'Difference: ', $difference

@@ Multiplication
product : -mul $a, $b
-print -text 'Product: ', $product

@@ Division
quotient : -div $b, $a
-print -text 'Quotient: ', $quotient

@@ Modulo
remainder : -mod $b, $a
-print -text 'Remainder: ', $remainder
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

## Implementation Details

Lüt is implemented in Rust and compiles to efficient native code through an intermediate C representation. The runtime includes a stack-based virtual machine that executes bytecode instructions derived from your Lüt source code. While this is not the most efficient or ideal way to do things, it is more fun than strangling myself.

I am working on a more efficient and robust compiler, but it is not yet ready for use. Contributions are welcome!