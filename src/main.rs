#[macro_use]
extern crate clap;
extern crate ears;
extern crate rand;
extern crate tempfile;
extern crate hex;

use clap::{App, Arg};
use ears::{AudioController, Recorder, Sound};
use tempfile::NamedTempFileOptions;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

//create tempfile
    let named_temp_file = NamedTempFileOptions::new()
        .prefix("echran")
        .suffix(".wav")
        .create()
        .unwrap();
    //get tempfile path
    let name = named_temp_file.path()
        .file_name().unwrap();
    let nameStr = name.to_os_string().into_string().unwrap();

    // Create a Sound with the path of the sound file.
    let mut snd = Sound::new(m.value_of("tone_file").unwrap()).unwrap();
    // Play it
    snd.play();

    // Create a new context to record audio
    let context = ears::init_in().unwrap();
    // Create the recorder
    let mut recorder = Recorder::new(context);
    while snd.is_playing() {
        // Start to record the tone
        recorder.start();
    }
    // Stop the recorder
    recorder.stop();
    // Then store the recorded data in a file
    let chars_to_trim: &[char] = &['.', 'w', 'a', 'v'];
    let trimmed_str: &str = nameStr.as_str().trim_matches(chars_to_trim);
    recorder.save_to_file(trimmed_str);

    let mut file = BufReader::new(File::open(nameStr.as_str()).unwrap());
    for &byte in file. {
        write!(&mut s, "{:X} ", byte).unwrap();
    }
    println!("{}", s);
}
