use std::io;

fn main(){
  
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("Enter the value for a");
   io::stdin().read_line(&mut input1).expect("invalid");
   let a:f32 = input1.trim().parse().expect("invalid");

   println!("Enter the value of b");
   io::stdin().read_line(&mut input2).expect("invalid");
   let b:f32 = input2.trim().parse().expect("invalid");

   println!("Enter the value of c");
   io::stdin().read_line(&mut input3).expect("invalid");
   let c:f32 = input3.trim().parse().expect("invalid");

   let discriminant:f32 = (b.powf(2.0)) - (4.0 * a * c);

   if discriminant > 0.0{
      println!("there are two distinct roots");
    }
    else if discriminant == 0.0{
      println!("there is only one distinct root"); 
    }
    else if discriminant < 0.0{
      println!("there is no real root");
    }
    else{
      println!("non existent");     
    }
}
    

