# LC4 - Logic Chain 4

LC4 is a miniature parser/virtual processor capable of executing simple instructions across an ordered set of 1-bit registers.

**Warning**: This project represents my very first attempt with Rust and was conceived from a simple and enjoyable idea in my notebook. LC4 serves no practical purpose and is purely a toy project created to learn the fundamentals of the Rust programming language.

### Getting Started

To build, test, and run LC4, follow these steps:

```
# Build LC4 by running the following command
cargo build

# Execute tests using the following command
cargo test

# Start LC4 by navigating to your build directory or ensuring the binary is in your path
lc4 [registers]

# For example, the following command will start LC4 with 6 registers (the default is 4, hence the name LC4)
lc4 6
```

### Understanding the Output from LC4

Upon launching LC4, you will see the following output:

```
INT: [0,0,0,0]
INS:
```

Here, `INT: [0,0,0,0]` represents the current register values, and `INS: ` indicates that LC4 is ready to accept a sequence of instructions.

Below is a simple example in which we specify an instruction sequence that accomplishes the following:

-   Sets the first register to the literal `1`
-   Performs no operation on the second register
-   Flips the third and fourth registers

```
INT: [0,0,0,0] # Initial register values.
INS: [1,-,+,+] # Entered instruction sequence.
REG: [1,0,1,1] # Updated register state.
INS:
```

For more information on operators and literals, please refer to the next section.

## Instruction Set

An instruction sequence follows this format (assuming LC4 is running with the default 4 registers):

```
[-,-,-,-]
```

Here, each `-` can be replaced with a literal or operator:

-   `1`: Sets the current register to 1.
-   `0`: Sets the current register to 0.
-   `-`: No operation (no-op).
-   `+`: Flips the current register.
-   `&`: Checks the register and sets the equal flag to false if the register isn't 1.
-   `=`: Stores the value of the equal flag into the current register.
-   `!`: Halts program execution if the current register is 1.

## Chains

LC4 supports chaining instruction sequences together using the `>` symbol. Here's an example:

```
[-,-,-,-] > [-,-,-,-]
```

This enables the construction of programs rather than running individual sequences.

## Examples

The following program takes 3 input values and produces an output value if only the first and third registers are `1`:

```
INT: [0,0,0,0]
INS: [1,0,1,0] # Setting input values before running the program.
REG: [1,0,1,0]
INS: [-,+,-,-] > [&,&,&,-] > [-,+,-,=] # Our program.
REG: [1,0,1,1] # As expected, the fourth register is 1 because the first and third registers were both 1, and the second was 0.
INS:
```

Let's run the program again with different initial values:

```
INT: [0,0,0,0]
INS: [0,1,1,0] # Setting starting values once more.
REG: [0,1,1,0]
INS: [-,+,-,-] > [&,&,&,-] > [-,+,-,=] # Same program as above.
REG: [0,1,1,0] # The result shows that the fourth register is 0 because the first register was 0 and the second was 1.
INS:
```

## License

GNU General Public License v3.0
