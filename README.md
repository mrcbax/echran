The Echo Randomizer
===
A proof of concept truly random number generator.

Installation:
---
Grab the repo:
> git clone https://github.com/LogoiLab/echran.git

Build:
> cd echran && cargo update && cargo build --release

Your executable is located at:
> ./target/release/echran

Usage:
---

Input:
> ./echran --help

Output:

> Echo Randomizer 0.1.0
Chad Baxter <cbaxter@mail.umw.edu>
A Proof of Concept truly random number generator.

> USAGE:
>    echran <randomness> <tone_file>

> FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

> ARGS:
    <randomness>    Length of recording in milliseconds.
    <tone_file>     The path to an ogg tone file to play.

This project is licensed under the [Software Distribution Disclaimer License](https://www.github.com/LogoiLab/SDIS2L)
