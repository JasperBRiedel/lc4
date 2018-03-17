# LC4 AKA Logic Chain 4 

LC4 is miniature parser / virtual processor, capable of simple 1 bit calculations across many registers.

**Warning**, this project is my very first with Rust and was designed based on a random idea in one of my notebooks.
and so, lc4 serves no real purpose other than for learning and fun.

### Getting started
Building, testing and starting LC4:
```
# Simply run the following to build LC4
cargo build

# To execute tests run the following
cargo test

# To start LC4 simply navigate to your build directory or ensure the binary is in your path
lc4 [registers]

# For an example, the following will start LC4 with 6 registers. (the default is 4, hence the name, lc4)
lc4 6
```

### Understanding the output from LC4

Upon starting LC4, you'll be given the following output:
```
INT: [0,0,0,0]
INS:
```
Where `INT: [0,0,0,0]` is the current register values.

And `INS: ` is waiting for the user to input a sequence.


Below is a simple example where we've specified an instruction sequence that does the following:
- set the first register with literal `1`
- no operation on the second register
- flip the third and fourth registers
```
INT: [0,0,0,0] # The initial register values.
INS: [1,-,+,+] # The instruction senescence that was entered.
REG: [1,0,1,1] # The new state of the registers.
INS:
```
Please see the next section for more information on operators and literals.

## Instruction set

An instruction sequence is formatted as follows (in the case of running LC4 with the default 4 registers)
```
[-,-,-,-]
```

Where each `-` can be replaced with either a literal or operator


`1` set the current register to 1

`0` set the current register to 0

`-` no-op 

`+` flip the current register

`&` check register and set equal flag to false if register isn't 1

`=` store value of equal flag into current register

`!` halt program execution if the current register is 1

## Chains

LC4 supports chaining instruction sequences together using the `>` symbol. here's an example:
```
[-,-,-,-] > [-,-,-,-]
```
The above allows the construction of programs, rather than just running individual sequences.

## Examples

The following program takes 3 input values and gives us an output value if only the first and third registers are `1`
```
INT: [0,0,0,0]
INS: [1,0,1,0] # This is where we set our input before running our program
REG: [1,0,1,0]
INS: [-,+,-,-] > [&,&,&,-] > [-,+,-,=] # Our program
REG: [1,0,1,1] # The result, as expect the fourth register is 1 because both the first and third registers were 1 and the second was 0 
INS:
```
Let's run the program again with different starting values
```
INT: [0,0,0,0]
INS: [0,1,1,0] # Once again we set our starting values
REG: [0,1,1,0]
INS: [-,+,-,-] > [&,&,&,-] > [-,+,-,=] # Our program, the same as above
REG: [0,1,1,0] # The result, where the fourth register is 0 because the first register was 0 and the second was 1
INS:
```

## Contributions

Although I'm not looking for any code contributions, I'd love any feedback on improving this project, as I'm extremely new to rust!

## License

GNU General Public License v3.0
