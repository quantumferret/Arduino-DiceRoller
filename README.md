[![Continuous Integration](https://github.com/quantumferret/Arduino-DiceRoller/actions/workflows/ci.yml/badge.svg)](https://github.com/quantumferret/Arduino-DiceRoller/actions/workflows/ci.yml)

# Arduino-DiceRoller
A project for NSWI170 Computer Systems course, written in Rust. It was complicated by a compiler issue where LLVM is not properly linking some operations for larger integers (e.g. bitwise left shift for u64 types), such that I couldn't use typical implementations for a pseudorandom number generator such as Pcg32, XorShift and so on.
