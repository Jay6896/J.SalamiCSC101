use std::io;

fn main() 
{
 let f1 = "Poundo Yam/Edinkaiko Soup".to_string();
 let f2 = "Fried Rice & Chicken".to_string();
 let f3 = "Amala & Ewedu Soup".to_string();
 let f4 = "Eba & Egusi Soup".to_string();
 let f5 = "White Rice & Stew".to_string();
 let pval:f32 = 3200.0;
 let fval:f32 = 3000.0;
 let aval:f32 = 2500.0;
 let eval:f32 = 2000.0;
 let wval:f32 = 2500.0;
 let mut input1 = String::new();
 let mut input1q = String::new();
 let mut input2 = String::new();
 let mut input2q = String::new();
 let mut input3 = String::new();
 let mut input3q = String::new();
 let mut input4 = String::new();
 let mut input4q = String::new();
 let mut input5 = String::new();
 let mut input5q = String::new();
 let mut pquantity:f32 = 0.0;
 let mut fquantity:f32 = 0.0;
 let mut aquantity:f32 = 0.0;
 let mut equantity:f32 = 0.0;
 let mut wquantity:f32 = 0.0;

 println!("Develop a Rust program that displays \nthe following menu for the food items available to take orders from the customer. \n\nThe program inputs the type of food and quantity. It finally \ndisplays the total charges for the order according to price \ncriteria. If the total order is greater than N10,000, give a \ndiscount of 5%");
 let menu = format!("{}   - N3,200\n{}        - N3,000\n{}          - N2,500\n{}            - N2,000\n{}           - N2,500", f1, f2, f3, f4, f5);
 println!("\nThe food on the menu consists of: \n\n{}", menu);

 println!("What would you like to order? \nWould you like to order Poundo Yam/Edinkaiko Soup? \nIf so Type P in uppercase, if not type anything else");
 io::stdin().read_line(&mut input1).expect("Failed to read input");
 input1 = input1.trim().to_string();

 if input1 == "P" 
 {
   println!("What quantity of Poundo Yam/Edinkaiko Soup would you like?");
   io::stdin().read_line(&mut input1q).expect("Failed to read input");
   pquantity = input1q.trim().parse().expect("Failed to read input, make sure you typed an integer value");
 }
 
   println!("Would you like to order Fried Rice & Chicken? \nIf so Type F in uppercase, if not type anything else");
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   input2 = input2.trim().to_string();

   if input2 == "F" 
   {
    println!("What quantity of Fried Rice & Chicken would you like?");
    io::stdin().read_line(&mut input2q).expect("Failed to read input");
    fquantity = input2q.trim().parse().expect("Failed to read input, make sure you typed an integer value");
   }


   println!("Would you like to order Amala & Ewedu Soup? \nIf so Type A in uppercase, if not type anything else");
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   input3 = input3.trim().to_string();

   if input3 == "A"
   {
    println!("What quantity of Fried Rice & Chicken would you like?");
    io::stdin().read_line(&mut input3q).expect("Failed to read input");
    aquantity = input2q.trim().parse().expect("Failed to read input, make sure you typed an integer value");
   }

   println!("Would you like to order Eba & Egusi Soup? \nIf so Type E in uppercase, if not type anything else");
   io::stdin().read_line(&mut input4).expect("Failed to read input");
   input4 = input4.trim().to_string();

   if input4 == "E"
   {
    println!("What quantity of Eba & Egusi Soup would you like?");
    io::stdin().read_line(&mut input4q).expect("Failed to read input");
    equantity = input4q.trim().parse().expect("Failed to read input, make sure you typed an integer value");
   }

   println!("Would you like to order White Rice & Stew? \nIf so Type W in uppercase, if not type anything else");
   io::stdin().read_line(&mut input5).expect("Failed to read input");
   input5 = input5.trim().to_string();

    if input5 == "W"
    {
     println!("What quantity of White Rice & Stew would you like?");
     io::stdin().read_line(&mut input5q).expect("Failed to read input");
     wquantity = input5q.trim().parse().expect("Failed to read input, make sure you typed an integer value");
    }

    let total:f32 = (pval * pquantity) + (fval * fquantity) + (aval * aquantity) + (eval * equantity) + (wval * wquantity);
    if total >= 10000.0
    {
     let discounttotal:f32 = total - ((5.0/ 100.0) * total);
     println!("Since your total is above N10,000, You are receiving a 10% discount and your discounted total is: {}", discounttotal);
    }

    else
    {
     println!("Your total is: N{}", total);
    }
}