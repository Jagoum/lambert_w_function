use lambert_funtion::lambert_funtion;
mod lambert_funtion;
fn main() {
    println!("{}",lambert_funtion(-0.367879441171299999).unwrap());
}

#[test]

fn it_worlks(){
    assert_eq!(lambert_funtion(-0.367879441171299999).unwrap(),(-0.9999997330570848));
}
