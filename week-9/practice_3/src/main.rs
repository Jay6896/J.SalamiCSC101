use std::fs;

fn main() 
{
    fs::remove_file(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-9\practice_3\FILE_THAT_WOULD_BE_DELETED.txt");
    println!("file is removed");
}
