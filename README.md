# Pratt Parsing Calculator

This program is a simple calculator that utilizes Pratt parsing to evaluate mathematical expressions involving integers,, durations and dates. Pratt parsing, also known as top-down operator precedence parsing, is an efficient method for parsing expressions.

## Features

- Evaluate mathematical expressions involving integers.
- Evaluates dates and durations
- Support for basic arithmetic operators: addition (+), subtraction (-), multiplication (\*), and division (/).
- Proper precedence and associativity handling.
- Error handling for invalid expressions.

## Installation

1. Clone this repository to your local machine:

```
git clone https://github.com/matthew-a-klein/individual_project.git
```

2. Ensure you have Rust installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).

## Usage

Run the program by executing the `main.rs` file with Rust. Provide the file name containing the programme as a command-line argument. For example, from the project root:

```bash
cargo run src/example_progs/add_day.pratt
```

This will output the date `2023-06-06 00:00:00 UTC`.

## Supported Operators

### For Integers

- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`

### For Dates

- Addition of durations to dates, creating a new date
- Subtracting one date from another, to create a duration

### For Durations

- Multiplication of durations by an integer, to create a new duration
- Addition and subtraction of one duration to another, creating a new duration

## Supported programming constructs

### Variable assignment

Variable names consist of at least one letter, followed by an optional number.
Variables are assigned by placing an '=' operator between the variable name and a valid expression.

#### Example

integer = 3

date = 11//12//23

### Function calls

Functions are assigned by a variable name followed by brackets containing 0 or more argument names.
They can then be called with standard function call syntax

#### Example

addDay(date) = date + 1 \* day

addDay(11//12//2023)

This will result in the date 12//12/2023

### Conditional Ternary Expressions

Ternary operations specify a boolean value, followed by a '?' operator, followed by the expression to be evaluated of the boolean evaluates to true, followed by a ':' operator, followed by the expression to be evaluated if the boolean expression evaluates to false.

#### Example

3 == 1 + 2 ? 11//12//2023 : 12//11/2023
