[![Continuous Integration](https://github.com/quantumferret/Arduino-DiceRoller/actions/workflows/ci.yml/badge.svg)](https://github.com/quantumferret/Arduino-DiceRoller/actions/workflows/ci.yml)

# Arduino-DiceRoller
A project for NSWI170 Computer Systems course, written in Rust. It was complicated by a compiler issue (with a pull request currently blocked and pending review in the LLVM repository) where LLVM is not properly linking some operations for larger integers (e.g. bitwise left shift for u64 types), such that I couldn't use typical implementations for a pseudorandom number generator such as Pcg32, XorShift and so on. As such, I had to reinvent the wheel to some extent when it came to generating random dice rolls, and currently the generator used (while sufficient) is sure to have statistical limitations.


### Flashing an Arduino Uno with the binary distributable (Only for Linux and MacOS)
1. In a Bash (or similar) shell, clone the repository
    `git clone https://github.com/quantumferret/Arduino-DiceRoller.git`

2. Run `cd Arduino-DiceRoller` in your shell

3. Flash the board
    `./uno_runner.sh artifacts/nswi170-final-project.elf`
    
  Possible issues:
    Make sure `uno_runner.sh` is executable! If in doubt, run `chmod +x uno_runner.sh` inside the project directory.
