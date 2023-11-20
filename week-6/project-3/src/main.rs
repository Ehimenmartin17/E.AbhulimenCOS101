use std::io;

fn main(){
   let mut n = String::new();
   let input = io::stdin();

   println!("Amount of numbers to be displayed");
   input.read_line(&mut n).expect("invalid string");
   let n:i32 = n.trim().parse().expect("invalid integer");

   println!("");

   for a in 1..n+1{
       for b in 1..n+1{
           print!("{}\t",a*b);
        }
        println!("");
    }
}    

