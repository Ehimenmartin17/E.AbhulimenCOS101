//defining dimension of the laptop brands
struct Brands{
    hp:i32 , IBM:i32 , Toshiba:i32 , Dell:i32
}

// logic to calculate sum cost
impl Brands{
    fn sum(&self)->i32{
        self.hp * 3 + self.IBM * 3 + self.Toshiba * 3 + self.Dell * 3
    }
}

fn main(){
   let cost = Brands{
       hp:650_000,
       IBM:755_000,
       Toshiba:550_000,
       Dell:850_000
    };
    //print sum of cost
    println!("hp costs {} \n IBM costs {} \n Toshiba costs {} \n Dell costs {} \n
    Sum of customer purchase {}", cost.hp,cost.IBM,cost.Toshiba,cost.Dell,cost.sum());
}

           
