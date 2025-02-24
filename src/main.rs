use std::io::{self};

use lib::lambert_funtion;
use structopt::clap::ErrorKind;
pub mod lib;


fn main() {
 
 let mut  args = String::new();
 println!("Enter the number (x)");
io::stdin().read_line(&mut args).expect("Failed to read line");
 let x: f64 = match  args.trim().parse() {
    Ok(x) => x,
    Err(_) => {println!("{:?}",ErrorKind::InvalidValue);
    -0.0
    }
};
println!("W({x}) = {}",lambert_funtion(x).unwrap());
}

#[test]

fn it_worlks() {
    assert_eq!(
        lambert_funtion(-0.367879441171299999).unwrap(),
        (-0.9999991202156511)
    );
}
