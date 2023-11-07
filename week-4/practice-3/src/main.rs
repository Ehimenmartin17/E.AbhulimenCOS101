fn main() {
   let name1 = "Ehimen Abhulimen";
   println!("My name is {}",name1 );
   
   //find and replace
   let name2 = name1.replace("Ehimen","Abhulimen");
   println!("you can also call me {}",name2);
   let faculty = "Faculty of science and Technology";

   //find and replace
   let school = faculty.replace("Faculty", "school");
   println!("I am a student of the {}", school);

}   