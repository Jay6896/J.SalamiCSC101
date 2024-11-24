 fn main()
{
  let n1 = "Electrical".to_string(); 
  let n2 ="Electronic".to_string(); 
  let n3 = "Engineering".to_string();
  let n4 = n1 + &n2 + &n3;  // n2 & n3 reference is passed

 // About Electrical/Electronic
  println!("\nThe {} is informed by the aspiration to \ntrain electrical/electronic engineering professionals \nin the areas of design, building and maintenance of \nelectrical control systems, ", n4);
  let w1 =  "Computer".to_string();
  let w2 =  "Science".to_string();
  let w3 = w1+ &w2; // w2 reference is passed
  println!();
  println!("{} is aimed at developing competent, creative, \ninnovative, entrepreneurial and ethically-minded persons, \ncapable of creating value in the diverse fields of \nComputer Science. ",w3);
}