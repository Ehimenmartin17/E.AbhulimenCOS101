use std::io;

fn checker(){

    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("invalid");
    let ch:char = input.trim().parse().expect("invalid");

    if ch >= '0' && ch <= '9'
    {
        println!("Character '{}' is a digit",ch);
    }
    else 
    {
        println!("character '{}' is not a digit",ch);
    }
}

fn main(){
    //calling function
    println!("Welcome! This program checks whether a character variable contains a digit or noy");
    checker()
}    

            
    

