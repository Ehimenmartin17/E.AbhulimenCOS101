use std::fs::Openoptions;
use std::io::Write;

fn main() {

    let mut file = Openoptions::new.append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello class".as_bytes()).expect("write failed");
        .as_bytes()).expect("write failed");
    println!("file append success");


}
    

