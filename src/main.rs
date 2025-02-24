use std::{arch::x86_64, env::{self, args}, io::Error};

use lib::lambert_funtion;
use structopt::clap::{value_t, ErrorKind};
pub mod lib;


fn main() {
 
 let args : Vec<String> = env::args().collect();
 let x: f64 = match  args[1].trim().parse() {
    Ok(x) => x,
    Err(_) => {println!("{:?}",ErrorKind::InvalidValue);
    -0.0
    }
};
println!("{}",lambert_funtion(x).unwrap());
}

#[test]

fn it_worlks() {
    assert_eq!(
        lambert_funtion(-0.367879441171299999).unwrap(),
        (-0.9999991202156511)
    );
}
