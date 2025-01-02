use std::io::Write;

fn red()
{
    let row1:[&str;2] = ["S/N", "NAME OF COMMISIONER"];
    let row2:[&str;2] = ["1", "Aigbogun Alamba Daudu"];
    let row3:[&str;2] = ["2", "Murtala Afeez Bendu"];
    let row4:[&str;2] = ["3", "Okorocha Calistus Ogbona"];
    let row5:[&str;2] = ["4", "Adewale Jimoh Akanbi"];
    let row6:[&str;2] = ["5", "Osazuwa Faith Etieye"];

    println!("--------------------------------------------");
    println!("|{} |{}                  |",row1[0],row1[1]);
    println!("--------------------------------------------");
    println!("|{}   |{}                |",row2[0],row2[1]);
    println!("|{}   |{}                  |",row3[0],row3[1]);
    println!("|{}   |{}             |",row4[0],row4[1]);
    println!("|{}   |{}                 |",row5[0],row5[1]);
    println!("|{}   |{}                 |",row6[0],row6[1]);
}

fn green()
{
    let row1:[&str;2] = ["S/N", "MINISTRY"];
    let row2:[&str;2] = ["1", "Internal Affairs"];
    let row3:[&str;2] = ["2", "Justice"];
    let row4:[&str;2] = ["3", "Defence"];
    let row5:[&str;2] = ["4", "Power & Steel"];
    let row6:[&str;2] = ["5", "Petroleum"];

    println!("--------------------------------------------");
    println!("|{} |{}                             |",row1[0],row1[1]);
    println!("--------------------------------------------");
    println!("|{}   |{}                     |",row2[0],row2[1]);
    println!("|{}   |{}                              |",row3[0],row3[1]);
    println!("|{}   |{}                              |",row4[0],row4[1]);
    println!("|{}   |{}                        |",row5[0],row5[1]);
    println!("|{}   |{}                            |",row6[0],row6[1]);
}

fn blue()
{
    let row1:[&str;2] = ["S/N", "GEOPOLITICAL ZONE"];
    let row2:[&str;2] = ["1", "South West"];
    let row3:[&str;2] = ["2", "North East"];
    let row4:[&str;2] = ["3", "South South"];
    let row5:[&str;2] = ["4", "South West"];
    let row6:[&str;2] = ["5", "South East"];

    println!("--------------------------------------------");
    println!("|{} |{}                    |",row1[0],row1[1]);
    println!("--------------------------------------------");
    println!("|{}   |{}                           |",row2[0],row2[1]);
    println!("|{}   |{}                           |",row3[0],row3[1]);
    println!("|{}   |{}                          |",row4[0],row4[1]);
    println!("|{}   |{}                           |",row5[0],row5[1]);
    println!("|{}   |{}                           |",row6[0],row6[1]);
}
fn main() 
{
    //unfortunately I realized that there was no point of the functions in the grand scheme of things

    let row1:[&str;6] = ["S/N", "NAME OF COMMISIONER", "S/N", "MINISTRY", "S/N", "GEOPOLITICAL ZONE"];
    let row2:[&str;6] = ["1", "Aigbogun Alamba Daudu", "1", "Internal Affairs", "1", "South West"];
    let row3:[&str;6] = ["2", "Murtala Afeez Bendu", "2", "Justice", "2", "North East"];
    let row4:[&str;6] = ["3", "Okorocha Calistus Ogbona", "3", "Defence", "3", "South South"];
    let row5:[&str;6] = ["4", "Adewale Jimoh Akanbi", "4", "Power & Steel", "4", "South West"];
    let row6:[&str;6] = ["5", "Osazuwa Faith Etieye", "5", "Petroleum", "5", "South East"];
    
    println!("The Initial Datasets are as follows \n\nThe First Dataset is\n");
    red();
    println!("\nThe Second Dataset is\n");
    green();
    println!("\nThe Thrid and last Dataset is\n");
    blue();


    println!("\nProceeding to combine Datasets");
    println!("\nThis is the Fully Combined Dataset");

    println!("----------------------------------------------------------------------------------------------------------------------------------");
    println!("|{} |{}                  |{} |{}                             |{} |{}                    |",row1[0],row1[1],row1[2],row1[3],row1[4],row1[5]);
    println!("----------------------------------------------------------------------------------------------------------------------------------");
    println!("|{}   |{}                |{}   |{}                     |{}   |{}                           |",row2[0],row2[1],row2[2],row2[3],row2[4],row2[5]);
    println!("|{}   |{}                  |{}   |{}                              |{}   |{}                           |",row3[0],row3[1],row3[2],row3[3],row3[4],row3[5]);
    println!("|{}   |{}             |{}   |{}                              |{}   |{}                          |",row4[0],row4[1],row4[2],row4[3],row4[4],row4[5]);
    println!("|{}   |{}                 |{}   |{}                        |{}   |{}                           |",row5[0],row5[1],row5[2],row5[3],row5[4],row5[5]);
    println!("|{}   |{}                 |{}   |{}                            |{}   |{}                           |",row6[0],row6[1],row6[2],row6[3],row6[4],row6[5]);

    println!("\nProceeding to Write File");

    let mut file = std::fs::File::create(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-9\Project3\Convicted Ministers.txt").expect("create failed");

    let firstrow = format!("|{} |{}                  |{} |{}                             |{} |{}                    |\n",row1[0],row1[1],row1[2],row1[3],row1[4],row1[5]);
    let secondrow = format!("|{}   |{}                |{}   |{}                     |{}   |{}                           |\n",row2[0],row2[1],row2[2],row2[3],row2[4],row2[5]);
    let thirdrow = format!("|{}   |{}                  |{}   |{}                              |{}   |{}                           |\n",row3[0],row3[1],row3[2],row3[3],row3[4],row3[5]);
    let fourthrow = format!("|{}   |{}             |{}   |{}                              |{}   |{}                          |\n",row4[0],row4[1],row4[2],row4[3],row4[4],row4[5]);
    let fifthrow = format!("|{}   |{}                 |{}   |{}                        |{}   |{}                           |\n",row5[0],row5[1],row5[2],row5[3],row5[4],row5[5]);
    let sixthrow = format!("|{}   |{}                 |{}   |{}                            |{}   |{}                           |\n",row6[0],row6[1],row6[2],row6[3],row6[4],row6[5]);

    file.write_all("----------------------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("write failed");
    file.write_all(firstrow.as_bytes()).expect("write failed");
    file.write_all("----------------------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("write failed");
    file.write_all(secondrow.as_bytes()).expect("write failed");
    file.write_all(thirdrow.as_bytes()).expect("write failed");
    file.write_all(fourthrow.as_bytes()).expect("write failed");
    file.write_all(fifthrow.as_bytes()).expect("write failed");
    file.write_all(sixthrow.as_bytes()).expect("write failed");

    println!("File has been written successfully!");
}
