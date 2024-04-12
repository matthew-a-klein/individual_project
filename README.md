# Pratt Parsing Calculator

This program is a simple calculator that utilizes Pratt parsing to evaluate mathematical expressions involving integers,, durations and dates. Pratt parsing, also known as top-down operator precedence parsing, is an efficient method for parsing expressions.

## Features

- Evaluate mathematical expressions involving integers.
- Evaluates dates and durations
- Support for basic arithmetic operators: addition (+), subtraction (-), multiplication (\*), and division (/).
- Proper precedence and associativity handling.
- Error handling for invalid expressions.

## Usage

Ensure you have Rust installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).

Run the program by executing the `main.rs` file with Rust. Provide the file name containing the programme as a command-line argument. For example, from the project root:

```bash
cargo run src/example_progs/add_day.pratt
```

This will output the date `2023-06-06 00:00:00 UTC`.

Other example programmes can be found in the same folder.

## Supported Operators

### For Integers

- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`

### For Dates

- Addition of durations to dates, creating a new date
- Subtracting one date from another, to create a duration
- Dates are declared with the syntax dd//mm//yyyy

### For Durations

- Multiplication of durations by an integer, to create a new duration
- Addition and subtraction of one duration to another, creating a new duration
- Durations are declared by using the duration keywords
- Keywords for second are 's', 'S', 'sec', 'Sec', 'second' and 'Second'
- Keywords for minute are 'm', 'M', 'min', 'Min', 'minute' and 'Minute'
- Keywords for hour are 'h', 'H', 'hour' and 'Hour'
- Keywords for day are 'd', 'D', 'day' and 'Day'
- Keywords for week are 'w', 'W', 'week' and 'Week'
- Keywords for month are 'month' and 'Month'
- Keywords for year are 'y', 'Y', 'year' and 'Year'

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

### Comments

Comments can be declared by /\* \*/ delimiters.

#### Example

/_ This is a comment. _/
