use std::write;

fn main() {

    let announce = "Week 9 - Rust File Input & output\n";
    let dept = "Department of software engineering";

    let mut file = std::fs::file::create("data.txt").expect("create failed");
    file.write_all("Welcome to rust programming\n"
        .as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file." );
}

    

