use std::io;
use std::io::Write;
 fn main() 
{
  let row1:[&str;4] = ["Student Name ", "Matric. Number ", "Department ", "Level "];
  let row2:[&str;4] = ["Oluchi Mordi ", "ACC1021111     ", "Accounting ", "300   "];
  let row3:[&str;4] = ["Adams Aliyu  ", "ECO10110101    ", "Economics  ", "100   "];
  let row4:[&str;4] = ["Shania Bolade", "CSC10328828    ", "Computer   ", "200   "];
  let row5:[&str;4] = ["Adekunle Gold", "EEE11020202    ", "Electrical ", "200   "];
  let row6:[&str;4] = ["Blanca Edemoh", "MEE10202001    ", "Mechanical ", "100   "];
  println!("Student Management Information System records \n--------------------------------------------------\n|                  PAU SMIS                      |\n--------------------------------------------------");
  println!("|{}|{}|{}|{}|",row1[0],row1[1],row1[2],row1[3]);
  println!("|{}|{}|{}|{}|",row2[0],row2[1],row2[2],row2[3]);
  println!("|{}|{}|{}|{}|",row3[0],row3[1],row3[2],row3[3]);
  println!("|{}|{}|{}|{}|",row4[0],row4[1],row4[2],row4[3]);
  println!("|{}|{}|{}|{}|",row5[0],row5[1],row5[2],row5[3]);
  println!("|{}|{}|{}|{}|",row6[0],row6[1],row6[2],row6[3]);

  let mut file = std::fs::File::create(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-9\Project2\PAU SMIS Default.txt").expect("create failed");
  
  let firstrow = format!("|{}|{}|{}|{}|\n",row1[0],row1[1],row1[2],row1[3]);
  let secondrow = format!("|{}|{}|{}|{}|\n",row2[0],row2[1],row2[2],row2[3]);
  let thirdrow = format!("|{}|{}|{}|{}|\n",row3[0],row3[1],row3[2],row3[3]);
  let fourthrow = format!("|{}|{}|{}|{}|\n",row4[0],row4[1],row4[2],row4[3]);
  let fifthrow = format!("|{}|{}|{}|{}|\n",row5[0],row5[1],row5[2],row5[3]);
  let sixthrow = format!("|{}|{}|{}|{}|\n",row6[0],row6[1],row6[2],row6[3]);
  
  file.write_all("Student Management Information System records \n--------------------------------------------------\n|                  PAU SMIS                      |\n--------------------------------------------------\n".as_bytes()).expect("write failed");
  file.write_all(firstrow.as_bytes()).expect("write failed");
  file.write_all(secondrow.as_bytes()).expect("write failed");
  file.write_all(thirdrow.as_bytes()).expect("write failed");
  file.write_all(fourthrow.as_bytes()).expect("write failed");
  file.write_all(fifthrow.as_bytes()).expect("write failed");
  file.write_all(sixthrow.as_bytes()).expect("write failed");


  let mut input1 = String::new();
  let mut input2 = String::new();

  
  println!("The default file has been successfully creaded!\n\nWould you like to make your own custom table?\nType y or n");
  io::stdin().read_line(&mut input1).expect("Failed to read input");
  let response = input1.trim();

  if response == "y" || response == "yes"
  {
    println!("How many Students are you inputting?");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let loopval:i32 = input2.trim().parse().expect("Invalid input");
    
    let mut name : Vec<String> = Vec::new();
    let mut matr : Vec<String> = Vec::new();
    let mut depa : Vec<String> = Vec::new();
    let mut year: Vec<i32> = Vec::new();

    for i in 0..loopval 
    {
       let mut inputn = String::new();
       let mut inputm = String::new();
       let mut inputd = String::new();
       let mut inputy = String::new();



       println!("What is the name of the student?");
       io::stdin().read_line(&mut inputn).expect("Failed to read input");
       let studentname:String = inputn.trim().parse().expect("Invalid input");

       println!("What is the students matriculation number?");
       io::stdin().read_line(&mut inputm).expect("Failed to read input");
       let matriculation:String = inputm.trim().parse().expect("Invalid input");

       println!("What is the students department?");
       io::stdin().read_line(&mut inputd).expect("Failed to read input");
       let department:String = inputd.trim().parse().expect("Invalid input");

       println!("What level is the student currently in? (Digits Only)");
       io::stdin().read_line(&mut inputy).expect("Failed to read input");
       let level:i32 = inputy.trim().parse().expect("Invalid input, dont add the word level, only the digits");
      
       name.push(studentname);
       matr.push(matriculation);
       depa.push(department);
       year.push(level);
    }

       println!("This is your output");
       println!("Student Management Information System records \n-------------------------------------------------------------\n|                          PAU SMIS                         |\n-------------------------------------------------------------");
       println!("|{:<15}|{:<15}|{:<20}|{:<5}|",row1[0],row1[1],row1[2],row1[3]);
       for i in 0..name.len() 
       {
        println!("|{:<15}|{:<15}|{:<20}|{:<6}|",name[i], matr[i], depa[i], year[i]);
       }

       
   let mut file = std::fs::File::create(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-9\Project2\PAU SMIS Custom.txt").expect("create failed");
  
  
   file.write_all("Student Management Information System records \n-------------------------------------------------------------\n|                          PAU SMIS                         |\n-------------------------------------------------------------".as_bytes()).expect("write failed");
   let header = format!("\n|{:<15}|{:<15}|{:<20}|{:<5}|\n",row1[0],row1[1],row1[2],row1[3]);
   file.write_all(header.as_bytes()).expect("write failed");

   for i in 0..name.len() 
   {
    let row = format!("|{:<15}|{:<15}|{:<20}|{:<6}|\n",name[i], matr[i], depa[i], year[i]);
    file.write_all(row.as_bytes()).expect("write failed");
   }
   println!("Your Custom File has been created successfully!");
 }
  else
  {
    println!("Thank you for using PAU SMIS");
  }
}
