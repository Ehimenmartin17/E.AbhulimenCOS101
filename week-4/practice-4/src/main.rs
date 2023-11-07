fn main() {
   
   let fullname = "Okonedo Tymon Nosa";
   let department = "Software Engineering";
   let uni = "Covenant University";

   let mut school = "School of science".to_string();
   // push string
   school.push_str(" and Technology");

   println!("My name is: {}", fullname);
   // check length
   println!("The length my fullname is: {}",fullname.len());
   println!("I am a student of {} Department", department);
   println!("{}",school);
   println!("{}",uni)

}
