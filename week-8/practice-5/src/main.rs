fn main(){
    // create an empty vector "city"
    let mut city : Vec<String> = Vec::new();
    // print city vector
    println!("The city vector has element {}",city.len());
    //push new elements into
    let mut input1 = String::new();
    println!("How many cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("invalid");
    let city_num:i32 = input1.trim().parse().expect("invalid");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter city {}", count+1);
        std::io::stdin().read_line(&mut input2).expect("invalid");
        let new_city:String = input2.trim().parse().expect("invalid");
        city.push(new_city);
    }
    print!("Your preferred cities are:\n");
    let mut count=1;
    // loop to iterate elements in vector
    for i in city  
    {
        //iterating through i on the vector
        println!("{} {}", count, i);
        count+=1;
    }    
}

