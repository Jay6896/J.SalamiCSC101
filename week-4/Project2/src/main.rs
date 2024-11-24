use std::io;

fn main() 
{
    let mut exp = String::new();
    let mut age = String::new();

    println!("Develop a program in Rust that takes as input the experience \nand age of an employee to determine the annual incentive, \nwith the following criteria:");
    println!("What is Your experience? \nAre you Experienced? \nType y or n");
    io::stdin().read_line(&mut exp).expect("Failed to read input");
    let exp = exp.trim();

    if exp == "y"
    {
      println!("What is Your Age?");
      io::stdin().read_line(&mut age).expect("Please enter a Correct age");
      let age:i32 = age.trim().parse().expect("Your input is not an integer, Please enter a Correct age");

      if age >= 40
      {
        println!("Your Incentive is N1,560,000");
      }


       if age >= 30 && age < 40
      {
        println!("Your Incentive is N1,480,000");
      }

        if age < 28
      {
        println!("Your Incentive is N1,300,000");
      }

       else 
       {
        println!("You dont have any Incentive");
       }
    }

     else if exp == "n"
     {
        println!("Your Incentive is N100,000");
     }
     else 
     {
        println!("Failed to read input, make sure your answer is EXACTLY y or n");
     }
}
