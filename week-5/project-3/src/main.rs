use std::io;

    fn main(){

    println!("P: Pounded yam 3200");
    println!("F: fried rice and chicken 3000", );
    println!("A: amala & ewedu 2500", );
    println!("E: Eba and egusi 2000", );
    println!("W: White rice and stew 2500", );

       let mut input1 = String::new();
       let mut input2 = String::new();
       let mut input3 = String::new();
       let mut input4 = String::new();
       let mut input5 = String::new();

       println!("Enter the quantity (plates) of P, if you don't want any input 0"); 
       io::stdin().read_line(&mut input1).expect("invalid");
       let P:f32 = input1.trim().parse().expect("invalid");

       println!("Enter the quantity (plates) of F, if you don't want any input 0");
       io::stdin().read_line(&mut input2).expect("invalid");
       let F:f32 = input2.trim().parse().expect("invalid");

       println!("Enter the quantity (plates) of A, if you don't want any input 0");
       io::stdin().read_line(&mut input3).expect("invalid");
       let A:f32 = input3.trim().parse().expect("invalid");

       println!("Enter the quantity (plates) of E, if you don't want any input 0");
       io::stdin().read_line(&mut input4).expect("invalid");
       let E:f32= input4.trim().parse().expect("invalid");

       println!("Enter the quantity (plates) of W, if you don't want any input 0");
       io::stdin().read_line(&mut input5).expect("invalid");
       let W:f32= input5.trim().parse().expect("invalid");

       let p1 = P * 3200.0;
       let f1 = F * 3000.0;
       let a1 = A * 2500.0;
       let e1 = E * 2000.0;
       let w1 = W * 2500.0;

       let totalcost = p1 + f1 + a1 + e1 + w1;


       if totalcost > 10000.00 {
        let discounted = 0.95 * totalcost;
          println!("you have received a discount of 5%.\nYour discounted price {}", discounted);
    }  
    else {
                  println!("Your total bill is {}", totalcost);
              }          
}

          
       


        
    

