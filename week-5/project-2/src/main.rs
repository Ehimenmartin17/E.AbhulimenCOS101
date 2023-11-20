use std::io;
    fn main(){

       let mut input = String::new();
       
       println!("Enter the age");
       io::stdin().read_line(&mut input).expect("invalid");
       let age:i32 = input.trim().parse().expect("invalid");

       if age >= 40{
          println!("your incentive is $1560000");
        }
          if age >=30 & age < 40{
            println!("your incentive is $1480000");
        }
          if age < 28{
             println!("your incentive is $100,000");
        }         

}
            


