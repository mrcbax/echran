#[macro_use]
extern crate clap;
extern crate ears;
extern crate rand;
extern crate tempfile;
extern crate ssdeep;

use clap::{App, Arg};
use ears::{AudioController, Recorder, Sound};
//use rand::{Rng, SeedableRng, StdRng};
use tempfile::NamedTempFileOptions;

//use std::fs::File;
//use std::rc::Rc;
//use std::cell::RefCell;

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
        .suffix(".temp")
        .create()
        .unwrap();
    //get tempfile path
    let name = named_temp_file.path()
        .file_name().unwrap();
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
    recorder.save_to_file(name.to_os_string().into_string().unwrap().as_str());
    let h = ssdeep::hash_from_file(name).unwrap();

    println!("ssdeep: {}", h);
}
