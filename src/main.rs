#[macro_use]
extern crate clap;
extern crate ears;
extern crate rand;

use clap::{App, Arg};
use rand::{Rng, SeedableRng, StdRng};
use ears::{AudioController, Record, Sound};

fn main() {
    let mut ran = rand::thread_rng();
    let m = App::new("Echo Randomizer")
    .author(crate_authors!())
    .version(crate_version!())
    .about("A Proof of Concept truly random number generator.")
    .arg(Arg::with_name("randomness")
        .index(1)
        .short("r")
        .help("The length of time in seconds to use as seed (default: 2)")
    )
    .get_matches();
    println!("i32: {}", ran.gen::<i32>());
}
