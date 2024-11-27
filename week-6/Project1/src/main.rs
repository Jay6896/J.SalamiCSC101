use std::io;

fn Trapezium()
{
    let mut hT = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();

    println!("What do you want the height of the Trapezium to be?");
    io::stdin().read_line(&mut hT).expect("Failed to read input");
    let heightvalue:f32 = hT.trim().parse().expect("Invalid input, ensure you typed numbers only");

    println!("What do you want the first base of the Trapezium to be?");
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1value:f32 = base1.trim().parse().expect("Invalid input, ensure you typed numbers only");

    println!("What do you want the second base of the Trapezium to be?");
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let base2value:f32 = base2.trim().parse().expect("Invalid input, ensure you typed numbers only");

    let formulaT = (heightvalue / 2.0) * (base1value + base2value);
    println!("The Area of this Trapezium is {}", formulaT);
}

fn Rhombus()
{
   let mut diagonal1 = String::new();
   let mut diagonal2 = String::new();

   println!("What do you want the first diagonal of the Rhombus to be?");
   io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
   let diagonal1value:f32 = diagonal1.trim().parse().expect("Invalid input, ensure you typed numbers only");


   println!("What do you want the second diagonal of the Rhombus to be?");
   io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
   let diagonal2value:f32 = diagonal2.trim().parse().expect("Invalid input, ensure you typed numbers only");

   let formulaR = 0.5 * diagonal1value * diagonal2value;
   println!("The Area of this Rhombus is {}", formulaR);
}

fn Parallelogram()
{
  let mut baseP = String::new();
  let mut altitude = String::new();

   println!("What do you want the base of the Parallelogram to be?");
   io::stdin().read_line(&mut baseP).expect("Failed to read input");
   let basePvalue:f32 = baseP.trim().parse().expect("Invalid input, ensure you typed numbers only");


   println!("What do you want the altitude of the Parallelogram to be?");
   io::stdin().read_line(&mut altitude).expect("Failed to read input");
   let altitudevalue:f32 = altitude.trim().parse().expect("Invalid input, ensure you typed numbers only");

   let formulaP = basePvalue * altitudevalue;
   println!("The Area of this Rhombus is {}", formulaP);
}

fn Cube()
{
  let mut length = String::new();

  println!("What do you want the length of the Cube to be?");
  io::stdin().read_line(&mut length).expect("Failed to read input");
  let lengthvalue:f32 = length.trim().parse().expect("Invalid input, ensure you typed numbers only");

  let formulaCb = 6.0 * (lengthvalue * lengthvalue);
  println!("The Area of this Cube is {}", formulaCb);
}

fn Cylinder()
{
   let mut radius = String::new();
   let mut hC = String::new();

  println!("What do you want the radius of the Cylinder to be?");
  io::stdin().read_line(&mut radius).expect("Failed to read input");
  let radiusvalue:f32 = radius.trim().parse().expect("Invalid input, ensure you typed numbers only");

  println!("What do you want the height of the Cylinder to be?");
  io::stdin().read_line(&mut hC).expect("Failed to read input");
  let hCvalue:f32 = hC.trim().parse().expect("Invalid input, ensure you typed numbers only");

  let formulaCy = 3.14 * (radiusvalue * radiusvalue) * hCvalue; 
  println!("The Volume of this Cylinder is {}", formulaCy);
}



fn main() 
{
    let mut input = String::new();
    println!("Your MTH 101 Professor has asked you to develop a Rust program \nthat performs the following calculations: \nArea of Trapezium formula = height/2*(base1+base2) \nArea of the rhombus formula = ½ × diagonal1 × diagonal2 \nArea of Parallelogram formula = base x altitude \nArea of Cube formula = 6 x (length of the side)2 \nVolume of Cylinder formula = π*radius2 *height \n\nUsing your knowledge of Rust Functions, develop the program that \nprompts a user to select an equation, reads inputs and then performs \nthe corresponding calculations.");
    println!("\nSo which of these equations would you like me to solve? \nInput 1 for trapezium, 2 for rhombus, and so on");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let response:i32 = input.trim().parse().expect("Invalid input, ensure you typed only 1 digit");

    if response == 1
    {
      println!("I will now proceed to solve for the area of a Trapezium for you, give me your input");
      Trapezium();
    }


    if response == 2
    {
      println!("I will now proceed to solve for the area of a Rhombus for you, give me your input");
      Rhombus();
    }

    if response == 3
    {
      println!("I will now proceed to solve for the area of a Parallelogram for you, give me your input");
      Parallelogram();
    }

    if response == 4
    {
      println!("I will now proceed to solve for the area of a cube for you, give me your input");
      Cube();
    }
    
    if response == 5
    {
      println!("I will now proceed to solve for the volume of a Cylinder for you, give me your input");
      Cylinder();
    }

    if response < 1 && response > 5
    {
      println!("The value you entered is not in the range of available formulas to solve");
    }
}
