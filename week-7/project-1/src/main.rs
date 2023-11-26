use std::io;

   fn Trapezium_area_calculator(){

      let mut input1 = String::new();
      let mut input2 = String::new();
      let mut input3 = String::new();

      println!("the height of the trapezium is");
      io::stdin().read_line(&mut input1).expect("invalid");
      let h:i32 = input1.trim().parse().expect("invalid");

      println!("the base1 of the trapezium is");
      io::stdin().read_line(&mut input2).expect("invalid");
      let B1:i32 = input2.trim().parse().expect("invalid");

      println!("the base2 of the trapezium is");
      io::stdin().read_line(&mut input3).expect("invalid");
      let B2:i32 = input3.trim().parse().expect("invalid");

      println!("the area of a trapezium is");
      let Area_of_Trapezium = h / 2 * (B1 + B2);
    }

    fn Rhombus_area_calculator(){

        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("the diagonal1 of a rhombus is");
        io::stdin().read_line(&mut input1).expect("invalid");
        let D1:i32 = input1.trim().parse().expect("invalid");

        println!("the diagonal2 of a rhombus is");
        io::stdin().read_line(&mut input2).expect("invalid");
        let D2:i32 = input2.trim().parse().expect("invalid");
        
        println!("the area of a rhobus is");
        let Area_of_rhombus = 1 / 2 * (D1 * D2);
    }

    fn Parallelogram_area_calculator(){

       let mut input1 = String::new();
       let mut input2 = String::new();

       println!("the base of a parallelogram is");
       io::stdin().read_line(&mut input1).expect("invalid");
       let b:i32 = input1.trim().parse().expect("invalid");

       println!("the altitude of a parallelogram");
       io::stdin().read_line(&mut input2).expect("invalid");
       let a:i32 = input2.trim().parse().expect("invalid");

       println!("the area of a parallelogram is");
       let Area_of_parallelogram = b * a;
    }     

    fn Cube_area_calculator(){

       let mut input1 = String::new();
    
    
       println!("the length of the side of a cube is ");
       io::stdin().read_line(&mut input1).expect("invalid");
       let l:i32 = input1.trim().parse().expect("invalid");

       println!("the area of a cube is");
       let Area_of_cube = 6 * (l) ^ 2;
    }
    fn Volume_area_calculator(){

       let mut input1 = String::new();
       let mut input2 = String::new();
       
       println!("the radius of a cylinder is");
       io::stdin().read_line(&mut input1).expect("invalid");
       let r:i32 = input1.trim().parse().expect("invalid");

       println!("the height of a cylinder is");
       io::stdin().read_line(&mut input1).expect("invalid");
       let h:i32 = input2.trim().parse().expect("invalid");

       println!("the area of a cylinder is");
       let Area_of_cylinder = (22/7) * (r^2 * h);
    }
    fn main(){
       println!("Welcome to the shape calculator");
       println!("please select from the following shapes");
       println!("1. Area_of_rhombus");
       println!("2. Area_of_cube");
       println!("3. Area_of_cylinder");
       println!("4. Area_of_Trapezium");
       println!("5. Area_of_parallelogram");
       
       let mut input1 = String::new();
       println!("Enter the shape of choice");
       io::stdin().read_line(&mut input1).expect("invalid");
       let shape:i32 = input1.trim().parse().expect("invalid");
       
       if shape == 1{
          Rhombus_area_calculator();
        }
        else if shape == 2{
           Cube_area_calculator();
        }
        else if shape == 3{
           Volume_area_calculator();
        }
        else if shape == 4{
           Trapezium_area_calculator();
        }
        else if shape == 5{
           Parallelogram_area_calculator();
        }          
}    
                        
