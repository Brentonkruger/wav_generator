
extern crate clap;
extern crate sample;
use clap::{Arg, App, SubCommand};
use sample::{signal, Signal};

fn main() {
    let matches = App::new("Sine Generator")
        .author("Brenton Kruger brenton@circalengineering.com")
        .about("Generates wave files for development work")
        .version("0.1")
        .arg(Arg::with_name("sample_rate")
            .short("sr")
            .long("sample_rate")
            .value_name("RATE")
            .default_value("44100")
            .help("Sets the sample rate of the file in Hz")
            .takes_value(true))
        .arg(Arg::with_name("bit_depth")
            .short("bd")
            .long("bit_depth")
            .value_name("DEPTH")
            .default_value("24")
            .help("Sets the bit depth of the file")
            .takes_value(true))
        .arg(Arg::with_name("frequency")
            .short("f")
            .long("frequency")
            .value_name("FREQ")
            .default_value("150")
            .help("sets the frequency of the sine wave in Hz")
            .takes_value(true))
        .arg(Arg::with_name("duration")
            .short("d")
            .long("duration")
            .value_name("SEC")
            .default_value("2")
            .help("sets the duration of the track in seconds")
            .takes_value(true))
        .get_matches();


    let sample_rate = matches.value_of("sample_rate").unwrap().parse::<f64>().unwrap();
    let bit_depth = matches.value_of("bit_depth").unwrap().parse::<f64>().unwrap();
    let frequency = matches.value_of("frequency").unwrap().parse::<f64>().unwrap();
    let duration = matches.value_of("duration").unwrap().parse::<f64>().unwrap();

    let mut signal = signal::rate(sample_rate).const_hz(frequency).sine();


}
