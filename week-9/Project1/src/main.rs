use std::io::Write;

fn main() 
{
    let row1 = "|Lager           |Stout                |Non-Alcoholic      |\n";
    let row2 = "|33 Export       |Legend               |Maltina            |\n";
    let row3 = "|Desperados      |Turbo King           |Amstel Malta       |\n";
    let row4 = "|Goldberg        |Williams             |Malta Gold         |\n";
    let row5 = "|Gulder          |                     |Fayrouz            |\n";
    let row6 = "|Heineken        |                     |                   |\n";
    let row7 = "|Star            |                     |                   |\n";
    let mut file = std::fs::File::create(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-9\Project1\Nigerian Brewery Limited Portfolio.txt").expect("create failed");
    file.write_all("Welcome to the Nigerian Brewery Limited Portfolio! \nHere are the high quality drinks that we serve\n\n".as_bytes()).expect("write failed");
    file.write_all(row1.as_bytes()).expect("write failed");
    file.write_all(row2.as_bytes()).expect("write failed");
    file.write_all(row3.as_bytes()).expect("write failed");
    file.write_all(row4.as_bytes()).expect("write failed");
    file.write_all(row5.as_bytes()).expect("write failed");
    file.write_all(row6.as_bytes()).expect("write failed");
    file.write_all(row7.as_bytes()).expect("write failed");
    println!("Data Written Successfully!");
}
