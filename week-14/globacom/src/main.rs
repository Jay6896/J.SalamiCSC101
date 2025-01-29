use std::io::Read;

fn main() 
{
    let mut file = std::fs::File::open(r"C:\Users\Jamil\Documents\J.SalamiCSC101\week-14\globacom\target\staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
