use std::io::{self};

use lambert_w_function::lambert_funtion;
use structopt::{clap::ErrorKind, StructOpt};
pub mod lib;
#[derive(StructOpt)]
#[structopt()]
pub struct Arg{
     x:f64
}

impl Arg{

fn new(x:f64)->Self{

    Self{x}
}
}


fn main() {
 
let mut  args = Arg::from_args();
 println!("Enter the number (x)");

 match lambert_funtion(args.x) {
    Ok(w) => println!("W({}) = {}", args.x, w),
    Err(e) => eprintln!( "Error: {}",e),
 }

}

#[test]

fn it_worlks() {
    assert_eq!(
        lambert_funtion(-0.367879441171299999).unwrap(),
        (-0.9999991202156511)
    );
}
