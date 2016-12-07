#[macro_use]
extern crate clap;
extern crate ears;
extern crate rand;
//extern crate tempfile;

use clap::{App, Arg};
use ears::{AudioController, Recorder, Sound};
//use tempfile::NamedTempFileOptions;

use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::Read;

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
        .help("Length of recording in milliseconds.")
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
    /*let name = named_temp_file.path()
    .file_name().unwrap();
    let name_str = name.to_os_string().into_string().unwrap();
    */
    // Create a Sound with the path of the sound file.
    let mut snd = Sound::new(m.value_of("tone_file").unwrap()).unwrap();
    // Play it
    snd.play();

    // Create a new context to record audio
    let context = ears::init_in().unwrap();
    // Create the recorder
    let recorder = Recorder::new(context);
    // Start to record the tone
    ears::init();

    // initialize the RecordContext
    let ctxt = ears::init_in().expect("Initialization error!");

    // Create a new Recorder using the RecordContext
    let mut recorder = ears::Recorder::new(ctxt);
    println!("Recording for {} milliseconds", m.value_of("randomness").unwrap());
    recorder.start();
    let mils = m.value_of("randomness").unwrap().parse::<u64>();
    sleep(Duration::from_millis(mils.unwrap()));
    recorder.stop();
    match recorder.save_to_file("echran.wav"/*name_str.as_str()*/) {
        true => println!("Save okay!"),
        false => println!("Cannot save ...")
    }

    //Read the temp file
    //let chars_to_trim: &[char] = &['.', 'w', 'a', 'v'];
    //let trimmed_str: &str = name_str.as_str().trim_matches(chars_to_trim);
    //println!("Tempfile at: {}", name_str.as_str());
    let mut file = File::open("echran"/*trimmed_str*/).unwrap();

    let mut contents: Vec<u8> = Vec::new();
    // Returns amount of bytes read and append the result to the buffer
    let result = file.read_to_end(&mut contents).unwrap();
    println!("Read {} bytes", result);



}
