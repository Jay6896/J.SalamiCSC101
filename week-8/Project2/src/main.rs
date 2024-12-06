use std::io;

fn main() 
{
    let mut totalexp : Vec<String> = Vec::new();
    let mut totalname : Vec<String> = Vec::new();
    
    println!("Ernst & Young (EY) Global Limited, is a \nmultinational professional services network with \nheadquarters in London and branch offices all \naround the world. EY is one of the largest \nprofessional services networks in the world. \nEY Nigeria recently launched a project and are \nscouting for developers with the highest years of \nexperience. You have been contacted by the CEO \nof EY Nigeria to develop a Rust program to find \nthe person with the highest years of programming \nexperience during the job interview using any rust \ncompound data type.");
    println!("\n\nWelcome to the Ernst & Young Interview Generator Portal! \nHow many interviews would you like to run?");
    
    let mut input1 = String::new(); 
    io::stdin().read_line(&mut input1).expect("Failed to read Input");
    let intervno:i32 = input1.trim().parse().expect("Invalid Input");

    for i in 0..intervno
    {
      println!("Welcome to Ernst & Young Global Limited \nWe are Currently Recruiting programmers for our company \nPlease fill in the information below");
      println!("What is your name?");
      let mut nameinput = String::new(); 
      io::stdin().read_line(&mut nameinput).expect("Failed to read Input");
      let name:String = nameinput.trim().parse().expect("Invalid input");

      let mut expinput = String::new();
      println!("How many years of experience do you have?");
      io::stdin().read_line(&mut expinput).expect("Failed to read input");
      let expval:String = expinput.trim().parse().expect("Failed to read input");
      totalname.push(name);
      totalexp.push(expval);     

    }

      println!("Your Applicants in order are:");
      let mut order = 1;
      for (totalname, totalexp) in totalname.iter().zip(totalexp.iter())
      {
        println!("{}.) Name: {}, Programming Experience: {}",order,totalname,totalexp );
        order += 1;
      }
     if let Some((index, max_exp)) = totalexp.iter().enumerate().max_by_key(|&(_, exp)| exp) 
     {
        println!(
            "The Applicant with the highest experience is {} with {} years of experience.",
            totalname[index], max_exp
        );
     } 

      else 
      {
        println!("No applicants were entered.");
      }
}