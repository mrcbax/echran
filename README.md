The Echo Randomizer
===
A proof of concept truly random number generator.
---

The Echo Randomizer utilizes a speaker and microphone to read an "echo" of a room. Like sonar the microphone picks up a tone played by the speaker that has been changed by the environment it has echoed through. Effectively using the room around you as a random number generation seed.

The idea is that the sonic environment around you is constantly changing.

Not only that, but the hardware used to "echo" is unique to any given computer.

The only way to break this would be to soundproof and airflow-proof an unchanging room. If this were done you'd get the same results every time.

Installation:
---
Grab the repo:
> git clone https://github.com/LogoiLab/echran.git

Update your system:
> sudo apt-get install libopenal-dev libsndfile1-dev md5sum

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

>    echran **randomness** **tone_file**

> FLAGS:

>    -h, --help       Prints help information

>    -V, --version    Prints version information

> ARGS:

>    **randomness**    Length of recording in milliseconds.

>    **tone_file**     The path to an ogg tone file to play.

This project is licensed under the [Software Distribution Disclaimer License](https://www.github.com/LogoiLab/SDIS2L)
