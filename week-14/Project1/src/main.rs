use std::io;
use std::io::Read;

fn admin()
{
   println!("Welcome Administrator! \nYour Database item is the Database: globacom_dbase");
   let mut file = std::fs::File::open(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-14\Project1\target\globacom_dbase.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}

fn projmanager()
{
   println!("Welcome Project Manager! \nYour Database item is the Database table: project_tb");
   let mut file = std::fs::File::open(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-14\Project1\target\project_tb.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}

fn employee()
{
   println!("Welcome Employee! \nYour Database item is the Database table: staff_tb");
   let mut file = std::fs::File::open(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-14\Project1\target\staff_tb.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}

fn customer()
{
   println!("Welcome Customer! \nYour Database item is the Database table: customer_tb");
   let mut file = std::fs::File::open(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-14\Project1\target\customer_tb.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}

fn vendor()
{
   println!("Welcome Vendor! \nYour Database item is the Database table: dataplan_tb");
   let mut file = std::fs::File::open(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-14\Project1\target\dataplan_tb.sql").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
fn main() 
{
    let mut input1 = String::new();
    println!("Welcome to the Globacom Ltd Database \nPlease identify yourself with the Options Below to Access Your Designated Database Item \n\n(INPUT THE NUMBER BESIDE YOUR IDENTITY AS INDICATION)\n1 - Administrator \n2 - Project Manager \n3 - Employee \n4 - Customer \n5 - Vendor \n");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let userval:i32 = input1.trim().parse().expect("Invalid input");

    if userval == 1 
    {
      admin();
    }

    else if userval == 2 
    {
       projmanager();
    }

    else if userval == 3
    {
       employee();
    }

    else if userval == 4
    {
       customer();
    }

    else if userval == 5
    {
       vendor();
    }

    else 
    {
       println!("Your Value is out of scope, please try again");
    }
}
