use std::fs::File;
use std::io::Write;

fn main(){

    let student_names = vec!["Oluchi Mordi","Okonedo Tymon","Nwa Akanimo","Enakhena Emmanuel","Olokodana Anu"];
    let matric_numbers = Vec!["ACC102111111","EMA109876","JHU6789456","OIP0973980","IUY87450932"];
    let departments = Vec!["Medicine","Accounting","Engineering","Aviation","Medicine"];

    let file_name = "student_details.txt";

    let mut file = File::create(file_name).expect("Error: unable to create or open the file.");

    file.write_all(b" STUDENT INFORMATION MANAGEMENT SYSTEM\n").expect("Error writing to file");

    file.write_all(format!("{:<20} {:<20} {:<30} {:<10}\n", "Name", "Matric No", "Department", "Level")
    .as_bytes())
    .expect("Error writing to file");

    for n in 0..student_names.len() {
        file.write_all(
            format!("{:<20} {:<20} {:<30} {:<10}\n", student_names[n]), matric_numbers[n], departments[n], levels[n])
            .as_bytes()
        
        .expect("Error writing to file");
    }
    
    println!("Students details written to {}", file_name );         
}
    

