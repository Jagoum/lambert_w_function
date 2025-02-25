
use lambert_w_function::lambert_funtion;
use log::info;
use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt()]
pub struct Arg{
     x:f64
}

fn main() {
    env_logger::init();
 
let  args = Arg::from_args();
 info!("Evaulating the product log of {}",args.x);

 match lambert_funtion(args.x) {
    Ok(w) => println!("W({}) = {}", args.x, w),
    Err(e) => eprintln!( "{}",e),
 }

}

#[test]
fn it_worlks() {
    assert_eq!(
        lambert_funtion(-0.367879441171299999).unwrap(),
        (-0.9999991202156511)
    );
}
