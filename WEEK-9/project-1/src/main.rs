use std::io::Write;

fn main(){

    let line1 = "\nLager: 33 Export, Desperados, Gulder, Heineken, star, Goldberg";
    let line2 = "\nStout: Legend, Williams, Turbo king";
    let line3 = "\nmaltina, Amstel malta, Malta Gold, Fayrouz";

    let mut file = std::fs::File::create("project_1.txt").expect("create failed");
    file.write_all(line1.as_bytes()).expect("write failed");
    file.write_all(line2.as_bytes()).expect("write failed");
    file.write_all(line3.as_bytes()).expect("write failed");
    println!("\nData written to file");
}
    


