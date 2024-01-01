fn main() {
   
   // Name vector
   let name = vec!["Mary","Sam","john","Anu","mark"];

   //Age vector
   let age = vec![16,17,18,19,23];

   print!("\nAge allocation:\n");

   //loop to iterate elements in vector
   for i in 0..age.len()
   {
       //iterating through i on the vector
       print!("{} is {} years old\n",name[i],age[i]);
    }
}     

