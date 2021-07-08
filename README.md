[![Continuous Integration](https://github.com/quantumferret/Arduino-DiceRoller/actions/workflows/ci.yml/badge.svg)](https://github.com/quantumferret/Arduino-DiceRoller/actions/workflows/ci.yml)

# Arduino-DiceRoller
A project for NSWI170 Computer Systems course, written in Rust. It was complicated by a compiler issue where LLVM is not properly linking some operations for larger integers (e.g. bitwise left shift for u64 types), such that I couldn't use typical implementations for a pseudorandom number generator such as Pcg32, XorShift and so on. As such, I had to reinvent the wheel to some extent when it came to generating random dice rolls.


### Flashing an Arduino Uno with the binary distributable
1. In a Bash shell, clone the repository
    `git clone https://github.com/quantumferret/Arduino-DiceRoller.git`
    
2. Click on the `Actions` tab in the repository.

3. Click on the most recent workflow run with a green check next to it

4. Scroll down to the `Artifacts` box, and click on `nswi170-final-project` to download the nswi170-final-project.elf file needed to flash the board.

5. In your shell, `cd Arduino-DiceRoller`

6. Flash the board
    `./uno_runner.sh <path-to-downloaded-elf-file>/nswi170-final-project.elf`
    
  Possible issues:
    You may need to unpack the `nswi170-final-project` download, if your operating system does not do so for you. Standard Bash unzipping tools should do the trick if this is the case.
    Make sure `uno_runner.sh` is executable! If in doubt, run `chmod +x uno_runner.sh` inside the project directory.
