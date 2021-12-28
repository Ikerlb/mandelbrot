// use std::env;
mod render;
mod encode;

use num::complex::Complex;
use clap::{Arg, App, SubCommand, AppSettings};
use std::str::FromStr;

fn from_pair<T: FromStr>(s: &str) -> Option<(T, T)>{
    let i = s.find(",")?;
    match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
        (Ok(fst), Ok(snd)) => Some((fst, snd)),
        (_, _) => None
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>>{
    match from_pair(s) {
        Some((fst, snd)) => Some(Complex::new(fst, snd)),
        None => None, 
    }
}

fn main() {
    let matches = App::new("Simple mandlebrot program")
        .version("1.0")
        .author("Iker Lissarrague")
        .about("Does awesome things")
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .help("Sets width of target image")
            .takes_value(true))
         .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .help("Sets height of target image")
            .takes_value(true))
        .arg(Arg::with_name("invert")
            .short("i")
            .long("invert")
            .help("Inverts color"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")   
            .required(true)
            .takes_value(true) 
            .help("Describes output path"))
         .arg(Arg::with_name("upper_left")
            .short("u")
            .long("upper")   
            .allow_hyphen_values(true)
            .number_of_values(1)
            .required(true)
            .takes_value(true) 
            .help("Lower right complex point from which to start indicated as a comma separated pair of floating point numbers."))
          .arg(Arg::with_name("lower_right")
            .short("l")
            .long("lower")   
            .allow_hyphen_values(true)
            .required(true)
            .takes_value(true) 
            .help("Lower right complex point from which to start indicated as a comma separated pair of floating point numbers."))
       .get_matches();

    let w = matches.value_of("width").unwrap_or("500");
    let h = matches.value_of("height").unwrap_or("300");
    let img_bounds = (usize::from_str(w).unwrap(), usize::from_str(h).unwrap());

    let sc1 = matches.value_of("lower_right").unwrap();
    let sc2 = matches.value_of("upper_left").unwrap();

    let c1 = parse_complex(sc1).expect("Upper left complex point pair has an invalid format."); 
    let c2 = parse_complex(sc2).expect("Lower right complex point pair has an invalid format."); 

    let bounds = (c1, c2);

    let mut pixels: Vec<u8> = vec![0; img_bounds.0 * img_bounds.1];

    let filename = matches.value_of("output").unwrap();
    let invert = matches.is_present("invert");

    render::render(&mut pixels, img_bounds, bounds);
    if invert {  
        render::invert(&mut pixels, img_bounds);
    }
    encode::write_image(&pixels, &filename, img_bounds);
}
