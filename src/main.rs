#[macro_use]
extern crate clap;
extern crate ears;
extern crate rand;
extern crate tempfile;
extern crate ssdeep;

use clap::{App, Arg};
use ears::{Sound, SoundData, AudioController};
use rand::{Rng, SeedableRng, StdRng};
use tempfile::NamedTempFileOptions;

use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    let m = App::new("Echo Randomizer")
        .author(crate_authors!())
        .version(crate_version!())
        .about("A Proof of Concept truly random number generator.")
        .arg(Arg::with_name("randomness")
        .index(1)
        .required(true)
        .short("r")
        .takes_value(true)
        .help("The number of times to play the a tone (default: 1)")
    )
    .arg(Arg::with_name("tone_file")
        .index(2)
        .required(true)
        .short("t")
        .takes_value(true)
        .help("The path to an ogg tone file to play.")
    )
    .get_matches();

    let named_temp_file = NamedTempFileOptions::new()
        .prefix("tempile")
        .suffix(".temp")
        .create()
        .unwrap();
    let name = named_temp_file.path()
        .file_name().unwrap();
    // Create a Sound with the path of the sound file.
    let mut snd = Sound::new(m.value_of("tone_file").unwrap()).unwrap();

    // Play it
    snd.play();

    // Wait until the sound stopped playing
    while snd.is_playing() {}
    let h = ssdeep::hash_from_file(name).unwrap();
    println!("ssdeep: {}", h);
}
