#[macro_use]
extern crate clap;
extern crate ears;
extern crate rand;
extern crate sha1;
extern crate tempfile;

use clap::{App, Arg};
use ears::{AudioController, Recorder, Sound};
use tempfile::NamedTempFileOptions;

use std::io::{self, BufReader};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };
    let mut bytes: Vec<u8>;
    match file.read(&mut bytes) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{}", display),
    }

    let mut m = sha1::Sha1::new();
    for i in 0..bytes.len() {
        let temp: [u8] = [bytes.get(i).unwrap()];
        m.update(temp);
    }

    println!("{}", m.digest().to_string());
}
