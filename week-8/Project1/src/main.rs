use std::io;

fn officepos()
{

   let pubserv = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
   let office = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
   
   let mut officeexp = String::new();
   println!("How many years of experience do you have?");
   io::stdin().read_line(&mut officeexp).expect("Failed to read input");
   let oexpval:i32 = officeexp.trim().parse().expect("Failed to read input");


  if oexpval >= 1 && oexpval <= 2
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",office[0] , pubserv[0]);
   
   }

   if oexpval >= 3 && oexpval <= 5
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",office[1] , pubserv[1]);
   
   }

    if oexpval >= 5 && oexpval <= 8
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",office[2] , pubserv[2]);
   
   }

    if oexpval >= 8 && oexpval <= 10
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",office[3] , pubserv[3]);
   
   }

    if oexpval >= 10 && oexpval <= 13
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",office[4] , pubserv[4]);
   
   }

    if oexpval > 13
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",office[5] , pubserv[5]);
   
   }
}

fn academpos()
{
    let pubserv = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let academic = vec!["N/A", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
  
   let mut academexp = String::new();
   println!("How many years of experience do you have?");
   io::stdin().read_line(&mut academexp).expect("Failed to read input");
   let aexpval:i32 = academexp.trim().parse().expect("Failed to read input");


  if aexpval >= 1 && aexpval <= 2
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",academic[0] , pubserv[0]);
   
   }

   if aexpval >= 3 && aexpval <= 5
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",academic[1] , pubserv[1]);
   
   }

    if aexpval >= 5 && aexpval <= 8
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",academic[2] , pubserv[2]);
   
   }

    if aexpval >= 8 && aexpval <= 10
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",academic[3] , pubserv[3]);
   
   }

    if aexpval >= 10 && aexpval <= 13
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",academic[4] , pubserv[4]);
   
   }

    if aexpval > 13
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",academic[5] , pubserv[5]);
   
   }

}

fn lawyerpos()
{
  let pubserv = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
  let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    
   let mut lawexp = String::new();
   println!("How many years of experience do you have?");
   io::stdin().read_line(&mut lawexp).expect("Failed to read input");
   let lexpval:i32 = lawexp.trim().parse().expect("Failed to read input");


  if lexpval >= 1 && lexpval <= 2
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",lawyer[0] , pubserv[0]);
   
   }

   if lexpval >= 3 && lexpval <= 5
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",lawyer[1] , pubserv[1]);
   
   }

    if lexpval >= 5 && lexpval <= 8
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",lawyer[2] , pubserv[2]);
   
   }

    if lexpval >= 8 && lexpval <= 10
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",lawyer[3] , pubserv[3]);
   
   }

    if lexpval >= 10 && lexpval <= 13
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",lawyer[4] , pubserv[4]);
   
   }

    if lexpval > 13
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",lawyer[5] , pubserv[5]);
   
   }    
}

fn teacherpos()
{
 let pubserv = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
 let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
  
   let mut teachexp = String::new();
   println!("How many years of experience do you have?");
   io::stdin().read_line(&mut teachexp).expect("Failed to read input");
   let texpval:i32 = teachexp.trim().parse().expect("Failed to read input");


  if texpval >= 1 && texpval <= 2
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",teacher[0] , pubserv[0]);
   
   }

   if texpval >= 3 && texpval <= 5
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",teacher[1] , pubserv[1]);
   
   }

    if texpval >= 5 && texpval <= 8
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",teacher[2] , pubserv[2]);
   
   }

    if texpval >= 8 && texpval <= 10
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",teacher[3] , pubserv[3]);
   
   }

    if texpval >= 10 && texpval <= 13
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",teacher[4] , pubserv[4]);
   
   }

    if texpval > 13
   {
      println!("Your Field of Work is {} and Your Staff Position is {}",teacher[5] , pubserv[5]);
   
   }
}

fn main()
{
   println!("You have been invited to join the team of developers to build a Public Service APS level \nchecker for the Federal Government of Nigeria. You have been provided the following table \nbelow. \nDevelop a Rust program using vectors to validate Staff level. E.g. If Staff is an Associate \nLawyer, and has 5-8 years of work experience, then the staff holds position APS 5-8.");
   println!("Hello Welcome to the Public Service APS Level checker from your Federal Government, \nAnswer these questions and we will determine your staff position");

   let mut input1 = String::new();
   println!("What Public Service sector are you in?");
   println!("Office Administrator, Academic, Lawyer, Teacher \nPress 1 for Office Administrator, 2 for Academic, 3 for Lawyer and so on");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let sector:i32 = input1.trim().parse().expect("Failed to read input");

   
    if sector == 1 
    {
       officepos();
    }

    if sector == 2
    {
       academpos();
    }

     if sector == 3
    {
       lawyerpos();
    }
   
      if sector == 4
    {
       teacherpos();
    }
}
